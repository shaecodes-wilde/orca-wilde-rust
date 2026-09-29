#![cfg_attr(windows, windows_subsystem = "windows")]
use anyhow::{Result, bail};
use std::path::PathBuf;
#[cfg(all(windows, feature = "windows-scene"))]
mod windows_scene;
fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() || args == ["--help"] {
        println!(
            "Wilde Native Rust development pilot.\n--pilot --database <absolute *.wilde-dev.sqlite> runs the synthetic SQLite/PTY proof.\n--scene --isolated-display --profile-root <absolute native dev root> --profile-id <id> runs the Windows-only diagnostic scene, when compiled and independently approved on an isolated display."
        );
        return Ok(());
    }
    if args.first().is_some_and(|arg| arg == "--pilot") {
        if args.len() != 3 || args[1] != "--database" {
            bail!("pilot requires --database <explicit absolute native dev SQLite path>");
        }
        let evidence = wilde_desktop::run_headless_pilot(&PathBuf::from(&args[2]))?;
        println!("{}", serde_json::to_string_pretty(&evidence)?);
        return Ok(());
    }
    if args.first().is_some_and(|arg| arg == "--scene") {
        #[cfg(all(windows, feature = "windows-scene"))]
        {
            return windows_scene::run(&args[1..]);
        }
        #[cfg(not(all(windows, feature = "windows-scene")))]
        bail!("native diagnostic scene is not compiled for this target; no platform substitution");
    }
    bail!("unsupported command; use --help")
}
