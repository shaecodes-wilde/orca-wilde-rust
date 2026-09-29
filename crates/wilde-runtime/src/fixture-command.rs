use anyhow::{Context, Result, bail};
use portable_pty::{CommandBuilder, PtySize};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use wilde_contracts::{SessionIdentity, StreamCursor};

static NEXT_CWD: AtomicU64 = AtomicU64::new(1);

pub(crate) fn validate(identity: &SessionIdentity, size: PtySize) -> Result<()> {
    StreamCursor::from_snapshot(identity.clone(), 0)
        .map_err(|error| anyhow::anyhow!("invalid fixture identity: {error:?}"))?;
    if identity.generation == 0 || identity.execution_host_id != "local-fixture" {
        bail!("fixture requires local-fixture host and nonzero generation");
    }
    for value in [&identity.session_id, &identity.workspace_id] {
        if value.len() > 240 || value.chars().any(char::is_control) {
            bail!("invalid bounded fixture identifier");
        }
    }
    validate_size(size)
}

pub(crate) fn validate_size(size: PtySize) -> Result<()> {
    if size.rows == 0 || size.cols == 0 || size.rows > 500 || size.cols > 500 {
        bail!("fixture dimensions must be between 1 and 500");
    }
    if size.pixel_width != 0 || size.pixel_height != 0 {
        bail!("fixture uses character dimensions only");
    }
    Ok(())
}

pub(crate) struct FixtureDirectory(pub PathBuf, pub bool);

impl FixtureDirectory {
    pub fn create() -> Result<Self> {
        let base = std::env::temp_dir()
            .canonicalize()
            .context("resolve temporary root")?;
        for _ in 0..16 {
            let serial = NEXT_CWD.fetch_add(1, Ordering::Relaxed);
            let path = base.join(format!("wilde-pty-fixture-{}-{serial}", std::process::id()));
            match std::fs::create_dir(&path) {
                Ok(()) => return Ok(Self(path, true)),
                Err(error) if error.kind() == std::io::ErrorKind::AlreadyExists => continue,
                Err(error) => return Err(error).context("create fixture cwd"),
            }
        }
        bail!("fixture cwd collision budget exhausted")
    }

    pub fn remove(&self) -> bool {
        // The fixture creates no files; never recursively remove an unexpected occupant.
        std::fs::remove_dir(&self.0).is_ok() || !self.0.exists()
    }
}

impl Drop for FixtureDirectory {
    fn drop(&mut self) {
        if self.1 {
            let _ = self.remove();
        }
    }
}

pub(crate) fn command(cwd: &Path) -> Result<CommandBuilder> {
    #[cfg(windows)]
    let mut command = {
        let root = std::env::var_os("SystemRoot").context("SystemRoot is unavailable")?;
        let exe = PathBuf::from(&root)
            .join("System32")
            .join("WindowsPowerShell")
            .join("v1.0")
            .join("powershell.exe");
        if !exe.is_absolute() || !exe.is_file() {
            bail!("installed Windows PowerShell fixture executable is unavailable");
        }
        let mut command = CommandBuilder::new(exe);
        command.env_clear();
        command.env("SystemRoot", root);
        command.args([
            "-NoLogo",
            "-NoProfile",
            "-NonInteractive",
            "-Command",
            WINDOWS_FIXTURE,
        ]);
        command
    };
    #[cfg(unix)]
    let mut command = {
        if !Path::new("/bin/sh").is_file() {
            bail!("installed /bin/sh fixture executable is unavailable");
        }
        let mut command = CommandBuilder::new("/bin/sh");
        command.env_clear();
        command.args(["-c", UNIX_FIXTURE]);
        command
    };
    #[cfg(not(any(windows, unix)))]
    return Err(anyhow::anyhow!("fixture platform unsupported"));
    #[cfg(any(windows, unix))]
    {
        command.cwd(cwd);
        command.env("HOME", cwd);
        command.env("USERPROFILE", cwd);
        command.env("TEMP", cwd);
        command.env("TMP", cwd);
        command.env("TERM", "xterm-256color");
        Ok(command)
    }
}

#[cfg(windows)]
const WINDOWS_FIXTURE: &str = r#"$ErrorActionPreference='Stop'; [Console]::Write(([char]27)+'[32mWILDE_FIXTURE_READY'+([char]27)+"[0m`r`n"); while ($true) { $line=[Console]::ReadLine(); if ($null -eq $line) { exit 0 }; switch ($line) { 'ping' { [Console]::Write("WILDE_FIXTURE_PONG`r`n") } 'finish' { [Console]::Write("WILDE_FIXTURE_DONE`r`n"); exit 0 } default { exit 23 } } }"#;

#[cfg(unix)]
const UNIX_FIXTURE: &str = r#"printf '\033[32mWILDE_FIXTURE_READY\033[0m\r\n'
while IFS= read -r line; do
    case "$line" in
        ping) printf 'WILDE_FIXTURE_PONG\r\n' ;;
        finish) printf 'WILDE_FIXTURE_DONE\r\n'; exit 0 ;;
        *) exit 23 ;;
    esac
done
exit 0"#;
