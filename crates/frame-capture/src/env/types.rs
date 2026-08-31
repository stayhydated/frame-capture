use super::*;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CaptureFrame(pub(super) NonZeroU32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureEnv {
    pub(super) route_var: CaptureEnvVar,
    pub(super) path_var: CaptureEnvVar,
    pub(super) frame_var: CaptureEnvVar,
    pub(super) width_var: CaptureEnvVar,
    pub(super) height_var: CaptureEnvVar,
    pub(super) scenario_var: CaptureEnvVar,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CaptureEnvBuilder {
    pub(super) env: CaptureEnv,
}

/// Environment data for launching a host-owned capture route.
///
/// This type models only the shared `FRAME_CAPTURE_*` protocol. It does not
/// spawn a process, write screenshots, or carry facade-specific flags.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureLaunchEnv {
    pub(super) env: CaptureEnv,
    pub(super) route_id: CaptureRouteId,
    pub(super) output_path: Option<CaptureOutputPath>,
    pub(super) frame: Option<CaptureFrame>,
    pub(super) size: Option<PixelSize>,
    pub(super) scenario_id: Option<CaptureScenarioId>,
}

/// One environment variable emitted by [`CaptureLaunchEnv`].
///
/// Values are stored as [`OsString`] so command launchers can pass paths
/// without forcing a Unicode conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureLaunchEnvVar {
    pub(super) name: String,
    pub(super) value: OsString,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CaptureFrameGate {
    pub(super) frame: u32,
    pub(super) requested: bool,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum CaptureRouteSource {
    Default,
    Env,
}

/// Error returned while adapting raw launch-env request data into typed values.
#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CaptureLaunchEnvError {
    #[error("{source}")]
    InvalidRouteId { source: ParseCaptureRouteIdError },
    #[error("{source}")]
    InvalidOutputPath { source: CaptureOutputPathError },
    #[error("{source}")]
    InvalidFrame { source: ParseCaptureFrameError },
    #[error("set both capture width and height, or neither")]
    PartialSize,
    #[error("capture width and height must be greater than zero")]
    InvalidSize { width: u32, height: u32 },
    #[error("{source}")]
    InvalidScenarioId { source: ParseCaptureStateIdError },
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum CaptureEnvError {
    #[error("{var} must be valid Unicode")]
    NotUnicode { var: String },
    #[error("{var}: {source}")]
    InvalidRouteId {
        var: String,
        source: ParseCaptureRouteIdError,
    },
    #[error("{var}: {source}")]
    InvalidRoute {
        var: String,
        source: ParseRouteError,
    },
    #[error("{var}: {source}")]
    InvalidScenario {
        var: String,
        source: ParseScenarioError,
    },
    #[error("{var}: {source}")]
    InvalidStateId {
        var: String,
        source: ParseCaptureStateIdError,
    },
    #[error("{var}: {source}")]
    InvalidOutputPath {
        var: String,
        source: CaptureOutputPathError,
    },
    #[error("{var} must be a positive integer, got `{value}`")]
    InvalidInteger { var: String, value: String },
    #[error("{var} must be greater than zero")]
    ZeroDimension { var: String },
    #[error("set both {width_var} and {height_var}, or neither")]
    PartialSize {
        width_var: String,
        height_var: String,
    },
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseCaptureFrameError {
    #[error("capture frame must be a positive integer, got `{value}`")]
    Invalid { value: String },
    #[error("capture frame must be greater than zero")]
    Zero,
}
