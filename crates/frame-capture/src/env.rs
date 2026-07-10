use std::{
    borrow::Cow,
    collections::BTreeMap,
    env,
    ffi::{OsStr, OsString},
    fmt,
    num::NonZeroU32,
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::ids::{
    CaptureEnvVar, CaptureRouteId, ParseCaptureEnvVarError, ParseCaptureRouteIdError,
};
use crate::{
    CaptureConfig, CaptureInputSession, CaptureOutputPath, CaptureOutputPathError, CaptureRoute,
    CaptureScenario, CaptureScenarioId, CaptureSession, CaptureStateId, DEFAULT_CAPTURE_FRAME,
    ParseCaptureStateIdError, ParseRouteError, ParseScenarioError, PixelSize,
};

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CaptureFrame(NonZeroU32);

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureEnv {
    route_var: CaptureEnvVar,
    path_var: CaptureEnvVar,
    frame_var: CaptureEnvVar,
    width_var: CaptureEnvVar,
    height_var: CaptureEnvVar,
    scenario_var: CaptureEnvVar,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CaptureEnvBuilder {
    env: CaptureEnv,
}

/// Environment data for launching a host-owned capture route.
///
/// This type models only the shared `FRAME_CAPTURE_*` protocol. It does not
/// spawn a process, write screenshots, or carry facade-specific flags.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureLaunchEnv {
    env: CaptureEnv,
    route_id: CaptureRouteId,
    output_path: Option<CaptureOutputPath>,
    frame: Option<CaptureFrame>,
    size: Option<PixelSize>,
    scenario_id: Option<CaptureScenarioId>,
}

/// One environment variable emitted by [`CaptureLaunchEnv`].
///
/// Values are stored as [`OsString`] so command launchers can pass paths
/// without forcing a Unicode conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaptureLaunchEnvVar {
    name: String,
    value: OsString,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CaptureFrameGate {
    frame: u32,
    requested: bool,
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

impl CaptureFrame {
    /// Creates a positive capture frame number.
    ///
    /// # Panics
    ///
    /// Panics when `value` is zero. Use [`Self::try_new`] when zero is
    /// recoverable input.
    pub const fn new(value: u32) -> Self {
        match Self::try_new(value) {
            Some(frame) => frame,
            None => panic!("capture frame must be greater than zero"),
        }
    }

    pub const fn from_nonzero(value: NonZeroU32) -> Self {
        Self(value)
    }

    pub const fn try_new(value: u32) -> Option<Self> {
        let Some(value) = NonZeroU32::new(value) else {
            return None;
        };

        Some(Self(value))
    }

    pub const fn get(self) -> u32 {
        self.0.get()
    }

    pub const fn into_nonzero(self) -> NonZeroU32 {
        self.0
    }
}

impl fmt::Display for CaptureFrame {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(formatter)
    }
}

impl From<CaptureFrame> for NonZeroU32 {
    fn from(value: CaptureFrame) -> Self {
        value.into_nonzero()
    }
}

impl From<NonZeroU32> for CaptureFrame {
    fn from(value: NonZeroU32) -> Self {
        Self::from_nonzero(value)
    }
}

impl FromStr for CaptureFrame {
    type Err = ParseCaptureFrameError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let frame = value
            .parse::<u32>()
            .map_err(|_| ParseCaptureFrameError::Invalid {
                value: value.to_owned(),
            })?;

        Self::try_new(frame).ok_or(ParseCaptureFrameError::Zero)
    }
}

impl Serialize for CaptureFrame {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_u32(self.get())
    }
}

impl<'de> Deserialize<'de> for CaptureFrame {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let value = u32::deserialize(deserializer)?;
        Self::try_new(value).ok_or_else(|| de::Error::custom(ParseCaptureFrameError::Zero))
    }
}

impl CaptureEnv {
    pub fn frame_capture() -> Self {
        Self::new_unchecked(
            "FRAME_CAPTURE_ROUTE",
            "FRAME_CAPTURE_PATH",
            "FRAME_CAPTURE_FRAME",
            "FRAME_CAPTURE_WIDTH",
            "FRAME_CAPTURE_HEIGHT",
            "FRAME_CAPTURE_SCENARIO",
        )
    }

