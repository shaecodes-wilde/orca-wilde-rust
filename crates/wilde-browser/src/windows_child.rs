//! Compile spike only. No event loop, window, website, cookie migration or native RPC is created here.
use crate::{
    DevProfile, InitialSyntheticNavigation, SurfaceBounds, SurfaceError, validate_scene_launch,
};
use raw_window_handle::{HasWindowHandle, RawWindowHandle};
use std::{
    marker::PhantomData,
    rc::Rc,
    thread::{self, ThreadId},
};
use wry::{
    NewWindowResponse, Rect, WebContext, WebView, WebViewBuilder,
    dpi::{LogicalPosition, LogicalSize},
};

const SYNTHETIC_PAGE: &str = "<!doctype html><meta http-equiv='Content-Security-Policy' content=\"default-src 'none'; base-uri 'none'; form-action 'none'; frame-src 'none'\"><title>Synthetic browser fixture</title><p>Wilde synthetic browser fixture</p><label>Browser typing fixture <input aria-label='Browser typing fixture'></label><textarea aria-label='Browser multiline fixture'></textarea>";

pub struct MainThreadOwner {
    thread: ThreadId,
    _local: PhantomData<Rc<()>>,
}
impl MainThreadOwner {
    /// # Safety
    /// Caller must own the application's main-thread native event loop on an isolated display.
    /// No native app may call this on the user's desktop; the attestation alone is not evidence.
    pub unsafe fn for_isolated_event_loop(isolated_display: bool) -> Result<Self, SurfaceError> {
        let background = std::env::var_os("ORCA_BACKGROUND_LAUNCH").is_some_and(|v| v == "1");
        validate_scene_launch(background, isolated_display)?;
        Ok(Self {
            thread: thread::current().id(),
            _local: PhantomData,
        })
    }
}
// Field order disposes the child before its context; the parent and owner remain borrowed.
pub struct WindowsChild<'parent, W: HasWindowHandle> {
    view: WebView,
    _context: WebContext,
    _profile: DevProfile,
    _parent: &'parent W,
    _owner: &'parent MainThreadOwner,
}
#[derive(Debug)]
pub enum ChildError {
    Policy(SurfaceError),
    Profile(crate::ProfileError),
    WrongThread,
    ParentHandle,
    Engine(wry::Error),
}
impl<'parent, W: HasWindowHandle> WindowsChild<'parent, W> {
    pub fn synthetic(
        parent: &'parent W,
        owner: &'parent MainThreadOwner,
        profile: DevProfile,
        bounds: SurfaceBounds,
    ) -> Result<Self, ChildError> {
        if owner.thread != thread::current().id() {
            return Err(ChildError::WrongThread);
        }
        let background = std::env::var_os("ORCA_BACKGROUND_LAUNCH").is_some_and(|v| v == "1");
        validate_scene_launch(background, true).map_err(ChildError::Policy)?;
        if !matches!(
            parent
                .window_handle()
                .map_err(|_| ChildError::ParentHandle)?
                .as_raw(),
            RawWindowHandle::Win32(_)
        ) {
            return Err(ChildError::ParentHandle);
        }
        profile.revalidate().map_err(ChildError::Profile)?;
        let mut context = WebContext::new(Some(profile.directory().to_path_buf()));
        let initial_navigation = InitialSyntheticNavigation::default();
        let view = WebViewBuilder::new_with_web_context(&mut context)
            .with_bounds(rect(bounds))
            .with_visible(false)
            .with_focused(false)
            .with_devtools(false)
            .with_clipboard(false)
            .with_navigation_handler(move |url| initial_navigation.allows(&url))
            .with_new_window_req_handler(|_, _| NewWindowResponse::Deny)
            .with_download_started_handler(|_, _| false)
            .with_html(SYNTHETIC_PAGE)
            .build_as_child(parent)
            .map_err(ChildError::Engine)?;
        Ok(Self {
            view,
            _context: context,
            _profile: profile,
            _parent: parent,
            _owner: owner,
        })
    }
    pub fn resize(&self, bounds: SurfaceBounds) -> Result<(), ChildError> {
        self.view
            .set_bounds(rect(bounds))
            .map_err(ChildError::Engine)
    }
    /// Visibility only for the already attested isolated main-thread scene; never activates the parent.
    pub fn set_scene_visible(&self, visible: bool) -> Result<(), ChildError> {
        if self._owner.thread != thread::current().id() {
            return Err(ChildError::WrongThread);
        }
        let background = std::env::var_os("ORCA_BACKGROUND_LAUNCH").is_some_and(|v| v == "1");
        validate_scene_launch(background, true).map_err(ChildError::Policy)?;
        self.view.set_visible(visible).map_err(ChildError::Engine)
    }
    pub fn navigate_website(&self, _url: &str) -> Result<(), SurfaceError> {
        Err(SurfaceError::PermissionsUnsupported)
    }
}
fn rect(bounds: SurfaceBounds) -> Rect {
    let (x, y, width, height) = bounds.logical_rect();
    Rect {
        position: LogicalPosition::new(x, y).into(),
        size: LogicalSize::new(width, height).into(),
    }
}
