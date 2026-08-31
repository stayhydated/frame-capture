use super::*;

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
