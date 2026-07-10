//! Bevy facade for `frame-capture`.
//!
//! In capture mode, the facade configures Bevy without a primary window, runs
//! on `ScheduleRunnerPlugin`, redirects cameras to an offscreen image, saves
//! that image through Bevy's screenshot API, then emits `AppExit::Success`.

mod plugins;
mod runtime;
mod save;
mod session;
mod window;

pub use frame_capture::{
    CaptureConfig, CaptureEnv, CaptureEnvBuilder, CaptureEnvError, CaptureEnvVar, CaptureFrame,
    CaptureFrameGate, CaptureItemSpec, CaptureItemVariant, CaptureLaunchEnv,
    CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar, CaptureOutputName,
    CaptureOutputPath, CaptureOutputRoot, CaptureOutputStem, CapturePixelSizeInfo, CaptureRoute,
    CaptureRouteCatalog, CaptureRouteId, CaptureRouteIdRef, CaptureRouteInfo, CaptureRouteSource,
    CaptureRouteVariant, CaptureScenario, CaptureScenarioId, CaptureScenarioIdRef, CaptureSession,
    CaptureStateId, CaptureStateIdRef, NoCaptureScenario, ParseCaptureEnvVarError,
    ParseCaptureFrameError, ParseCaptureRouteIdError, ParseCaptureStateIdError, ParseRouteError,
    ParseScenarioError, PixelSize, RouteSpec, capture_output_path_for_name,
    capture_output_path_for_stem, capture_route_catalog, capture_route_infos,
};
#[cfg(feature = "registry")]
pub use frame_capture_macros::bevy_capture_route as capture_route;
#[cfg(feature = "macros")]
pub use frame_capture_macros::{CaptureRouteBevy, CaptureScenarioBevy};
#[cfg(feature = "registry")]
pub use frame_capture_routes_bevy::{
    BevyCaptureRegistryEnvExt, RegisteredCaptureSession, RegisteredRoute, RegisteredRouteError,
    RegisteredRouteKey, registered_route, registered_route_for_key, registered_routes,
    validate_registered_routes,
};

pub use self::{
    plugins::{
        RoutePlugin, RouteResourcePlugin, RouteStatePlugin, ScenarioPlugin, ScenarioResourcePlugin,
        ScenarioStatePlugin, SelectedCaptureRoute, SelectedCaptureScenario, SelectedStatePlugin,
    },
    runtime::{BevyCaptureAppExt, CapturePlugin, CaptureReady, CaptureWarmupPlugin},
    save::CaptureSaveError,
    session::{BevyCaptureConfig, BevyCaptureEnvExt, BevyCaptureSession},
    window::{capture_window_plugin, is_capture_enabled},
};

#[doc(hidden)]
pub mod __private {
    #[cfg(feature = "registry")]
    pub use frame_capture_routes_bevy::__private::inventory;
}

pub mod prelude {
    pub use crate::{
        BevyCaptureAppExt, BevyCaptureConfig, BevyCaptureEnvExt, BevyCaptureSession, CaptureConfig,
        CaptureEnv, CaptureEnvBuilder, CaptureEnvError, CaptureEnvVar, CaptureFrame,
        CaptureFrameGate, CaptureItemSpec, CaptureItemVariant, CaptureLaunchEnv,
        CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar, CaptureOutputName,
        CaptureOutputPath, CaptureOutputRoot, CaptureOutputStem, CapturePixelSizeInfo,
        CapturePlugin, CaptureReady, CaptureRoute, CaptureRouteCatalog, CaptureRouteId,
        CaptureRouteIdRef, CaptureRouteInfo, CaptureRouteSource, CaptureRouteVariant,
        CaptureScenario, CaptureScenarioId, CaptureScenarioIdRef, CaptureSession, CaptureStateId,
        CaptureStateIdRef, CaptureWarmupPlugin, NoCaptureScenario, ParseCaptureEnvVarError,
        ParseCaptureFrameError, ParseRouteError, PixelSize, RoutePlugin, RouteResourcePlugin,
        RouteSpec, RouteStatePlugin, ScenarioPlugin, ScenarioResourcePlugin, ScenarioStatePlugin,
        SelectedCaptureRoute, SelectedCaptureScenario, SelectedStatePlugin,
        capture_output_path_for_name, capture_output_path_for_stem, capture_route_catalog,
        capture_route_infos, capture_window_plugin, is_capture_enabled,
    };