    pub fn builder() -> CaptureEnvBuilder {
        CaptureEnvBuilder::default()
    }

    /// Construct a capture env from a shared variable prefix.
    ///
    /// # Panics
    ///
    /// Panics when the prefix produces an invalid env var name. Use
    /// [`Self::try_with_prefix`] when invalid prefixes are recoverable input.
    pub fn with_prefix(prefix: impl AsRef<str>) -> Self {
        match Self::try_with_prefix(prefix) {
            Ok(env) => env,
            Err(error) => panic!("capture env prefix must produce valid env var names: {error}"),
        }
    }

    /// Construct a capture env from a shared variable prefix.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when any generated env var name is
    /// empty, contains `=`, or contains NUL.
    pub fn try_with_prefix(prefix: impl AsRef<str>) -> Result<Self, ParseCaptureEnvVarError> {
        Ok(Self::builder().prefix(prefix)?.build())
    }

    pub fn route_var(&self) -> &str {
        self.route_var.as_str()
    }

    pub fn path_var(&self) -> &str {
        self.path_var.as_str()
    }

    pub fn frame_var(&self) -> &str {
        self.frame_var.as_str()
    }

    pub fn width_var(&self) -> &str {
        self.width_var.as_str()
    }

    pub fn height_var(&self) -> &str {
        self.height_var.as_str()
    }

    pub fn scenario_var(&self) -> &str {
        self.scenario_var.as_str()
    }

    pub fn is_capture_requested(&self) -> bool {
        env::var_os(self.path_var()).is_some()
    }

    /// Reads the selected route from the route env var, or returns the default.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when the env var is not Unicode or does not
    /// parse as one of `R`'s route ids.
    pub fn read_route<R: CaptureRoute>(&self) -> Result<R, CaptureEnvError> {
        let Some(value) = read_env_string(&self.route_var)? else {
            return Ok(R::DEFAULT);
        };

        R::from_id(&value).map_err(|source| CaptureEnvError::InvalidRoute {
            var: self.route_var.to_string(),
            source,
        })
    }

    /// Reads a route id from the route env var, or returns `default`.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when the env var is not Unicode or is not a
    /// valid relative route id.
    pub fn read_route_id_or(
        &self,
        default: &CaptureRouteId,
    ) -> Result<(CaptureRouteId, CaptureRouteSource), CaptureEnvError> {
        let Some(value) = read_env_string(&self.route_var)? else {
            return Ok((default.clone(), CaptureRouteSource::Default));
        };

        CaptureRouteId::new(value)
            .map(|id| (id, CaptureRouteSource::Env))
            .map_err(|source| CaptureEnvError::InvalidRouteId {
                var: self.route_var.to_string(),
                source,
            })
    }

    /// Reads the optional selected scenario from the scenario env var.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when the env var is not Unicode or does not
    /// parse as one of `S`'s scenario ids.
    pub fn read_scenario<S: CaptureScenario>(&self) -> Result<Option<S>, CaptureEnvError> {
        let Some(value) = read_env_string(&self.scenario_var)? else {
            return Ok(None);
        };

        S::from_id(&value)
            .map(Some)
            .map_err(|source| CaptureEnvError::InvalidScenario {
                var: self.scenario_var.to_string(),
                source,
            })
    }

    /// Reads the optional selected scenario id from the scenario env var.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when the env var is not Unicode or is not a
    /// valid scenario id.
    pub fn read_scenario_id(&self) -> Result<Option<CaptureScenarioId>, CaptureEnvError> {
        self.read_optional_scenario_id(&self.scenario_var)
    }

    /// Reads capture output, frame, and size settings.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when capture env vars are not Unicode,
    /// contain invalid integers, define only one size dimension, use zero
    /// dimensions, or provide an invalid output path.
    pub fn read_capture(
        &self,
        default_size: PixelSize,
    ) -> Result<Option<CaptureConfig>, CaptureEnvError> {
        let Some(path) = env::var_os(self.path_var()) else {
            return Ok(None);
        };

        let frame = self.read_optional_frame(&self.frame_var, DEFAULT_CAPTURE_FRAME)?;
        let size = self.read_size(default_size)?;

        CaptureConfig::try_new(PathBuf::from(path), frame, size)
            .map(Some)
            .map_err(|source| CaptureEnvError::InvalidOutputPath {
                var: self.path_var.to_string(),
                source,
            })
    }

