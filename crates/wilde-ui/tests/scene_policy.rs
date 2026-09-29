use std::path::{Path, PathBuf};
use wilde_ui::focus::{FocusDirection, FocusRoute};
use wilde_ui::launch::{DEVELOPMENT_APPLICATION_ID, LaunchError, LaunchRequest, admit};
use wilde_ui::layout::{LayoutError, Region, SceneLayout};

const REGIONS: [Region; 4] = [
    Region::Sidebar,
    Region::NativeInput,
    Region::Terminal,
    Region::Browser,
];

#[test]
fn repeated_resize_keeps_surfaces_disjoint_and_inside_logical_window() {
    for (width, height) in [
        (64, 64),
        (1024, 768),
        (1920, 1080),
        (3840, 2160),
        (64, 8192),
        (8192, 64),
    ] {
        for scale in [0.5, 1.0, 1.25, 1.5, 2.0, 4.0] {
            let layout = SceneLayout::from_physical(width, height, scale).unwrap();
            for (index, region) in REGIONS.iter().enumerate() {
                if let Some(bounds) = layout.bounds(*region) {
                    assert!(bounds.width() > 0.0 && bounds.height() > 0.0);
                    assert!(bounds.x() >= 0.0 && bounds.y() >= 0.0);
                    assert!(bounds.x() + bounds.width() <= layout.width() + 1e-6);
                    assert!(bounds.y() + bounds.height() <= layout.height() + 1e-6);
                    for other in REGIONS.iter().skip(index + 1) {
                        let other = layout.bounds(*other).unwrap();
                        assert!(
                            bounds.x() + bounds.width() <= other.x()
                                || other.x() + other.width() <= bounds.x()
                                || bounds.y() + bounds.height() <= other.y()
                                || other.y() + other.height() <= bounds.y()
                        );
                    }
                }
            }
        }
    }
}

#[test]
fn minimizing_and_restoring_removes_and_restores_all_surfaces() {
    for (width, height) in [(0, 0), (0, 720), (1280, 0), (1, 1), (63, 100), (100, 63)] {
        let layout = SceneLayout::from_physical(width, height, 1.0).unwrap();
        assert!(layout.is_suspended());
        assert!(
            REGIONS
                .iter()
                .all(|region| layout.bounds(*region).is_none())
        );
        assert_eq!(
            FocusRoute::step(Some(Region::Browser), FocusDirection::Forward, &layout),
            None
        );
    }
    assert!(
        !SceneLayout::from_physical(1280, 720, 1.0)
            .unwrap()
            .is_suspended()
    );
}

#[test]
fn scaling_changes_logical_bounds_without_reusing_stale_coordinates() {
    let before = SceneLayout::from_physical(1920, 1080, 1.0).unwrap();
    let after = SceneLayout::from_physical(1920, 1080, 2.0).unwrap();
    assert_eq!((after.width(), after.height()), (960.0, 540.0));
    assert!(
        after.bounds(Region::Browser).unwrap().width()
            < before.bounds(Region::Browser).unwrap().width()
    );
    assert!(
        SceneLayout::from_physical(64, 64, 2.0)
            .unwrap()
            .is_suspended()
    );
}

#[test]
fn invalid_dpi_and_coordinate_overflow_are_rejected() {
    for scale in [0.0, -1.0, f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert_eq!(
            SceneLayout::from_physical(100, 100, scale),
            Err(LayoutError::InvalidScale)
        );
    }
    for scale in [f64::MIN_POSITIVE, 1e-12] {
        assert_eq!(
            SceneLayout::from_physical(u32::MAX, 100, scale),
            Err(LayoutError::CoordinateOverflow)
        );
    }
    assert_eq!(
        SceneLayout::from_physical(u32::MAX, 100, 1.0),
        Err(LayoutError::CoordinateOverflow)
    );
}

#[test]
fn focus_traversal_and_reverse_traversal_wrap_between_native_and_browser_regions() {
    let layout = SceneLayout::from_physical(1280, 720, 1.0).unwrap();
    let mut current = None;
    for expected in REGIONS.into_iter().chain([Region::Sidebar]) {
        current = FocusRoute::step(current, FocusDirection::Forward, &layout);
        assert_eq!(current, Some(expected));
    }
    for expected in [
        Region::Browser,
        Region::Terminal,
        Region::NativeInput,
        Region::Sidebar,
    ] {
        current = FocusRoute::step(current, FocusDirection::Backward, &layout);
        assert_eq!(current, Some(expected));
    }
    assert_eq!(
        FocusRoute::step(None, FocusDirection::Backward, &layout),
        Some(Region::Browser)
    );
}

fn dev_root() -> PathBuf {
    if cfg!(windows) {
        PathBuf::from(r"C:\synthetic\wilde-native-rust-dev-profiles")
    } else {
        PathBuf::from("/synthetic/wilde-native-rust-dev-profiles")
    }
}

fn request(root: &Path) -> LaunchRequest<'_> {
    LaunchRequest {
        background_launch: None,
        isolated_display_flag: true,
        isolated_display_env: Some("1"),
        dev_profile_root: Some(root),
        application_id: DEVELOPMENT_APPLICATION_ID,
    }
}

#[test]
fn background_denial_overrides_all_other_opt_ins() {
    let root = dev_root();
    let mut request = request(&root);
    request.background_launch = Some("1");
    assert_eq!(admit(request), Err(LaunchError::BackgroundLaunchForbidden));
}

#[test]
fn each_missing_isolation_signal_denies_launch() {
    let root = dev_root();
    let mut request = request(&root);
    request.isolated_display_flag = false;
    assert_eq!(admit(request), Err(LaunchError::IsolatedDisplayRequired));
    request.isolated_display_flag = true;
    for env in [None, Some("0"), Some("true")] {
        request.isolated_display_env = env;
        assert_eq!(admit(request), Err(LaunchError::IsolatedDisplayRequired));
    }
}

#[test]
fn production_identity_and_implicit_or_misnamed_profiles_are_denied() {
    let root = dev_root();
    let mut request = request(&root);
    request.application_id = "systems.wilde.native-rust";
    assert_eq!(
        admit(request),
        Err(LaunchError::DevelopmentIdentityRequired)
    );
    request.application_id = DEVELOPMENT_APPLICATION_ID;
    request.dev_profile_root = None;
    assert_eq!(
        admit(request),
        Err(LaunchError::ExplicitDevelopmentProfileRequired)
    );
    let wrong = root.with_file_name("production-profiles");
    request.dev_profile_root = Some(&wrong);
    assert_eq!(
        admit(request),
        Err(LaunchError::InvalidDevelopmentProfileRoot)
    );
    request.dev_profile_root = Some(Path::new("wilde-native-rust-dev-profiles"));
    assert_eq!(
        admit(request),
        Err(LaunchError::InvalidDevelopmentProfileRoot)
    );
    let traversal = root.join("..").join("wilde-native-rust-dev-profiles");
    request.dev_profile_root = Some(&traversal);
    assert_eq!(
        admit(request),
        Err(LaunchError::InvalidDevelopmentProfileRoot)
    );
}

#[test]
fn admitted_policy_keeps_explicit_root_and_development_identity() {
    let root = dev_root();
    let permit = admit(request(&root)).unwrap();
    assert_eq!(permit.dev_profile_root(), root);
    assert_eq!(permit.application_id(), DEVELOPMENT_APPLICATION_ID);
}
