use std::io;
use std::path::{Path, PathBuf};
use std::time::Instant;

pub(crate) fn remove_before(root: &Path, deadline: Instant) -> io::Result<()> {
    let manifest = removal_manifest(root)?;
    for path in manifest {
        if !path.starts_with(root) {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "fixture removal escaped owned root",
            ));
        }
        if Instant::now() >= deadline {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "fixture directory cleanup deadline",
            ));
        }
        ensure_regular_directory(&path)?;
        match std::fs::remove_dir(&path) {
            Ok(()) => (),
            Err(error) if error.kind() == io::ErrorKind::NotFound => (),
            Err(error) => return Err(error),
        }
    }
    Ok(())
}

fn removal_manifest(root: &Path) -> io::Result<Vec<PathBuf>> {
    match ensure_regular_directory(root) {
        Ok(()) => (),
        Err(error) if error.kind() == io::ErrorKind::NotFound => return Ok(Vec::new()),
        Err(error) => return Err(error),
    }
    let Some(app_data) = only_child(root, "AppData")? else {
        return Ok(vec![root.to_owned()]);
    };
    let roaming = only_child(&app_data, "Roaming")?;
    let mut paths = Vec::with_capacity(3);
    if let Some(roaming) = roaming {
        ensure_regular_directory(&roaming)?;
        if std::fs::read_dir(&roaming)?.next().transpose()?.is_some() {
            return Err(unexpected_entry());
        }
        paths.push(roaming);
    }
    paths.push(app_data);
    paths.push(root.to_owned());
    Ok(paths)
}

fn only_child(directory: &Path, name: &str) -> io::Result<Option<PathBuf>> {
    ensure_regular_directory(directory)?;
    let mut entries = std::fs::read_dir(directory)?;
    let Some(entry) = entries.next().transpose()? else {
        return Ok(None);
    };
    if entry.file_name() != name || entries.next().transpose()?.is_some() {
        return Err(unexpected_entry());
    }
    let child = directory.join(name);
    ensure_regular_directory(&child)?;
    Ok(Some(child))
}

fn ensure_regular_directory(path: &Path) -> io::Result<()> {
    let metadata = std::fs::symlink_metadata(path)?;
    let is_link = metadata.file_type().is_symlink();
    #[cfg(windows)]
    let is_link = {
        use std::os::windows::fs::MetadataExt;
        is_link || metadata.file_attributes() & 0x400 != 0
    };
    if is_link || !metadata.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "fixture cleanup refuses links and non-directory entries",
        ));
    }
    Ok(())
}

fn unexpected_entry() -> io::Error {
    io::Error::new(
        io::ErrorKind::DirectoryNotEmpty,
        "fixture contains an unexpected entry; retaining manifest",
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::MAX_CLEANUP_WAIT;
    use crate::fixture_command::FixtureDirectory;

    #[test]
    fn exact_empty_windows_startup_manifest_is_removed_bottom_up() {
        let directory = FixtureDirectory::create().unwrap();
        let roaming = directory.0.join("AppData").join("Roaming");
        std::fs::create_dir_all(&roaming).unwrap();
        remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap();
        assert!(!directory.0.exists());
    }

    #[test]
    fn unexpected_file_preserves_the_whole_manifest_before_any_removal() {
        let directory = FixtureDirectory::create().unwrap();
        let roaming = directory.0.join("AppData").join("Roaming");
        std::fs::create_dir_all(&roaming).unwrap();
        let file = directory.0.join("unexpected-retained-file");
        std::fs::write(&file, b"synthetic").unwrap();
        let error = remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::DirectoryNotEmpty);
        assert!(
            file.is_file() && roaming.is_dir(),
            "validation must precede all manifest mutation"
        );
        std::fs::remove_file(file).unwrap();
        remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap();
    }

    #[test]
    fn expired_cleanup_budget_keeps_the_validated_manifest() {
        let directory = FixtureDirectory::create().unwrap();
        let roaming = directory.0.join("AppData").join("Roaming");
        std::fs::create_dir_all(&roaming).unwrap();
        let error = remove_before(&directory.0, Instant::now()).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::TimedOut);
        assert!(roaming.is_dir());
        remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap();
    }

    #[cfg(any(windows, unix))]
    #[test]
    fn linked_profile_directory_is_refused_without_following_target() {
        let directory = FixtureDirectory::create().unwrap();
        let target = FixtureDirectory::create().unwrap();
        let link = directory.0.join("AppData");
        #[cfg(unix)]
        let result = std::os::unix::fs::symlink(&target.0, &link);
        #[cfg(windows)]
        let result = std::os::windows::fs::symlink_dir(&target.0, &link);
        #[cfg(windows)]
        if result
            .as_ref()
            .is_err_and(|error| error.raw_os_error() == Some(1314))
        {
            eprintln!(
                "NOT RUN: Windows denied symlink creation; native reparse cleanup rejection remains unverified"
            );
            return;
        }
        result.unwrap();
        let error = remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap_err();
        assert_eq!(error.kind(), io::ErrorKind::InvalidInput);
        assert!(target.0.is_dir());
        #[cfg(unix)]
        std::fs::remove_file(&link).unwrap();
        #[cfg(windows)]
        std::fs::remove_dir(&link).unwrap();
        remove_before(&directory.0, Instant::now() + MAX_CLEANUP_WAIT).unwrap();
    }
}