    /// Reads a typed route session from the capture environment.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route or capture env vars are invalid.
    pub fn read_session<R: CaptureRoute>(&self) -> Result<CaptureSession<R>, CaptureEnvError> {
        let route = self.read_route::<R>()?;
        let capture = self.read_capture(route.spec().default_size())?;

        Ok(CaptureSession::new(route, capture))
    }

    /// Reads a typed route and scenario input session from the capture environment.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route, scenario, or capture env vars are
    /// invalid.
    pub fn read_session_with_scenario<R, S>(
        &self,
    ) -> Result<CaptureInputSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario,
    {
        self.read_session_with_inputs::<R, S>()
    }

    /// Reads a typed route and scenario input session from the capture environment.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route, scenario, or capture env vars are
    /// invalid.
    pub fn read_session_with_inputs<R, S>(
        &self,
    ) -> Result<CaptureInputSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario,
    {
        let session = self.read_session::<R>()?;
        let scenario = self.read_scenario::<S>()?;
        let route = *session.route();
        let capture = session.into_capture();

        Ok(CaptureInputSession::new(route, capture, scenario))
    }

    fn read_size(&self, default_size: PixelSize) -> Result<PixelSize, CaptureEnvError> {
        match (
            read_env_string(&self.width_var)?,
            read_env_string(&self.height_var)?,
        ) {
            (Some(width), Some(height)) => Ok(PixelSize::new(
                parse_dimension(&self.width_var, &width)?,
                parse_dimension(&self.height_var, &height)?,
            )),
            (None, None) => Ok(default_size),
            _ => Err(CaptureEnvError::PartialSize {
                width_var: self.width_var.to_string(),
                height_var: self.height_var.to_string(),
            }),
        }
    }

    fn read_optional_frame(
        &self,
        var: &CaptureEnvVar,
        default: CaptureFrame,
    ) -> Result<CaptureFrame, CaptureEnvError> {
        let Some(value) = read_env_string(var)? else {
            return Ok(default);
        };

        let frame = value
            .parse::<u32>()
            .map_err(|_| CaptureEnvError::InvalidInteger {
                var: var.to_string(),
                value,
            })?;

        CaptureFrame::try_new(frame).ok_or_else(|| CaptureEnvError::ZeroDimension {
            var: var.to_string(),
        })
    }

    fn read_optional_state_id(
        &self,
        var: &CaptureEnvVar,
    ) -> Result<Option<CaptureStateId>, CaptureEnvError> {
        read_env_string(var)?
            .map(CaptureStateId::new)
            .transpose()
            .map_err(|source| CaptureEnvError::InvalidStateId {
                var: var.to_string(),
                source,
            })
    }

    fn read_optional_scenario_id(
        &self,
        var: &CaptureEnvVar,
    ) -> Result<Option<CaptureScenarioId>, CaptureEnvError> {
        self.read_optional_state_id(var)
            .map(|id| id.map(CaptureScenarioId::from))
    }

    fn new_unchecked(
        route_var: &'static str,
        path_var: &'static str,
        frame_var: &'static str,
        width_var: &'static str,
        height_var: &'static str,
        scenario_var: &'static str,
    ) -> Self {
        Self {
            route_var: CaptureEnvVar::new(route_var).expect("static env var must be valid"),
            path_var: CaptureEnvVar::new(path_var).expect("static env var must be valid"),
            frame_var: CaptureEnvVar::new(frame_var).expect("static env var must be valid"),
            width_var: CaptureEnvVar::new(width_var).expect("static env var must be valid"),
            height_var: CaptureEnvVar::new(height_var).expect("static env var must be valid"),
            scenario_var: CaptureEnvVar::new(scenario_var).expect("static env var must be valid"),
        }
    }
}

impl Default for CaptureEnv {
    fn default() -> Self {
        Self::frame_capture()
    }
}

