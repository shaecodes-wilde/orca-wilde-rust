//! Windows composition experiment; execution requires an independently isolated display.
use anyhow::{Result, bail, ensure};
use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt,
    path::PathBuf,
    time::{Duration, Instant},
};
use tao::{
    dpi::LogicalSize,
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    platform::{run_return::EventLoopExtRunReturn, windows::WindowExtWindows},
    window::WindowBuilder,
};
use wilde_browser::{
    DevProfile, SurfaceBounds,
    windows_child::{MainThreadOwner, WindowsChild},
};
use wilde_ui::{
    launch::{DEVELOPMENT_APPLICATION_ID, LaunchRequest, admit},
    layout::{Region, SceneLayout},
};
use windows_sys::Win32::{
    Foundation::HWND,
    UI::WindowsAndMessaging::{
        CreateWindowExW, DestroyWindow, ES_AUTOVSCROLL, ES_MULTILINE, ES_READONLY, MoveWindow,
        SetWindowTextW, WS_BORDER, WS_CHILD, WS_TABSTOP, WS_VISIBLE,
    },
};

fn wide(value: &str) -> Vec<u16> {
    OsStr::new(value).encode_wide().chain(Some(0)).collect()
}
struct NativeControl(HWND);
impl NativeControl {
    fn new(parent: HWND, class: &str, text: &str, extra: u32) -> Result<Self> {
        let class = wide(class);
        let text = wide(text);
        // SAFETY: Parent is the borrowed live Tao window; strings remain alive through the call.
        let handle = unsafe {
            CreateWindowExW(
                0,
                class.as_ptr(),
                text.as_ptr(),
                WS_CHILD | WS_VISIBLE | extra,
                0,
                0,
                1,
                1,
                parent,
                std::ptr::null_mut(),
                std::ptr::null_mut(),
                std::ptr::null(),
            )
        };
        ensure!(
            !handle.is_null(),
            "native control creation failed: {}",
            std::io::Error::last_os_error()
        );
        Ok(Self(handle))
    }
    fn text(&self, text: &str) -> Result<()> {
        let text = wide(text);
        // SAFETY: This owner keeps its HWND live; Win32 copies the UTF16 text synchronously.
        ensure!(
            unsafe { SetWindowTextW(self.0, text.as_ptr()) } != 0,
            "native text update failed"
        );
        Ok(())
    }
    fn place(&self, bounds: Option<wilde_ui::layout::LogicalBounds>, scale: f64) -> Result<()> {
        let (x, y, width, height) = bounds.map_or((0, 0, 0, 0), |b| {
            (
                (b.x() * scale).round() as i32,
                (b.y() * scale).round() as i32,
                (b.width() * scale).round() as i32,
                (b.height() * scale).round() as i32,
            )
        });
        // SAFETY: Coordinates are bounded by SceneLayout and this live HWND is privately owned.
        ensure!(
            unsafe { MoveWindow(self.0, x, y, width, height, 1) } != 0,
            "native layout failed"
        );
        Ok(())
    }
}
impl Drop for NativeControl {
    fn drop(&mut self) {
        // SAFETY: Each control is destroyed once before its borrowed parent leaves this function.
        unsafe {
            DestroyWindow(self.0);
        }
    }
}
fn browser_bounds(layout: &SceneLayout, scale: f64) -> Result<SurfaceBounds> {
    let b = layout
        .bounds(Region::Browser)
        .ok_or_else(|| anyhow::anyhow!("browser surface suspended"))?;
    SurfaceBounds::within(
        b.x(),
        b.y(),
        b.width(),
        b.height(),
        scale,
        layout.width(),
        layout.height(),
    )
    .map_err(|e| anyhow::anyhow!("browser bounds: {e:?}"))
}

