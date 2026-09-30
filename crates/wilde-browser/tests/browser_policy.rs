use std::{
    fs,
    path::PathBuf,
    time::{SystemTime, UNIX_EPOCH},
};
use wilde_browser::*;

fn fixture_root() -> PathBuf {
    let base = PathBuf::from(
        std::env::var_os("WILDE_BROWSER_TEST_ROOT")
            .expect("set WILDE_BROWSER_TEST_ROOT to an F-drive build evidence directory"),
    );
    assert!(base.is_absolute());
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let root = base
        .join(format!("browser-policy-{}-{nonce}", std::process::id()))
        .join("wilde-native-rust-dev-profiles");
    fs::create_dir_all(root.join("synthetic-1")).unwrap();
    root
}
fn page(id: &str, recency: u64) -> WarmPage {
    WarmPage {
        page_id: id.into(),
        unsaved: UnsavedState::Clean,
        protection: Protection::default(),
        last_used_ms: recency,
        resident_since_ms: 0,
    }
}
#[test]
fn explicit_marked_profile_succeeds_but_default_existing_and_traversal_fail() {
    let root = fixture_root();
    assert!(matches!(
        DevProfile::inspect(&root, "synthetic-1"),
        Err(ProfileError::MissingMarker)
    ));
    fs::write(
        root.join("synthetic-1/.wilde-native-rust-dev-profile"),
        PROFILE_MARKER,
    )
    .unwrap();
    let profile = DevProfile::inspect(&root, "synthetic-1").unwrap();
    profile.revalidate().unwrap();
    assert!(matches!(
        DevProfile::inspect(&root, "../production"),
        Err(ProfileError::InvalidId)
    ));
    assert!(matches!(
        DevProfile::inspect(&PathBuf::from("."), "synthetic-1"),
        Err(ProfileError::InvalidRoot)
    ));
    fs::write(
        root.join("synthetic-1/.wilde-native-rust-dev-profile"),
        "production-profile\n",
    )
    .unwrap();
    assert_eq!(profile.revalidate(), Err(ProfileError::MissingMarker));
    // Retain these synthetic files as test evidence; profile closure never deletes stored state.
}
#[cfg(unix)]
#[test]
fn redirected_profile_is_rejected_even_with_a_valid_marker() {
    use std::os::unix::fs::symlink;
    let root = fixture_root();
    let outside = root.parent().unwrap().join("outside");
    fs::create_dir_all(&outside).unwrap();
    fs::write(
        outside.join(".wilde-native-rust-dev-profile"),
        PROFILE_MARKER,
    )
    .unwrap();
    symlink(&outside, root.join("redirected")).unwrap();
    assert!(matches!(
        DevProfile::inspect(&root, "redirected"),
        Err(ProfileError::Link)
    ));
}
#[test]
fn navigation_cannot_escape_allowlist_via_credentials_schemes_host_suffix_or_backslash() {
    let policy = NavigationPolicy::https_only(&["https://example.test"]).unwrap();
    assert!(policy.allows("https://example.test/path?next=https://evil.test"));
    for url in [
        "https://example.test.evil.test/",
        "https://example.test@evil.test/",
        "https://example.test\\@evil.test/",
        "https://%65xample.test/",
        "file:///etc/passwd",
        "javascript:alert(1)",
        "http://example.test/",
        "https://example.test:443/",
        "https://example.test/\n",
    ] {
        assert!(!policy.allows(url), "unexpected allowed URL: {url}");
    }
    assert!(NavigationPolicy::https_only(&["https://example.test/path"]).is_err());
}
#[test]
fn closing_one_owner_preserves_other_pages_and_duplicate_ids_fail_closed() {
    let mut pages = PageOwnership::default();
    pages.register("host-a", "page-a").unwrap();
    pages.register("host-b", "page-b").unwrap();
    assert_eq!(
        pages.register("host-b", "page-a"),
        Err(PolicyError::DuplicatePage)
    );
    assert_eq!(
        pages.release("host-b", "page-a"),
        Err(PolicyError::WrongOwner)
    );
    assert_eq!(pages.owned_pages("host-a"), vec!["page-a"]);
    pages.release("host-a", "page-a").unwrap();
    assert_eq!(pages.owned_pages("host-b"), vec!["page-b"]);
    assert_eq!(
        pages.release("host-a", "page-a"),
        Err(PolicyError::UnknownPage)
    );
}
#[test]
fn visible_dirty_unknown_media_download_and_lease_pages_are_outside_warm_budget() {
    let mut pages = vec![page("old", 1), page("recent", 2)];
    for (id, unsaved, protection) in [
        ("dirty", UnsavedState::Dirty, Protection::default()),
        ("unknown", UnsavedState::Unknown, Protection::default()),
        (
            "visible",
            UnsavedState::Clean,
            Protection {
                visible: true,
                ..Default::default()
            },
        ),
        (
            "media",
            UnsavedState::Clean,
            Protection {
                media: true,
                ..Default::default()
            },
        ),
        (
            "download",
            UnsavedState::Clean,
            Protection {
                transfer: true,
                ..Default::default()
            },
        ),
        (
            "lease-expired-not-reconciled",
            UnsavedState::Clean,
            Protection {
                automation_lease: true,
                ..Default::default()
            },
        ),
        (
            "remote",
            UnsavedState::Clean,
            Protection {
                remote_viewer: true,
                ..Default::default()
            },
        ),
        (
            "keepalive",
            UnsavedState::Clean,
            Protection {
                keep_alive: true,
                ..Default::default()
            },
        ),
    ] {
        let mut protected = page(id, 0);
        protected.unsaved = unsaved;
        protected.protection = protection;
        pages.push(protected);
    }
    assert_eq!(
        eviction_plan(&pages, 1, 100, 10).unwrap(),
        EvictionPlan {
            evict: vec!["old".into()],
            deferred_over_budget: 0,
            protected_resident_pages: 8,
            remaining_resident_above_warm_budget: 8,
        }
    );
}
#[test]
fn cooldown_and_clock_uncertainty_defer_budget_without_repeated_eviction() {
    let mut recent = page("recent", 10);
    recent.resident_since_ms = 95;
    assert_eq!(
        eviction_plan(&[recent.clone()], 0, 100, 10)
            .unwrap()
            .deferred_over_budget,
        1
    );
    assert_eq!(
        eviction_plan(&[recent.clone()], 0, 110, 10).unwrap().evict,
        vec!["recent"]
    );
    assert_eq!(
        eviction_plan(&[recent], 0, 90, 10)
            .unwrap()
            .deferred_over_budget,
        1
    );
    let duplicate = page("duplicate", 1);
    assert_eq!(
        eviction_plan(&[duplicate.clone(), duplicate], 0, 100, 10),
        Err(RetentionError::DuplicatePage)
    );
}
#[test]
fn bounds_and_launch_reject_infinite_outside_background_and_unisolated_scenes() {
    assert!(SurfaceBounds::within(100., 0., 200., 100., 1.5, 500., 300.).is_ok());
    for (x, width, scale) in [
        (-1., 100., 1.),
        (450., 100., 1.),
        (0., f64::NAN, 1.),
        (0., 100., f64::INFINITY),
        (0., 100., 0.),
    ] {
        assert_eq!(
            SurfaceBounds::within(x, 0., width, 100., scale, 500., 300.),
            Err(SurfaceError::InvalidBounds)
        );
    }
    assert_eq!(
        validate_scene_launch(true, true),
        Err(SurfaceError::BackgroundLaunch)
    );
    assert_eq!(
        validate_scene_launch(false, false),
        Err(SurfaceError::IsolationRequired)
    );
    assert_eq!(
        non_windows_embedding(),
        Err(SurfaceError::UnsupportedPlatform)
    );
}