#[allow(
    clippy::missing_errors_doc,
    reason = "bon generates public builder setter methods for fallible field conversions"
)]
#[bon::bon]
impl CaptureLaunchEnv {
    #[builder(
        start_fn(name = builder, vis = "pub"),
        finish_fn = build,
        builder_type(name = CaptureLaunchEnvBuilder, vis = "pub"),
        derive(Clone, Debug)
    )]
    fn from_parts(
        #[builder(with = |route_id: impl Into<String>| -> Result<_, CaptureLaunchEnvError> {
            CaptureRouteId::new(route_id)
                .map_err(|source| CaptureLaunchEnvError::InvalidRouteId { source })
        })]
        route_id: CaptureRouteId,
        #[builder(default = CaptureEnv::frame_capture())] env: CaptureEnv,
        #[builder(with = |output_path: impl Into<PathBuf>| -> Result<_, CaptureLaunchEnvError> {
            CaptureOutputPath::new(output_path)
                .map_err(|source| CaptureLaunchEnvError::InvalidOutputPath { source })
        })]
        output_path: Option<CaptureOutputPath>,
        #[builder(with = |frame: u32| -> Result<_, CaptureLaunchEnvError> {
            CaptureFrame::try_new(frame).ok_or(CaptureLaunchEnvError::InvalidFrame {
                source: ParseCaptureFrameError::Zero,
            })
        })]
        frame: Option<CaptureFrame>,
        #[builder(with = |width: u32, height: u32| -> Result<_, CaptureLaunchEnvError> {
            CaptureLaunchEnv::try_size(width, height)
        })]
        size: Option<PixelSize>,
        #[builder(with = |scenario_id: impl Into<String>| -> Result<_, CaptureLaunchEnvError> {
            CaptureScenarioId::new(scenario_id)
                .map_err(|source| CaptureLaunchEnvError::InvalidScenarioId { source })
        })]
        scenario_id: Option<CaptureScenarioId>,
    ) -> Self {
        Self {
            env,
            route_id,
            output_path,
            frame,
            size,
            scenario_id,
        }
    }

    /// Create a route-only launch environment from an already validated route id.
    pub fn new(route_id: CaptureRouteId) -> Self {
        Self {
            env: CaptureEnv::frame_capture(),
            route_id,
            output_path: None,
            frame: None,
            size: None,
            scenario_id: None,
        }
    }

    /// Parse a route id and create a route-only launch environment.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureLaunchEnvError`] when `route_id` is not a valid
    /// relative route id.
    pub fn try_new(route_id: impl Into<String>) -> Result<Self, CaptureLaunchEnvError> {
        Ok(Self::builder().route_id(route_id)?.build())
    }

    /// Environment variable names used by this launch environment.
    pub fn env(&self) -> &CaptureEnv {
        &self.env
    }

    /// Selected route id.
    pub fn route_id(&self) -> &CaptureRouteId {
        &self.route_id
    }

    /// Optional PNG output path. Presence requests capture mode.
    pub fn output_path(&self) -> Option<&CaptureOutputPath> {
        self.output_path.as_ref()
    }

    /// Optional capture frame override.
    pub fn frame(&self) -> Option<CaptureFrame> {
        self.frame
    }

    /// Optional capture size override.
    pub fn size(&self) -> Option<PixelSize> {
        self.size
    }

    /// Optional scenario id.
    pub fn scenario_id(&self) -> Option<&CaptureScenarioId> {
        self.scenario_id.as_ref()
    }

    /// Validate a complete width and height override.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureLaunchEnvError::InvalidSize`] when either dimension is
    /// zero.
    pub fn try_size(width: u32, height: u32) -> Result<PixelSize, CaptureLaunchEnvError> {
        PixelSize::try_new(width, height)
            .ok_or(CaptureLaunchEnvError::InvalidSize { width, height })
    }

    /// Validate an optional width and height override.
    ///
    /// Width and height must both be present and greater than zero, or both
    /// absent.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureLaunchEnvError::PartialSize`] when only one dimension
    /// is present, or [`CaptureLaunchEnvError::InvalidSize`] when either
    /// present dimension is zero.
    pub fn optional_size(
        width: Option<u32>,
        height: Option<u32>,
    ) -> Result<Option<(u32, u32)>, CaptureLaunchEnvError> {
        match (width, height) {
            (Some(width), Some(height)) => {
                Self::try_size(width, height).map(|_| Some((width, height)))
            },
            (None, None) => Ok(None),
            _ => Err(CaptureLaunchEnvError::PartialSize),
        }
    }

    /// Return the `FRAME_CAPTURE_*` variables in protocol order.
    pub fn vars(&self) -> Vec<CaptureLaunchEnvVar> {
        let mut vars = vec![CaptureLaunchEnvVar::new(
            self.env.route_var(),
            self.route_id.as_str(),
        )];

        if let Some(output_path) = &self.output_path {
            vars.push(CaptureLaunchEnvVar::new(
                self.env.path_var(),
                output_path.as_path().as_os_str(),
            ));
        }
        if let Some(frame) = self.frame {
            vars.push(CaptureLaunchEnvVar::new(
                self.env.frame_var(),
                frame.to_string(),
            ));
        }
        if let Some(size) = self.size {
            vars.push(CaptureLaunchEnvVar::new(
                self.env.width_var(),
                size.width().to_string(),
            ));
            vars.push(CaptureLaunchEnvVar::new(
                self.env.height_var(),
                size.height().to_string(),
            ));
        }
        if let Some(scenario_id) = &self.scenario_id {
            vars.push(CaptureLaunchEnvVar::new(
                self.env.scenario_var(),
                scenario_id.as_str(),
            ));
        }

        vars
    }

    /// Return the variables as a string map for JSON-oriented tools.
    ///
    /// Non-Unicode path bytes are converted with [`OsStr::to_string_lossy`].
    pub fn env_map_lossy(&self) -> BTreeMap<String, String> {
        self.vars()
            .into_iter()
            .map(|var| {
                let (name, value) = var.into_pair();
                (name, value.to_string_lossy().into_owned())
            })
            .collect()
    }
}

