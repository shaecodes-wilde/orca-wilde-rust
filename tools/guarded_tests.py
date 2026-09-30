"""Run Cargo test executables in an owned process boundary; never kill by name or saved PID."""
from __future__ import annotations
import argparse
import ctypes
import json
import os
from pathlib import Path
import signal
import subprocess
import time


def windows_run(argv: list[str], log: Path, timeout: float) -> dict:
    from ctypes import wintypes as w
    import msvcrt
    kernel = ctypes.WinDLL("kernel32", use_last_error=True)
    size = ctypes.c_size_t
    class BasicLimits(ctypes.Structure):
        _fields_ = [("process_time", ctypes.c_int64), ("job_time", ctypes.c_int64), ("flags", w.DWORD), ("minimum", size), ("maximum", size), ("active_limit", w.DWORD), ("affinity", size), ("priority", w.DWORD), ("scheduling", w.DWORD)]
    class IoCounters(ctypes.Structure):
        _fields_ = [(name, ctypes.c_uint64) for name in ("reads", "writes", "others", "read_bytes", "write_bytes", "other_bytes")]
    class ExtendedLimits(ctypes.Structure):
        _fields_ = [("basic", BasicLimits), ("io", IoCounters), ("process_memory", size), ("job_memory", size), ("peak_process", size), ("peak_job", size)]
    class Startup(ctypes.Structure):
        _fields_ = [("cb", w.DWORD), ("reserved", w.LPWSTR), ("desktop", w.LPWSTR), ("title", w.LPWSTR), ("x", w.DWORD), ("y", w.DWORD), ("width", w.DWORD), ("height", w.DWORD), ("columns", w.DWORD), ("rows", w.DWORD), ("fill", w.DWORD), ("flags", w.DWORD), ("show", w.WORD), ("reserved_count", w.WORD), ("reserved_bytes", ctypes.POINTER(ctypes.c_byte)), ("stdin", w.HANDLE), ("stdout", w.HANDLE), ("stderr", w.HANDLE)]
    class ProcessInfo(ctypes.Structure):
        _fields_ = [("process", w.HANDLE), ("thread", w.HANDLE), ("pid", w.DWORD), ("tid", w.DWORD)]
    class Accounting(ctypes.Structure):
        _fields_ = [(name, ctypes.c_int64) for name in ("user", "kernel", "period_user", "period_kernel")] + [(name, w.DWORD) for name in ("faults", "total", "active", "terminated")]
    definitions = {
        "CreateJobObjectW": ([ctypes.c_void_p, w.LPCWSTR], w.HANDLE),
        "SetInformationJobObject": ([w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD], w.BOOL),
        "AssignProcessToJobObject": ([w.HANDLE, w.HANDLE], w.BOOL),
        "CreateProcessW": ([w.LPCWSTR, w.LPWSTR, ctypes.c_void_p, ctypes.c_void_p, w.BOOL, w.DWORD, ctypes.c_void_p, w.LPCWSTR, ctypes.POINTER(Startup), ctypes.POINTER(ProcessInfo)], w.BOOL),
        "ResumeThread": ([w.HANDLE], w.DWORD),
        "WaitForSingleObject": ([w.HANDLE, w.DWORD], w.DWORD),
        "GetExitCodeProcess": ([w.HANDLE, ctypes.POINTER(w.DWORD)], w.BOOL),
        "TerminateJobObject": ([w.HANDLE, w.UINT], w.BOOL),
        "TerminateProcess": ([w.HANDLE, w.UINT], w.BOOL),
        "QueryInformationJobObject": ([w.HANDLE, ctypes.c_int, ctypes.c_void_p, w.DWORD, ctypes.c_void_p], w.BOOL),
        "CloseHandle": ([w.HANDLE], w.BOOL),
    }
    for name, (arguments, result) in definitions.items():
        function = getattr(kernel, name)
        function.argtypes, function.restype = arguments, result
    def checked(ok: object) -> None:
        if not ok:
            raise ctypes.WinError(ctypes.get_last_error())
    job = kernel.CreateJobObjectW(None, None)
    checked(job)
    process = ProcessInfo()
    assigned = False
    started = time.monotonic()
    try:
        limits = ExtendedLimits()
        limits.basic.flags = 0x2000  # Kill-on-close, with no breakaway allowance.
        checked(kernel.SetInformationJobObject(job, 9, ctypes.byref(limits), ctypes.sizeof(limits)))
        with log.open("wb") as output, open(os.devnull, "rb") as null:
            out_handle, in_handle = msvcrt.get_osfhandle(output.fileno()), msvcrt.get_osfhandle(null.fileno())
            os.set_handle_inheritable(out_handle, True)
            os.set_handle_inheritable(in_handle, True)
            startup = Startup()
            startup.cb, startup.flags = ctypes.sizeof(startup), 0x100
            startup.stdin, startup.stdout, startup.stderr = in_handle, out_handle, out_handle
            command = ctypes.create_unicode_buffer(subprocess.list2cmdline(argv))
            checked(kernel.CreateProcessW(argv[0], command, None, None, True, 0x08000000 | 0x4, None, str(Path(argv[0]).parent), ctypes.byref(startup), ctypes.byref(process)))
            try:
                checked(kernel.AssignProcessToJobObject(job, process.process))
                assigned = True
                if kernel.ResumeThread(process.thread) == 0xFFFFFFFF:
                    raise ctypes.WinError(ctypes.get_last_error())
                wait = kernel.WaitForSingleObject(process.process, int(timeout * 1000))
                timed_out = wait == 258
                if wait not in (0, 258):
                    raise RuntimeError(f"Unexpected owned wait result {wait}")
                if timed_out:
                    checked(kernel.TerminateJobObject(job, 124))
                    if kernel.WaitForSingleObject(process.process, 10000) != 0:
                        raise RuntimeError("Owned process termination remains unconfirmed")
                exit_code = w.DWORD()
                checked(kernel.GetExitCodeProcess(process.process, ctypes.byref(exit_code)))
                account = Accounting()
                checked(kernel.QueryInformationJobObject(job, 1, ctypes.byref(account), ctypes.sizeof(account), None))
                initial_active = account.active
                grace_started = time.monotonic()
                while account.active and time.monotonic() - grace_started < 1:
                    time.sleep(0.02)
                    checked(kernel.QueryInformationJobObject(job, 1, ctypes.byref(account), ctypes.sizeof(account), None))
                grace_seconds = time.monotonic() - grace_started
                active_after_grace = account.active
                had_remaining = active_after_grace != 0
                if had_remaining:
                    checked(kernel.TerminateJobObject(job, 125))
                stop_deadline = time.monotonic() + 10
                while account.active and time.monotonic() < stop_deadline:
                    time.sleep(0.02)
                    checked(kernel.QueryInformationJobObject(job, 1, ctypes.byref(account), ctypes.sizeof(account), None))
                return {"exit_code": exit_code.value, "timeout": timed_out, "forced_remaining_children": had_remaining, "initial_active": initial_active, "active_after_grace": active_after_grace, "grace_seconds": grace_seconds, "owned_job_empty": account.active == 0, "pid_informational_only": process.pid, "seconds": time.monotonic() - started}
            except Exception as failure:
                cleanup_errors = []
                cleanup_confirmed = False
                try:
                    if assigned:
                        checked(kernel.TerminateJobObject(job, 126))
                    else:
                        checked(kernel.TerminateProcess(process.process, 126))
                    if kernel.WaitForSingleObject(process.process, 10000) != 0:
                        raise RuntimeError("Owned failed-start child termination remains unconfirmed")
                    if assigned:
                        account = Accounting()
                        deadline = time.monotonic() + 10
                        while True:
                            checked(kernel.QueryInformationJobObject(job, 1, ctypes.byref(account), ctypes.sizeof(account), None))
                            if account.active == 0:
                                break
                            if time.monotonic() >= deadline:
                                raise RuntimeError("Owned failed-start job cleanup remains unconfirmed")
                            time.sleep(0.02)
                    cleanup_confirmed = True
                except Exception as cleanup_failure:
                    cleanup_errors.append(str(cleanup_failure))
                return {"exit_code": 1, "timeout": False, "owned_job_empty": cleanup_confirmed, "error": str(failure), "cleanup_errors": cleanup_errors, "pid_informational_only": process.pid, "seconds": time.monotonic() - started}
    finally:
        # Closing this private job affects only children admitted before their first instruction.
        kernel.CloseHandle(job)
        if process.thread:
            kernel.CloseHandle(process.thread)
        if process.process:
            kernel.CloseHandle(process.process)


