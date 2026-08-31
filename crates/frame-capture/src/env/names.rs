use super::*;

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