pub fn run(args: &[String]) -> Result<()> {
    ensure!(
        args.len() == 5
            && args[0] == "--isolated-display"
            && args[1] == "--profile-root"
            && args[3] == "--profile-id",
        "explicit isolated display, profile root and profile id required"
    );
    let root = PathBuf::from(&args[2]);
    let background = std::env::var("ORCA_BACKGROUND_LAUNCH").ok();
    let isolated = std::env::var("WILDE_NATIVE_ISOLATED_DISPLAY").ok();
    let permit = admit(LaunchRequest {
        background_launch: background.as_deref(),
        isolated_display_flag: true,
        isolated_display_env: isolated.as_deref(),
        dev_profile_root: Some(&root),
        application_id: DEVELOPMENT_APPLICATION_ID,
    })
    .map_err(|e| anyhow::anyhow!("native launch denied: {e:?}"))?;
    let profile = DevProfile::inspect(permit.dev_profile_root(), &args[4])
        .map_err(|e| anyhow::anyhow!("synthetic browser profile denied: {e:?}"))?;
    // The runtime separately requires the reviewed owned-job watchdog, before creating a child.
    let identity = wilde_desktop::fixture_identity()?;
    let mut terminal = wilde_ide::TerminalModel::new_session(identity.clone(), 24, 80, 100)
        .map_err(|e| anyhow::anyhow!("terminal model: {e:?}"))?;
    let mut fixture = wilde_runtime::OwnedFixtureSession::spawn(
        identity,
        wilde_runtime::PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        },
    )?;
    let mut event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Wilde Native Rust - isolated synthetic experiment")
        .with_inner_size(LogicalSize::new(1000.0, 640.0))
        .with_visible(false)
        .build(&event_loop)?;
    // SAFETY: This function owns the Tao main-thread event loop; operational isolation must be verified externally.
    let owner = unsafe { MainThreadOwner::for_isolated_event_loop(true) }
        .map_err(|e| anyhow::anyhow!("browser owner: {e:?}"))?;
    let parent = window.hwnd() as HWND;
    let sidebar = NativeControl::new(
        parent,
        "STATIC",
        "Wilde Native Rust\r\n\r\nDemo Solar\r\nChecklist Demo\r\n\r\nSynthetic data only",
        0,
    )?;
    let input = NativeControl::new(
        parent,
        "EDIT",
        "Native typing and IME fixture",
        WS_BORDER | WS_TABSTOP,
    )?;
    let output = NativeControl::new(
        parent,
        "EDIT",
        "Waiting for owned terminal fixture",
        WS_BORDER | WS_TABSTOP | ES_MULTILINE as u32 | ES_READONLY as u32 | ES_AUTOVSCROLL as u32,
    )?;
    let size = window.inner_size();
    let scale = window.scale_factor();
    let initial = SceneLayout::from_physical(size.width, size.height, scale)
        .map_err(|e| anyhow::anyhow!("layout: {e:?}"))?;
    let browser =
        WindowsChild::synthetic(&window, &owner, profile, browser_bounds(&initial, scale)?)
            .map_err(|e| anyhow::anyhow!("browser create: {e:?}"))?;
    let layout_surfaces = || -> Result<()> {
        let size = window.inner_size();
        let scale = window.scale_factor();
        let layout = SceneLayout::from_physical(size.width, size.height, scale)
            .map_err(|e| anyhow::anyhow!("layout: {e:?}"))?;
        sidebar.place(layout.bounds(Region::Sidebar), scale)?;
        input.place(layout.bounds(Region::NativeInput), scale)?;
        output.place(layout.bounds(Region::Terminal), scale)?;
        browser
            .set_scene_visible(!layout.is_suspended())
            .map_err(|e| anyhow::anyhow!("browser visibility: {e:?}"))?;
        if !layout.is_suspended() {
            browser
                .resize(browser_bounds(&layout, scale)?)
                .map_err(|e| anyhow::anyhow!("browser resize: {e:?}"))?;
        }
        Ok(())
    };
    layout_surfaces()?;
    window.set_visible(true);
    let mut failure = None;
    let mut ping = false;
    let mut finish = false;
    let started = Instant::now();
    event_loop.run_return(|event, _, flow| {
        *flow = ControlFlow::WaitUntil(Instant::now() + Duration::from_millis(50));
        let outcome = (|| -> Result<()> {
            match event {
                Event::WindowEvent {
                    event: WindowEvent::CloseRequested,
                    ..
                } => *flow = ControlFlow::Exit,
                Event::WindowEvent {
                    event: WindowEvent::Resized(_) | WindowEvent::ScaleFactorChanged { .. },
                    ..
                } => layout_surfaces()?,
                Event::MainEventsCleared => {
                    if let wilde_runtime::OutputEvent::Frame(frame) =
                        fixture.read_output(Duration::ZERO)?
                    {
                        terminal
                            .accept_frame(&frame)
                            .map_err(|e| anyhow::anyhow!("terminal frame: {e:?}"))?;
                        let text = terminal.contents();
                        output.text(&text.replace('\n', "\r\n"))?;
                        if text.contains("WILDE_FIXTURE_READY") && !ping {
                            fixture.write_input(wilde_runtime::FixtureInput::Ping)?;
                            ping = true;
                        }
                        if text.contains("WILDE_FIXTURE_PONG") && !finish {
                            fixture.write_input(wilde_runtime::FixtureInput::Finish)?;
                            finish = true;
                        }
                    }
                    if started.elapsed() > Duration::from_secs(30) {
                        *flow = ControlFlow::Exit;
                    }
                }
                _ => {}
            }
            Ok(())
        })();
        if let Err(error) = outcome {
            failure = Some(error);
            *flow = ControlFlow::Exit;
        }
    });
    drop(browser);
    drop(output);
    drop(input);
    drop(sidebar);
    drop(owner);
    drop(window);
    let cleanup = fixture.close(Duration::from_secs(2));
    if let Some(error) = failure {
        return Err(error);
    }
    if !cleanup.reader_joined || !cleanup.lifecycle_joined || !cleanup.cwd_removed {
        bail!("fixture cleanup incomplete: {cleanup:?}");
    }
    Ok(())
}