def unix_run(argv: list[str], log: Path, timeout: float) -> dict:
    if not all(hasattr(os, name) for name in ("waitid", "WNOWAIT")):
        raise RuntimeError("Non-reaping owned-group observation unavailable on this host")
    started = time.monotonic()
    with log.open("wb") as output:
        child = subprocess.Popen(argv, stdin=subprocess.DEVNULL, stdout=output, stderr=output, start_new_session=True)
        timed_out = False
        try:
            while os.waitid(os.P_PID, child.pid, os.WEXITED | os.WNOHANG | os.WNOWAIT) is None:
                if time.monotonic() - started >= timeout:
                    timed_out = True
                    break
                time.sleep(0.02)
            # The leader remains unreaped here, preventing reuse of this owned process-group ID.
            try:
                os.killpg(child.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            code = child.wait(timeout=10)
            return {"exit_code": code, "timeout": timed_out, "owned_group_stop_requested": True, "owned_job_empty": None, "seconds": time.monotonic() - started}
        finally:
            if child.returncode is None:
                try:
                    os.killpg(child.pid, signal.SIGKILL)
                except ProcessLookupError:
                    pass
                child.wait(timeout=10)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--run-id", required=True)
    parser.add_argument("--artifacts", type=Path, required=True)
    parser.add_argument("--target-root", type=Path, required=True)
    parser.add_argument("--logs", type=Path, required=True)
    parser.add_argument("--timeout", type=float, default=45)
    args = parser.parse_args()
    target = args.target_root.resolve(strict=True)
    if not args.target_root.is_absolute() or not 1 <= args.timeout <= 120:
        raise ValueError("Explicit absolute target and bounded timeout required")
    args.logs.mkdir(parents=True, exist_ok=True)
    receipt = {"run_id": args.run_id, "completed": False, "results": []}
    receipt_path = args.logs / "results.json"
    def save_receipt() -> None:
        replacement = receipt_path.with_suffix(".pending")
        replacement.write_text(json.dumps(receipt, indent=2) + "\n", encoding="utf-8")
        replacement.replace(receipt_path)
    save_receipt()
    artifacts = []
    with args.artifacts.open(encoding="utf-8-sig") as source:
        for line in source:
            message = json.loads(line)
            if message.get("reason") == "compiler-artifact" and message.get("profile", {}).get("test") and message.get("executable"):
                executable = Path(message["executable"]).resolve(strict=True)
                if target not in executable.parents:
                    raise ValueError("Executable escapes the reviewed Cargo target root")
                if executable not in artifacts:
                    artifacts.append(executable)
    if not artifacts:
        raise ValueError("No compiled test executables found")
    os.environ["WILDE_RUNTIME_FIXTURE_GUARD"] = "owned-runner-watchdog-v1"
    os.environ["ORCA_BACKGROUND_LAUNCH"] = "1"
    results = receipt["results"]
    failed = False
    for number, executable in enumerate(artifacts):
        log = args.logs / f"{number:02d}-{executable.stem}.log"
        runner = windows_run if os.name == "nt" else unix_run
        # Preserve early failure receipts even when a later test reaches the owned watchdog.
        result = runner([str(executable), "--test-threads=1", "--nocapture"], log, args.timeout)
        result.update(executable=str(executable), log=str(log))
        results.append(result)
        save_receipt()
        print(f"{executable.name}: exit={result['exit_code']} timeout={result['timeout']}", flush=True)
        if result["exit_code"] != 0 or result["timeout"] or result.get("forced_remaining_children") or result.get("owned_job_empty") is False:
            failed = True
            if result.get("owned_job_empty") is False:
                receipt["completed"] = True
                save_receipt()
                return 1
    receipt["completed"] = True
    save_receipt()
    return int(failed)


if __name__ == "__main__":
    raise SystemExit(main())
