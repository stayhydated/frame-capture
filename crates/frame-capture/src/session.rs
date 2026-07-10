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

    /// Creates a capture config after validating the output path.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureOutputPathError`] when `path` is empty, lacks a `.png`
    /// file name, or contains a non-Unicode file name.
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        CaptureItemSpec, CaptureItemVariant, CaptureRouteVariant, ParseRouteError,
        ParseScenarioError, RouteSpec,
    };

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum Route {
        Root,
    }

    impl CaptureRoute for Route {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[Self::Root];
        const VARIANTS: &'static [RouteSpec] =
            &[RouteSpec::new("root", "Root", PixelSize::new(10, 20))];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[CaptureRouteVariant {
            route: Self::Root,
            spec: Self::VARIANTS[0],
        }];

        fn spec(self) -> RouteSpec {
            Self::VARIANTS[0]
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            (value == "root")
                .then_some(Self::Root)
                .ok_or_else(|| ParseRouteError::new(value, ["root"]))
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

    fn config() -> CaptureConfig {
        CaptureConfig::try_new("capture.png", CaptureFrame::new(2), PixelSize::new(30, 40)).unwrap()
    }

    #[test]
    fn capture_config_exposes_and_consumes_validated_values() {
        let config = config();
        assert_eq!(config.path(), Path::new("capture.png"));
        assert_eq!(config.frame(), CaptureFrame::new(2));
        assert_eq!(config.size(), PixelSize::new(30, 40));
        assert_eq!(config.into_path_buf(), PathBuf::from("capture.png"));
        assert!(
            CaptureConfig::try_new("capture.jpg", CaptureFrame::new(1), PixelSize::new(1, 1))
                .is_err()
        );
    }

    #[test]
    fn capture_sessions_support_live_and_capture_consumption() {
        let live = CaptureSession::new(Route::Root, None);
        assert_eq!(live.route(), &Route::Root);
        assert!(!live.is_capture());
        assert_eq!(live.into_route(), Route::Root);

        let capture = CaptureSession::new(Route::Root, Some(config()));
        assert!(capture.is_capture());
        assert_eq!(capture.capture().unwrap().frame(), CaptureFrame::new(2));
        assert_eq!(
            capture.into_capture().unwrap().size(),
            PixelSize::new(30, 40)
        );
    }

    #[test]
    fn input_sessions_clone_config_and_split_into_parts() {
        assert_eq!(Route::Root.spec().id(), "root");
        assert_eq!(Route::from_id("root"), Ok(Route::Root));
        assert!(Route::from_id("missing").is_err());
        assert_eq!(Scenario::Loaded.id(), "loaded");
        assert_eq!(Scenario::from_id("loaded"), Ok(Scenario::Loaded));
        assert!(Scenario::from_id("missing").is_err());

        let session = CaptureInputSession::new(Route::Root, Some(config()), Some(Scenario::Loaded));
        assert_eq!(session.route(), Route::Root);
        assert!(session.is_capture());
        assert_eq!(session.scenario(), Some(Scenario::Loaded));
        assert_eq!(session.capture().unwrap().path(), Path::new("capture.png"));
        assert_eq!(
            session.capture_config().unwrap().frame(),
            CaptureFrame::new(2)
        );

        let (route, capture, scenario) = session.into_parts();
        assert_eq!(route, Route::Root);
        assert_eq!(capture.unwrap().size(), PixelSize::new(30, 40));
        assert_eq!(scenario, Some(Scenario::Loaded));
    }
}