    #[cfg(feature = "registry")]
    pub use crate::{
        BevyCaptureRegistryEnvExt, RegisteredCaptureSession, RegisteredRoute, RegisteredRouteKey,
        capture_route, registered_route, registered_route_for_key, registered_routes,
        validate_registered_routes,
    };
}

pub const DEFAULT_CAPTURE_FPS: f64 = 60.0;

#[cfg(test)]
mod tests {
    use bevy::prelude::*;

    use super::*;

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
    enum TestScenario {
        Empty,
        Loaded,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
    enum TestRouteState {
        Dashboard,
        Detail,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
    enum TestSelectedState {
        Review,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
    enum TestMappedRouteState {
        DashboardPage,
        DetailPage,
    }

    #[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, States)]
    enum TestMappedScenarioState {
        EmptyPreset,
        LoadedPreset,
    }

    #[derive(Resource)]
    struct InstalledScenario(Option<TestScenario>);

    #[derive(Resource)]
    struct MappedRouteResource(&'static str);

    #[derive(Resource)]
    struct MappedScenarioResource(bool);

    impl CaptureRoute for TestRouteState {
        const DEFAULT: Self = Self::Dashboard;
        const ROUTES: &'static [Self] = &[Self::Dashboard, Self::Detail];
        const VARIANTS: &'static [RouteSpec] = &[
            RouteSpec::new("dashboard", "Dashboard", PixelSize::new(640, 480)),
            RouteSpec::new("detail", "Detail", PixelSize::new(640, 480)),
        ];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[
            CaptureRouteVariant {
                route: Self::Dashboard,
                spec: Self::VARIANTS[0],
            },
            CaptureRouteVariant {
                route: Self::Detail,
                spec: Self::VARIANTS[1],
            },
        ];

        fn spec(self) -> RouteSpec {
            match self {
                Self::Dashboard => Self::VARIANTS[0],
                Self::Detail => Self::VARIANTS[1],
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            match value {
                "dashboard" => Ok(Self::Dashboard),
                "detail" => Ok(Self::Detail),
                _ => Err(ParseRouteError::new(value, ["dashboard", "detail"])),
            }
        }
    }

    impl CaptureScenario for TestScenario {
        const SCENARIOS: &'static [Self] = &[Self::Empty, Self::Loaded];
        const VARIANTS: &'static [&'static str] = &["empty", "loaded"];
        const SPECS: &'static [CaptureItemSpec] = &[
            CaptureItemSpec::new("empty", "Empty"),
            CaptureItemSpec::new("loaded", "Loaded"),
        ];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[
            CaptureItemVariant {
                value: Self::Empty,
                spec: Self::SPECS[0],
            },
            CaptureItemVariant {
                value: Self::Loaded,
                spec: Self::SPECS[1],
            },
        ];

        fn id(self) -> &'static str {
            match self {
                Self::Empty => "empty",
                Self::Loaded => "loaded",
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            match value {
                "empty" => Ok(Self::Empty),
                "loaded" => Ok(Self::Loaded),
                _ => Err(ParseScenarioError::new(
                    value,
                    Self::VARIANTS.iter().copied(),
                )),
            }
        }
    }

    #[test]
    fn route_and_scenario_plugins_install_selected_capture_state() {
        let mut app = App::new();
        app.add_plugins(RoutePlugin::new(
            TestRouteState::Detail,
            |route: TestRouteState, app: &mut App| {
                app.insert_resource(MappedRouteResource(route.spec().title()));
            },
        ));
        app.add_plugins(ScenarioPlugin::new(
            Some(TestScenario::Loaded),
            |scenario, app: &mut App| {
                app.insert_resource(InstalledScenario(scenario));
            },
        ));

        assert_eq!(
            app.world().resource::<InstalledScenario>().0,
            Some(TestScenario::Loaded)
        );
        assert_eq!(app.world().resource::<MappedRouteResource>().0, "Detail");
    }

