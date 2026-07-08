use std::ops::Deref;

use bevy::{
    app::{App, PluginGroup},
    prelude::*,
    state::state::FreelyMutableState,
    window::{Window, WindowPlugin, WindowResolution},
};
use frame_capture::{
    CaptureConfig, CaptureEnv, CaptureEnvError, CaptureInputSession, CaptureRoute, CaptureScenario,
    NoCaptureScenario, PixelSize,
};

use crate::{
    BevyCaptureAppExt as _, RouteResourcePlugin, RouteStatePlugin, ScenarioResourcePlugin,
    ScenarioStatePlugin, SelectedStatePlugin, capture_window_plugin,
};

#[derive(Clone, Debug, Resource)]
pub struct BevyCaptureConfig {
    capture: CaptureConfig,
}

/// Bevy-oriented capture session with typed route and optional scenario input.
///
/// Use `CaptureEnv::read_bevy_session` when an app does not support scenario ids.
/// Use `read_bevy_session_with_scenario` or `read_bevy_session_with_inputs` when the
/// selected scenario should seed app state or resources.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BevyCaptureSession<R, S = NoCaptureScenario> {
    route: R,
    capture: Option<CaptureConfig>,
    scenario: Option<S>,
}

pub trait BevyCaptureEnvExt {
    /// Reads a Bevy capture session without scenario support.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route or capture env vars are invalid,
    /// or when a scenario env var is present for this scenario-free session.
    fn read_bevy_session<R>(&self) -> Result<BevyCaptureSession<R>, CaptureEnvError>
    where
        R: CaptureRoute;

    /// Reads a Bevy capture session with typed scenario support.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route, scenario, or capture env vars are
    /// invalid.
    fn read_bevy_session_with_scenario<R, S>(
        &self,
    ) -> Result<BevyCaptureSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario;

    /// Reads a Bevy capture session with typed route and scenario inputs.
    ///
    /// # Errors
    ///
    /// Returns [`CaptureEnvError`] when route, scenario, or capture env vars are
    /// invalid.
    fn read_bevy_session_with_inputs<R, S>(
        &self,
    ) -> Result<BevyCaptureSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario;
}

impl BevyCaptureConfig {
    pub fn new(capture: CaptureConfig) -> Self {
        Self { capture }
    }

    pub fn capture(&self) -> &CaptureConfig {
        &self.capture
    }
}