impl CaptureLaunchEnvVar {
    /// Create an environment variable from a name and OS string value.
    pub fn new(name: impl Into<String>, value: impl Into<OsString>) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }

    /// Environment variable name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Environment variable value.
    pub fn value(&self) -> &OsStr {
        &self.value
    }

    /// Return the value as a lossy Unicode string.
    pub fn value_string_lossy(&self) -> Cow<'_, str> {
        self.value.to_string_lossy()
    }

    /// Consume this variable into a name/value pair.
    pub fn into_pair(self) -> (String, OsString) {
        (self.name, self.value)
    }
}

impl CaptureEnvBuilder {
    /// Sets all capture env var names from a shared prefix.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when any generated env var name is
    /// empty, contains `=`, or contains NUL.
    pub fn prefix(mut self, prefix: impl AsRef<str>) -> Result<Self, ParseCaptureEnvVarError> {
        let prefix = prefix.as_ref();
        self.env.route_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_ROUTE"))?;
        self.env.path_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_PATH"))?;
        self.env.frame_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_FRAME"))?;
        self.env.width_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_WIDTH"))?;
        self.env.height_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_HEIGHT"))?;
        self.env.scenario_var = CaptureEnvVar::new(format!("{prefix}_CAPTURE_SCENARIO"))?;
        Ok(self)
    }

    /// Sets the route env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn route_var(mut self, value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.route_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    /// Sets the output path env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn path_var(mut self, value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.path_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    /// Sets the capture frame env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn frame_var(mut self, value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.frame_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    /// Sets the capture width env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn width_var(mut self, value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.width_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    /// Sets the capture height env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn height_var(mut self, value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.height_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    /// Sets the scenario env var name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is not a valid env var
    /// name.
    pub fn scenario_var(
        mut self,
        value: impl Into<String>,
    ) -> Result<Self, ParseCaptureEnvVarError> {
        self.env.scenario_var = CaptureEnvVar::new(value)?;
        Ok(self)
    }

    pub fn build(self) -> CaptureEnv {
        self.env
    }
}

impl CaptureFrameGate {
    pub fn frame(&self) -> u32 {
        self.frame
    }

