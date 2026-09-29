use std::{
    fs,
    path::{Component, Path, PathBuf},
};

pub const PROFILE_MARKER: &str = "wilde-native-rust-dev-profile-v1\n";
const ROOT_NAME: &str = "wilde-native-rust-dev-profiles";

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileError {
    InvalidRoot,
    InvalidId,
    Link,
    MissingMarker,
    Io,
}

#[derive(Debug)]
pub struct DevProfile {
    root: PathBuf,
    id: String,
    directory: PathBuf,
}
impl DevProfile {
    /// Inspects a caller-provisioned synthetic profile; never creates or deletes data.
    pub fn inspect(root: &Path, id: &str) -> Result<Self, ProfileError> {
        if !root.is_absolute()
            || root.file_name().and_then(|v| v.to_str()) != Some(ROOT_NAME)
            || root
                .components()
                .any(|c| matches!(c, Component::ParentDir | Component::CurDir))
        {
            return Err(ProfileError::InvalidRoot);
        }
        if id.is_empty()
            || id.len() > 64
            || !id.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')
        {
            return Err(ProfileError::InvalidId);
        }
        let directory = root.join(id);
        for path in directory.ancestors() {
            let metadata = fs::symlink_metadata(path).map_err(|_| ProfileError::Io)?;
            if is_link(&metadata) {
                return Err(ProfileError::Link);
            }
            if !metadata.is_dir() {
                return Err(ProfileError::InvalidRoot);
            }
        }
        let canonical_root = fs::canonicalize(root).map_err(|_| ProfileError::Io)?;
        let canonical_directory = fs::canonicalize(&directory).map_err(|_| ProfileError::Io)?;
        if canonical_directory.parent() != Some(canonical_root.as_path()) {
            return Err(ProfileError::InvalidRoot);
        }
        let marker = directory.join(".wilde-native-rust-dev-profile");
        let metadata = fs::symlink_metadata(&marker).map_err(|_| ProfileError::MissingMarker)?;
        if is_link(&metadata)
            || !metadata.is_file()
            || metadata.len() != PROFILE_MARKER.len() as u64
        {
            return Err(ProfileError::MissingMarker);
        }
        if fs::read(&marker).map_err(|_| ProfileError::Io)? != PROFILE_MARKER.as_bytes() {
            return Err(ProfileError::MissingMarker);
        }
        Ok(Self {
            root: root.into(),
            id: id.into(),
            directory: canonical_directory,
        })
    }
    pub fn directory(&self) -> &Path {
        &self.directory
    }
    pub fn revalidate(&self) -> Result<(), ProfileError> {
        let current = Self::inspect(&self.root, &self.id)?;
        if current.directory != self.directory {
            return Err(ProfileError::InvalidRoot);
        }
        Ok(())
    }
}
fn is_link(metadata: &fs::Metadata) -> bool {
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        metadata.file_attributes() & 0x400 != 0
    }
    #[cfg(not(windows))]
    {
        metadata.file_type().is_symlink()
    }
}