    #[test]
    fn resource_plugins_insert_selected_capture_inputs() {
        let mut app = App::new();
        app.add_plugins(RouteResourcePlugin::new(TestRouteState::Detail));
        app.add_plugins(ScenarioResourcePlugin::new(Some(TestScenario::Loaded)));

        let route = *app
            .world()
            .resource::<SelectedCaptureRoute<TestRouteState>>();
        let scenario = *app
            .world()
            .resource::<SelectedCaptureScenario<TestScenario>>();

        assert_eq!(route.route(), TestRouteState::Detail);
        assert_eq!(route.id(), "detail");
        assert_eq!(route.spec().title(), "Detail");
        assert_eq!(scenario.scenario(), Some(TestScenario::Loaded));
        assert_eq!(
            scenario.unwrap_or(TestScenario::Empty),
            TestScenario::Loaded
        );
    }

    #[test]
    fn scenario_resource_keeps_live_default_explicit() {
        let mut app = App::new();
        app.add_plugins(ScenarioResourcePlugin::<TestScenario>::new(None));

        let scenario = *app
            .world()
            .resource::<SelectedCaptureScenario<TestScenario>>();

        assert_eq!(scenario.scenario(), None);
        assert_eq!(scenario.unwrap_or(TestScenario::Empty), TestScenario::Empty);
    }

    #[test]
    fn plain_bevy_session_rejects_unsupported_scenario_env() {
        let env = CaptureEnv::builder()
            .route_var("FRAME_CAPTURE_BEVY_PLAIN_SCENARIO_ROUTE_TEST")
            .unwrap()
            .path_var("FRAME_CAPTURE_BEVY_PLAIN_SCENARIO_PATH_TEST")
            .unwrap()
            .scenario_var("FRAME_CAPTURE_BEVY_PLAIN_SCENARIO_SCENARIO_TEST")
            .unwrap()
            .build();

        unsafe {
            std::env::set_var(env.scenario_var(), "loaded");
        }
        let error = env.read_bevy_session::<TestRouteState>().unwrap_err();
        unsafe {
            std::env::remove_var(env.scenario_var());
        }

        assert!(matches!(error, CaptureEnvError::InvalidScenario { .. }));
    }

    #[test]
    fn bevy_capture_session_uses_capture_size_for_window_resolution() {
        let session = BevyCaptureSession::new(
            TestRouteState::Detail,
            Some(
                CaptureConfig::try_new(
                    "capture.png",
                    CaptureFrame::new(3),
                    PixelSize::new(1200, 800),
                )
                .unwrap(),
            ),
            Some(TestScenario::Loaded),
        );

        assert_eq!(session.route(), TestRouteState::Detail);
        assert_eq!(session.scenario(), Some(TestScenario::Loaded));
        assert!(session.is_capture());
        assert_eq!(session.window_size(), PixelSize::new(1200, 800));
        assert_eq!(
            session.window_size_with_live_size(PixelSize::new(900, 600)),
            PixelSize::new(1200, 800),
        );

        let resolution = session.window_resolution();
        assert_eq!(resolution.width(), 1200.0);
        assert_eq!(resolution.height(), 800.0);
        assert_eq!(resolution.scale_factor_override(), Some(1.0));

        let window_plugin = session.capture_window_plugin(Window::default());
        assert!(window_plugin.primary_window.is_none());
    }

    #[test]
    fn bevy_capture_session_uses_route_size_for_live_window_resolution() {
        let session = BevyCaptureSession::<TestRouteState>::new(TestRouteState::Detail, None, None);

        assert!(!session.is_capture());
        assert_eq!(session.window_size(), PixelSize::new(640, 480));
        assert_eq!(
            session.window_size_with_live_size(PixelSize::new(900, 600)),
            PixelSize::new(900, 600),
        );
        let resolution = session.window_resolution_with_live_size(PixelSize::new(900, 600));
        assert_eq!(resolution.width(), 900.0);
        assert_eq!(resolution.height(), 600.0);
        assert_eq!(resolution.scale_factor_override(), Some(1.0));

        let window_plugin = session.capture_window_plugin(Window::default());
        assert!(window_plugin.primary_window.is_some());
    }

    #[test]
    fn bevy_capture_session_installs_selected_resources() {
        let session =
            BevyCaptureSession::new(TestRouteState::Detail, None, Some(TestScenario::Loaded));
        let mut app = App::new();

        session.add_selected_resources(&mut app);

        let route = *app
            .world()
            .resource::<SelectedCaptureRoute<TestRouteState>>();
        let scenario = *app
            .world()
            .resource::<SelectedCaptureScenario<TestScenario>>();

        assert_eq!(route.route(), TestRouteState::Detail);
        assert_eq!(scenario.scenario(), Some(TestScenario::Loaded));
    }

