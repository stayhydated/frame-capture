use std::path::{Path, PathBuf};

use crate::{
    CaptureFrame, CaptureOutputPath, CaptureOutputPathError, CaptureRoute, CaptureScenario,
    NoCaptureScenario, PixelSize,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureConfig {
    path: CaptureOutputPath,
    frame: CaptureFrame,
    size: PixelSize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureSession<R> {
    route: R,
    capture: Option<CaptureConfig>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureInputSession<R, S = NoCaptureScenario> {
    route: R,
    capture: Option<CaptureConfig>,
    scenario: Option<S>,
}

impl CaptureConfig {
    pub fn new(path: CaptureOutputPath, frame: CaptureFrame, size: PixelSize) -> Self {
        Self { path, frame, size }
    }

    pub fn try_new(
        path: impl Into<PathBuf>,
        frame: CaptureFrame,
        size: PixelSize,
    ) -> Result<Self, CaptureOutputPathError> {
        Ok(Self::new(CaptureOutputPath::new(path)?, frame, size))
    }

    pub fn path(&self) -> &Path {
        self.path.as_path()
    }

    pub fn frame(&self) -> CaptureFrame {
        self.frame
    }

    pub fn size(&self) -> PixelSize {
        self.size
    }

    pub fn into_path_buf(self) -> PathBuf {
        self.path.into_path_buf()
    }
}

impl<R> CaptureSession<R> {
    pub fn new(route: R, capture: Option<CaptureConfig>) -> Self {
        Self { route, capture }
    }

    pub fn route(&self) -> &R {
        &self.route
    }

    pub fn into_route(self) -> R {
        self.route
    }

    pub fn capture(&self) -> Option<&CaptureConfig> {
        self.capture.as_ref()
    }

    pub fn into_capture(self) -> Option<CaptureConfig> {
        self.capture
    }

    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }
}

impl<R, S> CaptureInputSession<R, S>
where
    R: CaptureRoute,
    S: CaptureScenario,
{
    pub fn new(route: R, capture: Option<CaptureConfig>, scenario: Option<S>) -> Self {
        Self {
            route,
            capture,
            scenario,
        }
    }

    pub fn route(&self) -> R {
        self.route
    }

    pub fn capture(&self) -> Option<&CaptureConfig> {
        self.capture.as_ref()
    }

    pub fn capture_config(&self) -> Option<CaptureConfig> {
        self.capture.clone()
    }

    pub fn scenario(&self) -> Option<S> {
        self.scenario
    }

    pub fn is_capture(&self) -> bool {
        self.capture.is_some()
    }

    pub fn into_parts(self) -> (R, Option<CaptureConfig>, Option<S>) {
        (self.route, self.capture, self.scenario)
    }
}
