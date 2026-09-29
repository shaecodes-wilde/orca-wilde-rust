# A6 browser embedding and policy recon

Status: headless policy implemented; native wrapper compile spike awaiting A1's serial check. No browser/window/event loop launched. Actual runner: Codex GPT-6.1-Sol High per coordinator routing; no independent runner-metadata API available in this worker.

## Acceptance boundary

Existing parent handle belongs to A1's main-thread isolated event loop. A4 owns nonoverlapping logical browser bounds and actual display DPI; A5's terminal is a native sibling outside that rectangle. A6 borrows parent and owner, stores WebView before WebContext so disposal drops the child first, and keeps profile data. A gated child visibility API is exposed only to the existing isolated owner; no parent activation or focus API exists. ORCA_BACKGROUND_LAUNCH=1 refuses construction; normal desktop execution is forbidden. The unsafe owner constructor requires real main-thread/isolated-display provenance from the caller; an attestation is not evidence. A borrowed window cannot prove its OS handle was not manually destroyed: A1 must retain the actual parent until child drop.

The only renderer content is fixed synthetic HTML with restrictive CSP. Only the first about:blank navigation is allowed for WebView2 NavigateToString to load fixed HTML; subsequent/external navigation, popups, downloads and automatic clipboard grants are disabled; no application IPC handler, initialization scripts, custom protocol, CDP endpoint or cookie migration exists. General websites return PermissionsUnsupported. WRY 0.54.4 lacks a general permission callback. A policy returning Deny is not proof that WebView2 enforces it. Production website acceptance needs native permission event controls, permissions/UI tests, request/subframe protections and auth/download/audio cases.

## Profile boundary

WebContext::default/new(None) is forbidden. DevProfile inspects only an explicit absolute root named wilde-native-rust-dev-profiles, a single ASCII profile ID and an exact dev marker; rejects symlink/reparse ancestors and verifies canonical containment. Caller provisions synthetic directories separately. Revalidate immediately before engine creation; no creation/deletion in adapter. Concurrent malicious filesystem substitution is not fully prevented by path checks; keep this root under trusted user control. This boundary prevents accidental profile reuse, not hostile same-user filesystem mutation.

WebView2 controls sharing a user-data folder share a session, including across host applications. Separate folders can cost extra browser processes. Closing a control does not prove all browser processes exited; never delete its profile solely after Drop. [Microsoft user-data-folder guidance](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/user-data-folder).

## Platform/candidate matrix

| Candidate | Windows | Linux/Wayland | macOS | Current evidence |
|---|---|---|---|---|
| WRY 0.54.4 / system WebView | build_as_child accepts existing Win32 handle | generic child API explicitly X11-only; GTK Fixed/container needed, native GPU toolkit composition unproved | cached docs describe NSView subview; no runner | cached source/API inspection; GUI NOT RUN |
| CEF / Chromium | native parent child and offscreen APIs exist | embedding of arbitrary toolkit Wayland child not proved; windowless route needs actual pixels/input/accessibility integration | app/helper packaging required | official upstream docs only; no pinned binary, dependency or executable |

Cached tao is 0.34.8; raw-window-handle 0.6.2. Wrapper depends on exact wry 0.54.4, optional Windows target only, default features off with os-webview. It does not use tao or own the event loop. Updated WRY release selection is outside this compile spike; cached version is evidence scope, not a production recommendation.

Official WRY docs describe the GTK-specific Wayland path: [WRY docs](https://docs.rs/wry/latest/wry/). Exact cached 0.54.4 lib.rs lines 1490-1514 independently state the generic API does not work on Wayland. Current docs have moved beyond this cached version. Windows native child parent rules exist at [CreateWindowExW](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-createwindowexw); API availability does not establish toolkit clipping/overlay behavior.

CEF offers windowed and offscreen embedding; offscreen hosts own pixels, input, resize and asynchronous disposal. Rust bindings, chosen CEF version, sandbox integration, distribution notices and GPU texture handling remain unselected. The general guide's old accelerated-compositing statement is not treated as a current performance limitation; it needs verification against a selected release. [CEF upstream general usage](https://chromiumembedded.github.io/cef/general_usage), [current CEF window header](https://github.com/chromiumembedded/cef/blob/master/include/internal/cef_win.h).

Wayland clients coordinate surfaces through their compositor; that is not evidence for reparenting one toolkit's arbitrary native browser child. [Wayland protocol model](https://wayland.freedesktop.org/docs/book/Protocol.html). Linux adapter intentionally returns UnsupportedPlatform until GTK/native scene evidence exists.

## Reused source behavior and limits

Pinned source 1db4ef38577ec932f97067b7391625b7be599884 separates page creation/close from backing engine. Its current retention baseline is eight warm hidden pages, excluding visible/protected pages. This crate retains that budget and adds explicit dirty/unknown protections, media/transfers, leases awaiting owner reconciliation, remote viewers, keep-alive and cooldown. Plans do not mutate resources; resource owners must recheck immediately before destroying. Page counts do not attribute RAM or promise a cap. No freeze support, live heap transfer, process metrics or benchmarks claimed.

## A1 independent contract review

Reviewed initial worktree contracts and repaired shared A1 artifact. Initial concerns: closed verdict enum; unbounded identities; no bounded pre-decode ingress; payload host identity mistaken for authority. Repaired source now handles unknown string verdict as Unverifiable but rejects wrong types; validates bounded IDs; bounds raw wire bytes before decoding and decoded payload afterward; accepts an authorized-host argument and rejects mismatches. APPROVED for the local pilot contract semantics. Real transport authentication/framing, flow control and legacy cross-version pairing are not verified. No legacy endpoint is connected.

## Required next evidence

A1 runs headless tests and compile check with outputs on F. Then isolated Windows VM/CI scene must show actual page, sibling controls and terminal, multi-DPI resize, keyboard/IME focus, clipping/overlays, accessibility, repeated close/reopen and browser process ownership. Linux/Wayland GTK path and macOS runner remain independent gates. All GUI gates NOT RUN. Startup/idle/memory/latency benchmark NOT RUN; no speedup claim. No installer or platform integration touched.

Initial load correction: cached WebView2 path registers NavigationStarting before NavigateToString, so denying every URI can cancel the synthetic fixture. A headless stateful gate admits exactly the first about:blank and rejects all later requests. Fixed HTML includes browser input/textarea for isolated typing checks. API semantics documented by [Microsoft NavigateToString](https://learn.microsoft.com/en-us/microsoft-edge/webview2/reference/win32/icorewebview2#navigatetostring); actual engine event order/fixture load still requires GUI evidence.
