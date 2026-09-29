#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SurfaceBounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
    scale: f64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SurfaceError {
    InvalidBounds,
    BackgroundLaunch,
    IsolationRequired,
    UnsupportedPlatform,
    PermissionsUnsupported,
}
impl SurfaceBounds {
    pub fn within(
        x: f64,
        y: f64,
        width: f64,
        height: f64,
        scale: f64,
        parent_width: f64,
        parent_height: f64,
    ) -> Result<Self, SurfaceError> {
        if ![x, y, width, height, scale, parent_width, parent_height]
            .iter()
            .all(|v| v.is_finite())
            || x < 0.0
            || y < 0.0
            || width <= 0.0
            || height <= 0.0
            || scale <= 0.0
            || x + width > parent_width
            || y + height > parent_height
            || (x + width) * scale > f64::from(i32::MAX)
            || (y + height) * scale > f64::from(i32::MAX)
        {
            return Err(SurfaceError::InvalidBounds);
        }
        Ok(Self {
            x,
            y,
            width,
            height,
            scale,
        })
    }
    pub fn logical_rect(&self) -> (f64, f64, f64, f64) {
        (self.x, self.y, self.width, self.height)
    }
    pub fn scale(&self) -> f64 {
        self.scale
    }
}
/// Pure validation; a caller attestation never proves that a desktop is isolated.
pub fn validate_scene_launch(background: bool, isolated_display: bool) -> Result<(), SurfaceError> {
    if background {
        return Err(SurfaceError::BackgroundLaunch);
    }
    if !isolated_display {
        return Err(SurfaceError::IsolationRequired);
    }
    Ok(())
}
/// Generic raw-handle Linux embedding is intentionally unavailable, including X11 in this pilot.
pub fn non_windows_embedding() -> Result<(), SurfaceError> {
    Err(SurfaceError::UnsupportedPlatform)
}