impl<R, S> BevyCaptureSession<R, S>
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

    pub fn window_size(&self) -> PixelSize {
        self.capture
            .as_ref()
            .map(CaptureConfig::size)
            .unwrap_or_else(|| self.route.spec().default_size())
    }

    /// Returns the capture size in capture mode, otherwise `live_size`.
    ///
    /// Use this when the live app has its own window-size override instead of
    /// using the route's default capture size.
    pub fn window_size_with_live_size(&self, live_size: PixelSize) -> PixelSize {
        self.capture
            .as_ref()
            .map(CaptureConfig::size)
            .unwrap_or(live_size)
    }

    pub fn window_resolution(&self) -> WindowResolution {
        let size = self.window_size();
        WindowResolution::new(size.width(), size.height()).with_scale_factor_override(1.0)
    }

    /// Returns a deterministic Bevy window resolution for a live-size override.
    ///
    /// Capture mode still uses the requested capture size. Live mode uses
    /// `live_size`.
    pub fn window_resolution_with_live_size(&self, live_size: PixelSize) -> WindowResolution {
        let size = self.window_size_with_live_size(live_size);
        WindowResolution::new(size.width(), size.height()).with_scale_factor_override(1.0)
    }

    pub fn capture_window_plugin(&self, live_window: Window) -> WindowPlugin {
        capture_window_plugin(self.capture(), live_window)
    }

    pub fn add_capture_plugins<'a, P: PluginGroup>(
        &self,
        app: &'a mut App,
        plugins: P,
    ) -> &'a mut App {
        app.add_capture_plugins(plugins, self.capture_config())
    }

    pub fn add_selected_resources<'a>(&self, app: &'a mut App) -> &'a mut App
    where
        R: Send + Sync,
        S: Send + Sync,
    {
        app.add_plugins(RouteResourcePlugin::new(self.route));
        app.add_plugins(ScenarioResourcePlugin::new(self.scenario));
        app
    }

    pub fn add_route_state<'a>(&self, app: &'a mut App) -> &'a mut App
    where
        R: FreelyMutableState,
    {
        app.add_plugins(RouteStatePlugin::new(self.route));
        app
    }

    pub fn add_scenario_state<'a>(&self, app: &'a mut App, default_state: S) -> &'a mut App
    where
        S: FreelyMutableState,
    {
        app.add_plugins(ScenarioStatePlugin::new(self.scenario, default_state));
        app
    }

    /// Maps the selected route into an app-owned Bevy `State`.
    ///
    /// Use this when the route enum is target-neutral but the app has its own
    /// state type for schedules or run conditions.
    pub fn add_mapped_route_state<'a, T>(
        &self,
        app: &'a mut App,
        map: impl FnOnce(R) -> T,
    ) -> &'a mut App
    where
        T: FreelyMutableState,
    {
        app.add_plugins(SelectedStatePlugin::new(map(self.route)));
        app
    }

    /// Maps the optional selected scenario into an app-owned Bevy `State`.
    pub fn add_mapped_scenario_state<'a, T>(
        &self,
        app: &'a mut App,
        map: impl FnOnce(Option<S>) -> T,
    ) -> &'a mut App
    where
        T: FreelyMutableState,
    {
        app.add_plugins(SelectedStatePlugin::new(map(self.scenario)));
        app
    }

    /// Maps the selected route into an app-owned Bevy resource.
    pub fn insert_mapped_route_resource<'a, T>(
        &self,
        app: &'a mut App,
        map: impl FnOnce(R) -> T,
    ) -> &'a mut App
    where
        T: Resource,
    {
        app.insert_resource(map(self.route));
        app
    }

    /// Maps the optional selected scenario into an app-owned Bevy resource.
    pub fn insert_mapped_scenario_resource<'a, T>(
        &self,
        app: &'a mut App,
        map: impl FnOnce(Option<S>) -> T,
    ) -> &'a mut App
    where
        T: Resource,
    {
        app.insert_resource(map(self.scenario));
        app
    }

    pub fn into_parts(self) -> (R, Option<CaptureConfig>, Option<S>) {
        (self.route, self.capture, self.scenario)
    }
}

impl BevyCaptureEnvExt for CaptureEnv {
    fn read_bevy_session<R>(&self) -> Result<BevyCaptureSession<R>, CaptureEnvError>
    where
        R: CaptureRoute,
    {
        self.read_bevy_session_with_inputs::<R, NoCaptureScenario>()
    }

    fn read_bevy_session_with_scenario<R, S>(
        &self,
    ) -> Result<BevyCaptureSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario,
    {
        self.read_bevy_session_with_inputs::<R, S>()
    }

    fn read_bevy_session_with_inputs<R, S>(
        &self,
    ) -> Result<BevyCaptureSession<R, S>, CaptureEnvError>
    where
        R: CaptureRoute,
        S: CaptureScenario,
    {
        Ok(self.read_session_with_inputs::<R, S>()?.into())
    }
}

impl Deref for BevyCaptureConfig {
    type Target = CaptureConfig;

    fn deref(&self) -> &Self::Target {
        &self.capture
    }
}

impl From<CaptureConfig> for BevyCaptureConfig {
    fn from(capture: CaptureConfig) -> Self {
        Self::new(capture)
    }
}

impl<R, S> From<CaptureInputSession<R, S>> for BevyCaptureSession<R, S>
where
    R: CaptureRoute,
    S: CaptureScenario,
{
    fn from(session: CaptureInputSession<R, S>) -> Self {
        let (route, capture, scenario) = session.into_parts();
        Self::new(route, capture, scenario)
    }
}