    #[test]
    fn bevy_capture_session_installs_route_and_scenario_states() {
        let session =
            BevyCaptureSession::new(TestRouteState::Detail, None, Some(TestScenario::Loaded));
        let mut app = App::new();

        session.add_route_state(&mut app);
        session.add_scenario_state(&mut app, TestScenario::Empty);

        assert_eq!(
            *app.world().resource::<State<TestRouteState>>().get(),
            TestRouteState::Detail,
        );
        assert_eq!(
            *app.world().resource::<State<TestScenario>>().get(),
            TestScenario::Loaded,
        );
    }

    #[test]
    fn selected_state_plugin_inserts_bevy_state() {
        let mut app = App::new();
        app.add_plugins(SelectedStatePlugin::new(TestSelectedState::Review));

        assert_eq!(
            *app.world().resource::<State<TestSelectedState>>().get(),
            TestSelectedState::Review,
        );
    }

    #[test]
    fn bevy_capture_session_installs_mapped_states() {
        let session =
            BevyCaptureSession::new(TestRouteState::Detail, None, Some(TestScenario::Loaded));
        let mut app = App::new();

        session.add_mapped_route_state(&mut app, |route| match route {
            TestRouteState::Dashboard => TestMappedRouteState::DashboardPage,
            TestRouteState::Detail => TestMappedRouteState::DetailPage,
        });
        session.add_mapped_scenario_state(&mut app, |scenario| match scenario {
            Some(TestScenario::Loaded) => TestMappedScenarioState::LoadedPreset,
            Some(TestScenario::Empty) | None => TestMappedScenarioState::EmptyPreset,
        });
        assert_eq!(
            *app.world().resource::<State<TestMappedRouteState>>().get(),
            TestMappedRouteState::DetailPage,
        );
        assert_eq!(
            *app.world()
                .resource::<State<TestMappedScenarioState>>()
                .get(),
            TestMappedScenarioState::LoadedPreset,
        );
    }

    #[test]
    fn bevy_capture_session_inserts_mapped_resources() {
        let session =
            BevyCaptureSession::new(TestRouteState::Detail, None, Some(TestScenario::Loaded));
        let mut app = App::new();

        session.insert_mapped_route_resource(&mut app, |route| {
            MappedRouteResource(route.spec().title())
        });
        session.insert_mapped_scenario_resource(&mut app, |scenario| {
            MappedScenarioResource(matches!(scenario, Some(TestScenario::Loaded)))
        });

        assert_eq!(app.world().resource::<MappedRouteResource>().0, "Detail");
        assert!(app.world().resource::<MappedScenarioResource>().0);
    }

    #[test]
    fn route_state_plugin_inserts_selected_route_as_bevy_state() {
        let mut app = App::new();
        app.add_plugins(RouteStatePlugin::new(TestRouteState::Detail));

        assert_eq!(
            *app.world().resource::<State<TestRouteState>>().get(),
            TestRouteState::Detail,
        );
    }

    #[test]
    fn scenario_state_plugin_inserts_selected_or_default_state() {
        let mut app = App::new();
        app.add_plugins(ScenarioStatePlugin::new(
            Some(TestScenario::Loaded),
            TestScenario::Empty,
        ));

        assert_eq!(
            *app.world().resource::<State<TestScenario>>().get(),
            TestScenario::Loaded,
        );

        let mut app = App::new();
        app.add_plugins(ScenarioStatePlugin::new(None, TestScenario::Empty));

        assert_eq!(
            *app.world().resource::<State<TestScenario>>().get(),
            TestScenario::Empty,
        );
    }

    #[test]
    fn capture_warmup_marks_ready_after_requested_frames() {
        let mut app = App::new();
        app.insert_resource(BevyCaptureConfig::new(
            CaptureConfig::try_new("capture.png", CaptureFrame::new(1), PixelSize::new(10, 10))
                .unwrap(),
        ));
        app.add_plugins(CaptureWarmupPlugin::frames(2));

        assert_eq!(
            *app.world().resource::<CaptureReady>(),
            CaptureReady::Pending
        );

        app.update();
        assert_eq!(
            *app.world().resource::<CaptureReady>(),
            CaptureReady::Pending
        );

        app.update();
        assert_eq!(*app.world().resource::<CaptureReady>(), CaptureReady::Ready);
    }