    pub fn requested(&self) -> bool {
        self.requested
    }

    pub fn advance(&mut self) {
        if !self.requested {
            self.frame = self.frame.saturating_add(1);
        }
    }

    pub fn ready(&self, target_frame: CaptureFrame) -> bool {
        !self.requested && self.frame >= target_frame.get()
    }

    pub fn mark_requested(&mut self) {
        self.requested = true;
    }
}

fn read_env_string(var: &str) -> Result<Option<String>, CaptureEnvError> {
    match env::var(var) {
        Ok(value) => Ok(Some(value)),
        Err(env::VarError::NotPresent) => Ok(None),
        Err(env::VarError::NotUnicode(_)) => Err(CaptureEnvError::NotUnicode {
            var: var.to_owned(),
        }),
    }
}

fn parse_dimension(var: &str, value: &str) -> Result<u32, CaptureEnvError> {
    let dimension = value
        .parse::<u32>()
        .map_err(|_| CaptureEnvError::InvalidInteger {
            var: var.to_owned(),
            value: value.to_owned(),
        })?;

    if dimension == 0 {
        return Err(CaptureEnvError::ZeroDimension {
            var: var.to_owned(),
        });
    }

    Ok(dimension)
}

#[cfg(test)]
mod tests {
    use std::num::NonZeroU32;

    use super::*;
    use crate::{CaptureItemSpec, CaptureItemVariant, CaptureRouteVariant, RouteSpec};

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Route {
        Root,
        Review,
    }

    impl CaptureRoute for Route {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[Self::Root, Self::Review];
        const VARIANTS: &'static [RouteSpec] = &[
            RouteSpec::new("root", "Root", PixelSize::new(100, 100)),
            RouteSpec::new("review", "Review", PixelSize::new(200, 150)),
        ];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[
            CaptureRouteVariant {
                route: Self::Root,
                spec: Self::VARIANTS[0],
            },
            CaptureRouteVariant {
                route: Self::Review,
                spec: Self::VARIANTS[1],
            },
        ];

