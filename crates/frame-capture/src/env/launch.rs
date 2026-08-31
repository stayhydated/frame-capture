use super::*;

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
