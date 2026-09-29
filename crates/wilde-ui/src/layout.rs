#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Sidebar,
    NativeInput,
    Terminal,
    Browser,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct LogicalBounds {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl LogicalBounds {
    pub fn x(self) -> f64 {
        self.x
    }
    pub fn y(self) -> f64 {
        self.y
    }
    pub fn width(self) -> f64 {
        self.width
    }
    pub fn height(self) -> f64 {
        self.height
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LayoutError {
    InvalidScale,
    CoordinateOverflow,
}

#[derive(Debug, Clone, PartialEq)]
pub struct SceneLayout {
    width: f64,
    height: f64,
    surfaces: Option<[LogicalBounds; 4]>,
}

impl SceneLayout {
    pub fn from_physical(width: u32, height: u32, scale: f64) -> Result<Self, LayoutError> {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(LayoutError::InvalidScale);
        }
        let width = f64::from(width) / scale;
        let height = f64::from(height) / scale;
        if !width.is_finite()
            || !height.is_finite()
            || width > f64::from(i32::MAX)
            || height > f64::from(i32::MAX)
        {
            return Err(LayoutError::CoordinateOverflow);
        }
        // Suspend child surfaces before narrow bounds become unusable.
        let surfaces = if width < 64.0 || height < 64.0 {
            None
        } else {
            let sidebar = (width / 4.0).min(240.0);
            let content = width - sidebar;
            let input = (height / 4.0).min(32.0);
            let terminal = content / 2.0;
            Some([
                LogicalBounds {
                    x: 0.0,
                    y: 0.0,
                    width: sidebar,
                    height,
                },
                LogicalBounds {
                    x: sidebar,
                    y: 0.0,
                    width: content,
                    height: input,
                },
                LogicalBounds {
                    x: sidebar,
                    y: input,
                    width: terminal,
                    height: height - input,
                },
                LogicalBounds {
                    x: sidebar + terminal,
                    y: input,
                    width: content - terminal,
                    height: height - input,
                },
            ])
        };
        Ok(Self {
            width,
            height,
            surfaces,
        })
    }

    pub fn width(&self) -> f64 {
        self.width
    }
    pub fn height(&self) -> f64 {
        self.height
    }
    pub fn is_suspended(&self) -> bool {
        self.surfaces.is_none()
    }
    pub fn bounds(&self, region: Region) -> Option<LogicalBounds> {
        let index = match region {
            Region::Sidebar => 0,
            Region::NativeInput => 1,
            Region::Terminal => 2,
            Region::Browser => 3,
        };
        self.surfaces.map(|surfaces| surfaces[index])
    }
}