        fn spec(self) -> RouteSpec {
            match self {
                Self::Root => Self::VARIANTS[0],
                Self::Review => Self::VARIANTS[1],
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            match value {
                "root" => Ok(Self::Root),
                "review" => Ok(Self::Review),
                _ => Err(ParseRouteError::new(value, ["root", "review"])),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Scenario {
        Loaded,
    }

    impl CaptureScenario for Scenario {
        const SCENARIOS: &'static [Self] = &[Self::Loaded];
        const VARIANTS: &'static [&'static str] = &["loaded"];
        const SPECS: &'static [CaptureItemSpec] = &[CaptureItemSpec::new("loaded", "Loaded")];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[CaptureItemVariant {
            value: Self::Loaded,
            spec: Self::SPECS[0],
        }];

        fn id(self) -> &'static str {
            "loaded"
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            (value == "loaded")
                .then_some(Self::Loaded)
                .ok_or_else(|| ParseScenarioError::new(value, ["loaded"]))
        }
    }

    fn env(prefix: &str) -> CaptureEnv {
        CaptureEnv::try_with_prefix(prefix).unwrap()
    }

    fn clear(env: &CaptureEnv) {
        for var in [
            env.route_var(),
            env.path_var(),
            env.frame_var(),
            env.width_var(),
            env.height_var(),
            env.scenario_var(),
        ] {
            unsafe { std::env::remove_var(var) };
        }
    }

    #[test]
    fn capture_frames_cover_nonzero_parse_display_and_serde_contracts() {
        let nonzero = NonZeroU32::new(3).unwrap();
        let frame = CaptureFrame::from_nonzero(nonzero);
        assert_eq!(frame.get(), 3);
        assert_eq!(frame.into_nonzero(), nonzero);
        assert_eq!(CaptureFrame::from(nonzero), frame);
        assert_eq!(NonZeroU32::from(frame), nonzero);
        assert_eq!(frame.to_string(), "3");
        assert_eq!("3".parse::<CaptureFrame>().unwrap(), frame);
        assert!(matches!(
            "three".parse::<CaptureFrame>(),
            Err(ParseCaptureFrameError::Invalid { .. })
        ));
        assert_eq!(
            "0".parse::<CaptureFrame>(),
            Err(ParseCaptureFrameError::Zero)
        );
        assert_eq!(serde_json::to_string(&frame).unwrap(), "3");
        assert!(serde_json::from_str::<CaptureFrame>("0").is_err());
        assert!(std::panic::catch_unwind(|| CaptureFrame::new(0)).is_err());
    }

    #[test]
    fn capture_env_builder_sets_every_protocol_name() {
        let env = CaptureEnv::builder()
            .route_var("APP_ROUTE")
            .unwrap()
            .path_var("APP_PATH")
            .unwrap()
            .frame_var("APP_FRAME")
            .unwrap()
            .width_var("APP_WIDTH")
            .unwrap()
            .height_var("APP_HEIGHT")
            .unwrap()
            .scenario_var("APP_SCENARIO")
            .unwrap()
            .build();
        assert_eq!(
            [
                env.route_var(),
                env.path_var(),
                env.frame_var(),
                env.width_var(),
                env.height_var(),
                env.scenario_var()
            ],
            [
                "APP_ROUTE",
                "APP_PATH",
                "APP_FRAME",
                "APP_WIDTH",
                "APP_HEIGHT",
                "APP_SCENARIO"
            ]
        );
        assert_eq!(
            CaptureEnv::with_prefix("APP2").route_var(),
            "APP2_CAPTURE_ROUTE"
        );
        assert_eq!(CaptureEnv::default(), CaptureEnv::frame_capture());
        assert!(std::panic::catch_unwind(|| CaptureEnv::with_prefix("BAD=APP")).is_err());
    }

    #[test]
    fn capture_env_reads_route_and_scenario_values_and_errors() {
        let env = env("FRAME_CAPTURE_ENV_IDS_TEST");
        clear(&env);
        assert_eq!(Route::Root.spec().id(), "root");
        assert_eq!(Scenario::Loaded.id(), "loaded");
        assert_eq!(env.read_route::<Route>().unwrap(), Route::Root);
        assert_eq!(env.read_scenario::<Scenario>().unwrap(), None);
        assert_eq!(env.read_scenario_id().unwrap(), None);

        unsafe { std::env::set_var(env.route_var(), "review") };
        assert_eq!(env.read_route::<Route>().unwrap(), Route::Review);
        let default = CaptureRouteId::new("root").unwrap();
        assert_eq!(
            env.read_route_id_or(&default).unwrap().1,
            CaptureRouteSource::Env
        );

        unsafe { std::env::set_var(env.route_var(), "missing") };
        assert!(matches!(
            env.read_route::<Route>(),
            Err(CaptureEnvError::InvalidRoute { .. })
        ));
        unsafe { std::env::set_var(env.route_var(), "../missing") };
        assert!(matches!(
            env.read_route_id_or(&default),
            Err(CaptureEnvError::InvalidRouteId { .. })
        ));

        unsafe { std::env::set_var(env.scenario_var(), "loaded") };
        assert_eq!(
            env.read_scenario::<Scenario>().unwrap(),
            Some(Scenario::Loaded)
        );
        assert_eq!(env.read_scenario_id().unwrap().unwrap().as_str(), "loaded");
        unsafe { std::env::set_var(env.scenario_var(), "missing") };
        assert!(matches!(
            env.read_scenario::<Scenario>(),
            Err(CaptureEnvError::InvalidScenario { .. })
        ));
        unsafe { std::env::set_var(env.scenario_var(), "states/loaded") };
        assert!(matches!(
            env.read_scenario_id(),
            Err(CaptureEnvError::InvalidStateId { .. })
        ));
        clear(&env);
    }

    #[test]
    fn capture_env_reads_capture_defaults_overrides_and_errors() {
        let env = env("FRAME_CAPTURE_ENV_CONFIG_TEST");
        clear(&env);
        let default_size = PixelSize::new(640, 480);
        assert!(!env.is_capture_requested());
        assert_eq!(env.read_capture(default_size).unwrap(), None);

        unsafe { std::env::set_var(env.path_var(), "capture.png") };
        assert!(env.is_capture_requested());
        let capture = env.read_capture(default_size).unwrap().unwrap();
        assert_eq!(capture.frame(), DEFAULT_CAPTURE_FRAME);
        assert_eq!(capture.size(), default_size);

        unsafe { std::env::set_var(env.frame_var(), "bad") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::InvalidInteger { .. })
        ));
        unsafe { std::env::set_var(env.frame_var(), "0") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::ZeroDimension { .. })
        ));
        unsafe { std::env::remove_var(env.frame_var()) };

        unsafe { std::env::set_var(env.width_var(), "320") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::PartialSize { .. })
        ));
        unsafe { std::env::set_var(env.height_var(), "bad") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::InvalidInteger { .. })
        ));
        unsafe { std::env::set_var(env.height_var(), "0") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::ZeroDimension { .. })
        ));
        unsafe { std::env::set_var(env.height_var(), "240") };
        assert_eq!(
            env.read_capture(default_size).unwrap().unwrap().size(),
            PixelSize::new(320, 240)
        );

        unsafe { std::env::set_var(env.path_var(), "capture.jpg") };
        assert!(matches!(
            env.read_capture(default_size),
            Err(CaptureEnvError::InvalidOutputPath { .. })
        ));
        clear(&env);
    }

    #[test]
    fn capture_env_session_alias_carries_typed_inputs() {
        let env = env("FRAME_CAPTURE_ENV_SESSION_ALIAS_TEST");
        clear(&env);
        unsafe {
            std::env::set_var(env.route_var(), "review");
            std::env::set_var(env.scenario_var(), "loaded");
        }
        let session = env.read_session_with_scenario::<Route, Scenario>().unwrap();
        assert_eq!(session.route(), Route::Review);
        assert_eq!(session.scenario(), Some(Scenario::Loaded));
        assert!(!session.is_capture());
        clear(&env);
    }

    #[cfg(unix)]
    #[test]
    fn capture_env_rejects_non_unicode_string_values() {
        use std::{ffi::OsString, os::unix::ffi::OsStringExt as _};

        let env = env("FRAME_CAPTURE_ENV_UNICODE_TEST");
        clear(&env);
        unsafe { std::env::set_var(env.route_var(), OsString::from_vec(vec![0xff])) };
        assert!(matches!(
            env.read_route::<Route>(),
            Err(CaptureEnvError::NotUnicode { .. })
        ));
        clear(&env);
    }

    #[test]
    fn launch_env_constructors_and_vars_cover_route_only_requests() {
        let route = CaptureRouteId::new("review").unwrap();
        let launch = CaptureLaunchEnv::new(route.clone());
        assert_eq!(launch.env(), &CaptureEnv::frame_capture());
        assert_eq!(launch.route_id(), &route);
        assert_eq!(launch.output_path(), None);
        assert_eq!(launch.frame(), None);
        assert_eq!(launch.size(), None);
        assert_eq!(launch.scenario_id(), None);
        assert_eq!(launch.vars().len(), 1);
        assert_eq!(CaptureLaunchEnv::try_new("review").unwrap(), launch);
        assert!(CaptureLaunchEnv::try_new("../review").is_err());
        assert_eq!(CaptureLaunchEnv::optional_size(None, None), Ok(None));

        let var = CaptureLaunchEnvVar::new("APP_ROUTE", "review");
        assert_eq!(var.name(), "APP_ROUTE");
        assert_eq!(var.value(), OsStr::new("review"));
        assert_eq!(
            var.into_pair(),
            ("APP_ROUTE".to_owned(), OsString::from("review"))
        );
    }

    #[test]
    fn frame_gate_advances_once_and_latches_requests() {
        let mut gate = CaptureFrameGate::default();
        assert_eq!(gate.frame(), 0);
        assert!(!gate.requested());
        assert!(!gate.ready(CaptureFrame::new(2)));
        gate.advance();
        gate.advance();
        assert!(gate.ready(CaptureFrame::new(2)));
        gate.mark_requested();
        gate.advance();
        assert_eq!(gate.frame(), 2);
        assert!(gate.requested());
        assert!(!gate.ready(CaptureFrame::new(2)));
    }
}