#[test]
fn synthetic_initial_navigation_loads_once_and_rejects_external_or_repeat_navigation() {
    let initial = InitialSyntheticNavigation::default();
    assert!(initial.allows("about:blank"));
    assert!(!initial.allows("about:blank"));
    assert!(!initial.allows("https://example.test/"));
    let invalid = InitialSyntheticNavigation::default();
    assert!(!invalid.allows("https://example.test/"));
    assert!(!invalid.allows("about:blank"));
}

#[test]
fn protected_only_pressure_is_visible_without_scheduling_eviction() {
    let mut dirty = page("dirty", 1);
    dirty.unsaved = UnsavedState::Dirty;
    let mut unknown = page("unknown", 2);
    unknown.unsaved = UnsavedState::Unknown;
    let mut visible = page("visible", 3);
    visible.protection.visible = true;
    let plan = eviction_plan(&[dirty, unknown, visible], 1, 100, 10).unwrap();
    assert!(plan.evict.is_empty());
    assert_eq!(plan.deferred_over_budget, 0);
    assert_eq!(plan.protected_resident_pages, 3);
    assert_eq!(plan.remaining_resident_above_warm_budget, 2);
}

#[test]
fn mixed_pressure_distinguishes_protection_from_cooldown_and_clock_rollback() {
    let mut dirty = page("dirty", 0);
    dirty.unsaved = UnsavedState::Dirty;
    let mut recent = page("recent", 95);
    recent.resident_since_ms = 95;
    let pages = [dirty, page("old", 1), recent];
    let plan = eviction_plan(&pages, 0, 100, 10).unwrap();
    assert_eq!(plan.evict, vec!["old"]);
    assert_eq!(plan.deferred_over_budget, 1);
    assert_eq!(plan.protected_resident_pages, 1);
    assert_eq!(plan.remaining_resident_above_warm_budget, 2);

    let rollback = eviction_plan(&pages, 0, 0, 10).unwrap();
    assert!(rollback.evict.is_empty());
    assert_eq!(rollback.deferred_over_budget, 2);
    assert_eq!(rollback.protected_resident_pages, 1);
    assert_eq!(rollback.remaining_resident_above_warm_budget, 3);

    let settled = eviction_plan(&pages, 0, 110, 10).unwrap();
    assert_eq!(settled.evict, vec!["old", "recent"]);
    assert_eq!(settled.deferred_over_budget, 0);
    assert_eq!(settled.protected_resident_pages, 1);
    assert_eq!(settled.remaining_resident_above_warm_budget, 1);
    assert_eq!(pages[0].unsaved, UnsavedState::Dirty);
}