    #[test]
    fn capture_warmup_marks_ready_without_capture_runtime() {
        let mut app = App::new();
        app.add_plugins(CaptureWarmupPlugin::frames(1));

        assert_eq!(
            *app.world().resource::<CaptureReady>(),
            CaptureReady::Pending
        );

        app.update();
        assert_eq!(*app.world().resource::<CaptureReady>(), CaptureReady::Ready);
    }

    #[test]
    fn capture_ready_and_warmup_values_expose_manual_control() {
        let mut ready = CaptureReady::default();
        assert!(ready.is_ready());
        ready.mark_pending();
        assert!(!ready.is_ready());
        ready.mark_ready();
        assert_eq!(ready, CaptureReady::ready());

        assert_eq!(CaptureWarmupPlugin::new(3).frame_count(), 3);
        assert_eq!(CaptureWarmupPlugin::frames(4).frame_count(), 4);
    }

    #[test]
    fn bevy_capture_config_and_session_support_consuming_accessors() {
        let capture =
            CaptureConfig::try_new("capture.png", CaptureFrame::new(2), PixelSize::new(30, 20))
                .unwrap();
        let config = BevyCaptureConfig::from(capture.clone());
        assert_eq!(config.capture(), &capture);
        assert_eq!(config.frame(), CaptureFrame::new(2));

        let session = BevyCaptureSession::new(
            TestRouteState::Detail,
            Some(capture),
            Some(TestScenario::Loaded),
        );
        assert_eq!(
            session.capture_config().unwrap().size(),
            PixelSize::new(30, 20)
        );
        let (route, capture, scenario) = session.into_parts();
        assert_eq!(route, TestRouteState::Detail);
        assert_eq!(capture.unwrap().frame(), CaptureFrame::new(2));
        assert_eq!(scenario, Some(TestScenario::Loaded));
    }

    #[test]
    fn plugin_value_accessors_expose_selected_inputs() {
        let selected = SelectedCaptureRoute::new(TestRouteState::Detail);
        assert_eq!(*selected, TestRouteState::Detail);
        assert_eq!(selected.route(), TestRouteState::Detail);

        let route_state = RouteStatePlugin::new(TestRouteState::Detail);
        assert_eq!(route_state.route(), TestRouteState::Detail);
        let scenario_state = ScenarioStatePlugin::new(None, TestScenario::Empty);
        assert_eq!(scenario_state.selected_state(), TestScenario::Empty);
        assert_eq!(
            ScenarioStatePlugin::new(Some(TestScenario::Loaded), TestScenario::Empty)
                .selected_state(),
            TestScenario::Loaded
        );
    }

    #[test]
    fn capture_window_helpers_and_live_runtime_are_explicit() {
        let capture =
            CaptureConfig::try_new("capture.png", CaptureFrame::new(1), PixelSize::new(10, 10))
                .unwrap();
        assert!(is_capture_enabled(Some(&capture)));
        assert!(!is_capture_enabled(None));

        let mut app = App::new();
        app.add_capture_runtime(None);
        app.add_capture_plugins(MinimalPlugins, None);
        assert!(!app.world().contains_resource::<BevyCaptureConfig>());
    }

    #[test]
    fn bevy_env_reads_selected_route_and_scenario() {
        assert!(TestRouteState::from_id("missing").is_err());
        assert_eq!(TestScenario::Empty.id(), "empty");
        assert_eq!(TestScenario::Loaded.id(), "loaded");
        assert_eq!(TestScenario::from_id("empty"), Ok(TestScenario::Empty));
        assert!(TestScenario::from_id("missing").is_err());

        let env = CaptureEnv::with_prefix("FRAME_CAPTURE_BEVY_INPUT_TEST");
        unsafe {
            std::env::set_var(env.route_var(), "detail");
            std::env::set_var(env.scenario_var(), "loaded");
        }
        let session = env
            .read_bevy_session_with_inputs::<TestRouteState, TestScenario>()
            .unwrap();
        assert_eq!(session.route(), TestRouteState::Detail);
        assert_eq!(session.scenario(), Some(TestScenario::Loaded));
        unsafe {
            std::env::remove_var(env.route_var());
            std::env::remove_var(env.scenario_var());
        }
    }
}
