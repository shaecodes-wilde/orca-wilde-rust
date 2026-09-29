use std::path::{Component, Path, PathBuf};

pub const DEVELOPMENT_APPLICATION_ID: &str = "systems.wilde.native-rust.dev";
pub const DEVELOPMENT_PROFILE_ROOT_NAME: &str = "wilde-native-rust-dev-profiles";

#[derive(Debug, Clone, Copy)]
pub struct LaunchRequest<'a> {
    pub background_launch: Option<&'a str>,
    pub isolated_display_flag: bool,
    pub isolated_display_env: Option<&'a str>,
    pub dev_profile_root: Option<&'a Path>,
    pub application_id: &'a str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LaunchError {
    BackgroundLaunchForbidden,
    IsolatedDisplayRequired,
    DevelopmentIdentityRequired,
    ExplicitDevelopmentProfileRequired,
    InvalidDevelopmentProfileRoot,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LaunchPermit {
    dev_profile_root: PathBuf,
}

impl LaunchPermit {
    pub fn dev_profile_root(&self) -> &Path {
        &self.dev_profile_root
    }
    pub fn application_id(&self) -> &'static str {
        DEVELOPMENT_APPLICATION_ID
    }
}

// The composer must additionally inspect A6's profile marker and actual display isolation.
pub fn admit(request: LaunchRequest<'_>) -> Result<LaunchPermit, LaunchError> {
    if request.background_launch == Some("1") {
        return Err(LaunchError::BackgroundLaunchForbidden);
    }
    if !request.isolated_display_flag || request.isolated_display_env != Some("1") {
        return Err(LaunchError::IsolatedDisplayRequired);
    }
    if request.application_id != DEVELOPMENT_APPLICATION_ID {
        return Err(LaunchError::DevelopmentIdentityRequired);
    }
    let root = request
        .dev_profile_root
        .ok_or(LaunchError::ExplicitDevelopmentProfileRequired)?;
    if !root.is_absolute()
        || root.file_name() != Some(DEVELOPMENT_PROFILE_ROOT_NAME.as_ref())
        || root
            .components()
            .any(|part| matches!(part, Component::ParentDir | Component::CurDir))
    {
        return Err(LaunchError::InvalidDevelopmentProfileRoot);
    }
    Ok(LaunchPermit {
        dev_profile_root: root.to_path_buf(),
    })
}
