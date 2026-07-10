//! Target-neutral capture wiring for wgpu-backed applications.
//!
//! This crate intentionally stops at the shared protocol: route specs, capture
//! dimensions, environment parsing, output paths, and frame gating. Rendering
//! facades such as `frame-capture-bevy` own the framework-specific texture and
//! screenshot mechanics.

mod env;
mod ids;
mod output;
mod session;
mod size;
mod spec;
mod traits;
mod validation;

#[cfg(feature = "macros")]
pub use frame_capture_macros::{CaptureRoute, CaptureScenario};

pub use self::{
    env::{
        CaptureEnv, CaptureEnvBuilder, CaptureEnvError, CaptureFrame, CaptureFrameGate,
        CaptureLaunchEnv, CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar,
        CaptureRouteSource, ParseCaptureFrameError,
    },
    ids::{
        CaptureEnvVar, CaptureRouteId, CaptureRouteIdRef, CaptureScenarioId, CaptureScenarioIdRef,
        CaptureStateId, CaptureStateIdRef, ParseCaptureEnvVarError, ParseCaptureRouteIdError,
        ParseCaptureStateIdError,
    },
    output::{
        CaptureOutputName, CaptureOutputPath, CaptureOutputPathError, CaptureOutputRoot,
        CaptureOutputStem, capture_output_path_for_name, capture_output_path_for_stem,
    },
    session::{CaptureConfig, CaptureInputSession, CaptureSession},
    size::{CapturePixelSizeInfo, ParsePixelSizeError, PixelSize},
    spec::{
        CaptureItemSpec, CaptureRouteCatalog, CaptureRouteInfo, RouteSpec, capture_route_catalog,
        capture_route_infos,
    },
    traits::{
        CaptureItemVariant, CaptureRoute, CaptureRouteVariant, CaptureScenario, NoCaptureScenario,
        ParseRouteError, ParseScenarioError,
    },
    validation::{
        CaptureCatalogValidationError, validate_capture_routes, validate_capture_scenarios,
    },
};

pub mod prelude {
    pub use crate::{
        CaptureConfig, CaptureEnv, CaptureEnvBuilder, CaptureEnvVar, CaptureFrame,
        CaptureFrameGate, CaptureInputSession, CaptureItemSpec, CaptureItemVariant,
        CaptureLaunchEnv, CaptureLaunchEnvBuilder, CaptureLaunchEnvError, CaptureLaunchEnvVar,
        CaptureOutputName, CaptureOutputPath, CaptureOutputRoot, CaptureOutputStem,
        CapturePixelSizeInfo, CaptureRoute, CaptureRouteCatalog, CaptureRouteId, CaptureRouteIdRef,
        CaptureRouteInfo, CaptureRouteVariant, CaptureScenario, CaptureScenarioId,
        CaptureScenarioIdRef, CaptureSession, CaptureStateId, CaptureStateIdRef, NoCaptureScenario,
        PixelSize, RouteSpec, capture_output_path_for_name, capture_output_path_for_stem,
        capture_route_catalog, capture_route_infos, validate_capture_routes,
        validate_capture_scenarios,
    };
}

pub const DEFAULT_CAPTURE_FRAME: CaptureFrame = CaptureFrame::new(12);

#[cfg(test)]
mod tests {
    use std::{
        collections::BTreeMap,
        num::NonZeroU32,
        path::{Path, PathBuf},
    };

    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestRoute {
        Root,
        Settings,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum TestScenario {
        LedOn,
        LedOff,
    }

    impl CaptureRoute for TestRoute {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[Self::Root, Self::Settings];
        const VARIANTS: &'static [RouteSpec] = &[
            RouteSpec::new("root", "Root", PixelSize::new(1920, 1080)),
            RouteSpec::new("settings", "Settings", PixelSize::new(800, 600)),
        ];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[
            CaptureRouteVariant {
                route: Self::Root,
                spec: Self::VARIANTS[0],
            },
            CaptureRouteVariant {
                route: Self::Settings,
                spec: Self::VARIANTS[1],
            },
        ];

        fn spec(self) -> RouteSpec {
            match self {
                Self::Root => Self::VARIANTS[0],
                Self::Settings => Self::VARIANTS[1],
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            match value {
                "root" => Ok(Self::Root),
                "settings" => Ok(Self::Settings),
                _ => Err(ParseRouteError::new(value, ["root", "settings"])),
            }
        }
    }

    impl CaptureScenario for TestScenario {
        const SCENARIOS: &'static [Self] = &[Self::LedOn, Self::LedOff];
        const VARIANTS: &'static [&'static str] = &["led-on", "led-off"];
        const SPECS: &'static [CaptureItemSpec] = &[
            CaptureItemSpec::new("led-on", "LED On"),
            CaptureItemSpec::new("led-off", "LED Off"),
        ];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[
            CaptureItemVariant {
                value: Self::LedOn,
                spec: Self::SPECS[0],
            },
            CaptureItemVariant {
                value: Self::LedOff,
                spec: Self::SPECS[1],
            },
        ];

        fn id(self) -> &'static str {
            match self {
                Self::LedOn => "led-on",
                Self::LedOff => "led-off",
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            match value {
                "led-on" => Ok(Self::LedOn),
                "led-off" => Ok(Self::LedOff),
                _ => Err(ParseScenarioError::new(
                    value,
                    Self::VARIANTS.iter().copied(),
                )),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum MissingDefaultRoute {
        Root,
        Settings,
    }

    impl CaptureRoute for MissingDefaultRoute {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[Self::Settings];
        const VARIANTS: &'static [RouteSpec] = &[
            RouteSpec::new("root", "Root", PixelSize::new(100, 100)),
            RouteSpec::new("settings", "Settings", PixelSize::new(100, 100)),
        ];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[
            CaptureRouteVariant {
                route: Self::Root,
                spec: Self::VARIANTS[0],
            },
            CaptureRouteVariant {
                route: Self::Settings,
                spec: Self::VARIANTS[1],
            },
        ];

        fn spec(self) -> RouteSpec {
            match self {
                Self::Root => Self::VARIANTS[0],
                Self::Settings => Self::VARIANTS[1],
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            match value {
                "root" => Ok(Self::Root),
                "settings" => Ok(Self::Settings),
                _ => Err(ParseRouteError::new(value, ["root", "settings"])),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum DuplicateScenario {
        A,
        B,
    }

    impl CaptureScenario for DuplicateScenario {
        const SCENARIOS: &'static [Self] = &[Self::A, Self::B];
        const VARIANTS: &'static [&'static str] = &["same", "same"];
        const SPECS: &'static [CaptureItemSpec] = &[
            CaptureItemSpec::new("same", "Same A"),
            CaptureItemSpec::new("same", "Same B"),
        ];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[
            CaptureItemVariant {
                value: Self::A,
                spec: Self::SPECS[0],
            },
            CaptureItemVariant {
                value: Self::B,
                spec: Self::SPECS[1],
            },
        ];

        fn id(self) -> &'static str {
            "same"
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            match value {
                "same" => Ok(Self::A),
                _ => Err(ParseScenarioError::new(value, ["same"])),
            }
        }
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum MissingRouteMetadata {
        Root,
        Settings,
    }

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum MissingScenarioMetadata {
        LedOn,
        LedOff,
    }

    impl CaptureRoute for MissingRouteMetadata {
        const DEFAULT: Self = Self::Root;
        const ROUTES: &'static [Self] = &[Self::Root, Self::Settings];
        const VARIANTS: &'static [RouteSpec] = &[
            RouteSpec::new("root", "Root", PixelSize::new(100, 100)),
            RouteSpec::new("settings", "Settings", PixelSize::new(100, 100)),
        ];
        const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = &[CaptureRouteVariant {
            route: Self::Root,
            spec: Self::VARIANTS[0],
        }];

        fn spec(self) -> RouteSpec {
            match self {
                Self::Root => Self::VARIANTS[0],
                Self::Settings => Self::VARIANTS[1],
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseRouteError> {
            match value {
                "root" => Ok(Self::Root),
                "settings" => Ok(Self::Settings),
                _ => Err(ParseRouteError::new(value, ["root", "settings"])),
            }
        }
    }

    impl CaptureScenario for MissingScenarioMetadata {
        const SCENARIOS: &'static [Self] = &[Self::LedOn, Self::LedOff];
        const VARIANTS: &'static [&'static str] = &["led-on", "led-off"];
        const SPECS: &'static [CaptureItemSpec] = &[CaptureItemSpec::new("led-on", "LED On")];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[CaptureItemVariant {
            value: Self::LedOn,
            spec: Self::SPECS[0],
        }];

        fn id(self) -> &'static str {
            match self {
                Self::LedOn => "led-on",
                Self::LedOff => "led-off",
            }
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            match value {
                "led-on" => Ok(Self::LedOn),
                "led-off" => Ok(Self::LedOff),
                _ => Err(ParseScenarioError::new(value, ["led-on", "led-off"])),
            }
        }
    }

    #[test]
    fn scales_from_long_edge_with_rounding() {
        assert_eq!(
            PixelSize::from_long_edge(PixelSize::new(1920, 1080), NonZeroU32::new(1600).unwrap()),
            PixelSize::new(1600, 900)
        );
    }

    #[test]
    fn parses_pixel_size() {
        assert_eq!("1600x900".parse(), Ok(PixelSize::new(1600, 900)));
        assert_eq!("1600X900".parse(), Ok(PixelSize::new(1600, 900)));
    }

    #[test]
    fn validates_manual_capture_catalog_invariants() {
        assert!(validate_capture_routes::<TestRoute>().is_ok());
        assert!(validate_capture_scenarios::<TestScenario>().is_ok());

        assert!(matches!(
            validate_capture_routes::<MissingDefaultRoute>(),
            Err(CaptureCatalogValidationError::MissingDefaultRoute { .. })
        ));
        assert!(matches!(
            validate_capture_scenarios::<DuplicateScenario>(),
            Err(CaptureCatalogValidationError::DuplicateId {
                kind: "scenario",
                ..
            })
        ));
        assert!(matches!(
            validate_capture_routes::<MissingRouteMetadata>(),
            Err(CaptureCatalogValidationError::MissingMetadata { kind: "route", .. })
        ));
        assert!(matches!(
            validate_capture_scenarios::<MissingScenarioMetadata>(),
            Err(CaptureCatalogValidationError::MissingMetadata {
                kind: "scenario",
                ..
            })
        ));

        assert_eq!(
            TestRoute::from_id("missing").unwrap_err().value(),
            "missing"
        );
        assert_eq!(MissingDefaultRoute::Settings.spec().id(), "settings");
        assert_eq!(
            MissingDefaultRoute::from_id("settings"),
            Ok(MissingDefaultRoute::Settings)
        );
        assert!(MissingDefaultRoute::from_id("missing").is_err());
        assert!(DuplicateScenario::from_id("missing").is_err());
        assert_eq!(MissingRouteMetadata::Settings.spec().id(), "settings");
        assert_eq!(
            MissingRouteMetadata::from_id("settings"),
            Ok(MissingRouteMetadata::Settings)
        );
        assert!(MissingRouteMetadata::from_id("missing").is_err());
        assert_eq!(MissingScenarioMetadata::LedOff.id(), "led-off");
        assert_eq!(
            MissingScenarioMetadata::from_id("led-off"),
            Ok(MissingScenarioMetadata::LedOff)
        );
        assert!(MissingScenarioMetadata::from_id("missing").is_err());
    }

    #[test]
    fn reads_capture_input_session_with_typed_state() {
        let env = CaptureEnv::builder()
            .route_var("FRAME_CAPTURE_INPUT_SESSION_ROUTE_TEST")
            .unwrap()
            .path_var("FRAME_CAPTURE_INPUT_SESSION_PATH_TEST")
            .unwrap()
            .width_var("FRAME_CAPTURE_INPUT_SESSION_WIDTH_TEST")
            .unwrap()
            .height_var("FRAME_CAPTURE_INPUT_SESSION_HEIGHT_TEST")
            .unwrap()
            .scenario_var("FRAME_CAPTURE_INPUT_SESSION_SCENARIO_TEST")
            .unwrap()
            .build();

        unsafe {
            std::env::set_var(env.route_var(), "settings");
            std::env::set_var(env.path_var(), "capture.png");
            std::env::set_var(env.width_var(), "320");
            std::env::set_var(env.height_var(), "240");
            std::env::set_var(env.scenario_var(), "led-on");
        }
        let session = env
            .read_session_with_inputs::<TestRoute, TestScenario>()
            .unwrap();
        unsafe {
            std::env::remove_var(env.route_var());
            std::env::remove_var(env.path_var());
            std::env::remove_var(env.width_var());
            std::env::remove_var(env.height_var());
            std::env::remove_var(env.scenario_var());
        }

        assert_eq!(session.route(), TestRoute::Settings);
        assert_eq!(session.scenario(), Some(TestScenario::LedOn));
        assert_eq!(session.capture().unwrap().size(), PixelSize::new(320, 240));
    }

    #[test]
    fn rejects_invalid_pixel_size() {
        assert!(matches!(
            "1600".parse::<PixelSize>(),
            Err(ParsePixelSizeError::MissingSeparator { .. })
        ));
        assert!(matches!(
            "widex900".parse::<PixelSize>(),
            Err(ParsePixelSizeError::InvalidWidth { .. })
        ));
        assert!(matches!(
            "1600xhigh".parse::<PixelSize>(),
            Err(ParsePixelSizeError::InvalidHeight { .. })
        ));
        assert!(matches!(
            "0x900".parse::<PixelSize>(),
            Err(ParsePixelSizeError::ZeroWidth { .. })
        ));
        assert!(matches!(
            "1600x0".parse::<PixelSize>(),
            Err(ParsePixelSizeError::ZeroHeight { .. })
        ));
    }

    #[test]
    fn builds_route_local_png_output_path() {
        let output = CaptureOutputStem::new("review").unwrap();

        assert_eq!(
            capture_output_path_for_stem("captures", TestRoute::Settings, &output).unwrap(),
            PathBuf::from("captures/settings/review.png")
        );
    }

    #[test]
    fn rejects_path_like_capture_output_names() {
        assert_eq!(
            CaptureOutputStem::new("nested/review"),
            Err(CaptureOutputPathError::PathComponent)
        );
        assert_eq!(
            CaptureOutputStem::new("review.png"),
            Err(CaptureOutputPathError::Extension)
        );
        assert_eq!(
            CaptureOutputName::new("review"),
            Err(CaptureOutputPathError::MissingExtension)
        );
    }

    #[test]
    fn builds_output_paths_from_explicit_png_names() {
        let root = CaptureOutputRoot::new("captures");
        assert_eq!(
            capture_output_path_for_name(
                &root,
                TestRoute::Settings,
                &CaptureOutputName::new("review.png").unwrap()
            )
            .unwrap(),
            PathBuf::from("captures/settings/review.png")
        );
    }

    #[test]
    fn validates_capture_state_ids() {
        let id = CaptureStateId::new("led-on").unwrap();
        assert_eq!(id.as_str(), "led-on");
        assert!(matches!(
            CaptureStateId::new("states/led-on"),
            Err(ParseCaptureStateIdError::PathComponent { .. })
        ));
    }

    #[test]
    fn validates_nested_capture_route_ids() {
        let id = CaptureRouteId::new("settings/tool").unwrap();
        assert_eq!(id.as_str(), "settings/tool");
        assert!(matches!(
            CaptureRouteId::new("../settings"),
            Err(ParseCaptureRouteIdError::InvalidPath { .. })
        ));
    }

    #[test]
    fn parses_scenario_errors_with_expected_ids() {
        let error = TestScenario::from_id("missing").unwrap_err();

        assert_eq!(error.value(), "missing");
        assert_eq!(error.expected(), ["led-on", "led-off"]);
    }

    #[test]
    fn no_capture_markers_reject_all_ids() {
        assert!(NoCaptureScenario::VARIANTS.is_empty());

        let scenario = NoCaptureScenario::from_id("state").unwrap_err();

        assert_eq!(scenario.value(), "state");
        assert!(scenario.expected().is_empty());
    }

    #[test]
    fn serializes_validated_core_values_as_plain_config_values() {
        assert_eq!(
            serde_json::to_string(&PixelSize::new(1600, 900)).unwrap(),
            r#""1600x900""#
        );
        assert_eq!(
            serde_json::from_str::<PixelSize>(r#""1600x900""#).unwrap(),
            PixelSize::new(1600, 900)
        );
        assert_eq!(
            serde_json::from_str::<CaptureStateId>(r#""led-on""#).unwrap(),
            CaptureStateId::new("led-on").unwrap()
        );
        assert!(serde_json::from_str::<CaptureOutputStem>(r#""review.png""#).is_err());
        assert_eq!(
            serde_json::from_str::<CaptureFrame>("12").unwrap(),
            CaptureFrame::new(12)
        );
        assert_eq!(
            serde_json::from_str::<CaptureOutputRoot>(r#""captures""#)
                .unwrap()
                .as_path(),
            Path::new("captures")
        );
        assert_eq!(
            serde_json::to_value(capture_route_catalog::<TestRoute>()).unwrap(),
            serde_json::json!({
                "routes": [
                    {
                        "id": "root",
                        "title": "Root",
                        "default_size": {
                            "width": 1920,
                            "height": 1080,
                            "label": "1920x1080"
                        }
                    },
                    {
                        "id": "settings",
                        "title": "Settings",
                        "default_size": {
                            "width": 800,
                            "height": 600,
                            "label": "800x600"
                        }
                    }
                ]
            })
        );
    }

    #[test]
    fn validates_capture_env_var_names() {
        assert_eq!(
            CaptureEnvVar::new("APP_CAPTURE_ROUTE").unwrap().as_str(),
            "APP_CAPTURE_ROUTE"
        );
        assert!(matches!(
            CaptureEnvVar::new(""),
            Err(ParseCaptureEnvVarError::Empty)
        ));
        assert!(matches!(
            CaptureEnvVar::new("APP=CAPTURE_ROUTE"),
            Err(ParseCaptureEnvVarError::ContainsEquals { .. })
        ));
    }

    #[test]
    fn builds_capture_env_with_named_builder_methods() {
        let env = CaptureEnv::builder()
            .route_var("APP_ROUTE")
            .unwrap()
            .path_var("APP_CAPTURE_PATH")
            .unwrap()
            .build();

        assert_eq!(env.route_var(), "APP_ROUTE");
        assert_eq!(env.path_var(), "APP_CAPTURE_PATH");
        assert_eq!(env.frame_var(), "FRAME_CAPTURE_FRAME");
        assert!(CaptureEnv::builder().route_var("APP=ROUTE").is_err());
    }

    #[test]
    fn route_id_helper_uses_default_when_route_env_is_missing() {
        let env = CaptureEnv::builder()
            .route_var("FRAME_CAPTURE_MISSING_ROUTE_ID_HELPER_TEST")
            .unwrap()
            .build();
        let default = CaptureRouteId::new("dashboard").unwrap();

        let (route_id, source) = env.read_route_id_or(&default).unwrap();

        assert_eq!(route_id, default);
        assert_eq!(source, CaptureRouteSource::Default);
    }

    #[test]
    fn try_capture_env_with_prefix_returns_parse_errors() {
        let env = CaptureEnv::try_with_prefix("MY_APP").unwrap();

        assert_eq!(env.route_var(), "MY_APP_CAPTURE_ROUTE");
        assert_eq!(env.path_var(), "MY_APP_CAPTURE_PATH");
        assert_eq!(env.frame_var(), "MY_APP_CAPTURE_FRAME");
        assert_eq!(env.width_var(), "MY_APP_CAPTURE_WIDTH");
        assert_eq!(env.height_var(), "MY_APP_CAPTURE_HEIGHT");
        assert_eq!(env.scenario_var(), "MY_APP_CAPTURE_SCENARIO");

        assert!(CaptureEnv::try_with_prefix("BAD=APP").is_err());
    }

    #[test]
    fn capture_env_with_prefix_reuses_builder_helper() {
        let env = CaptureEnvBuilder::default().prefix("APP").unwrap().build();

        assert_eq!(env.route_var(), "APP_CAPTURE_ROUTE");
    }

    #[test]
    fn builds_typed_capture_launch_env_vars() {
        let launch = CaptureLaunchEnv::builder()
            .route_id("settings")
            .unwrap()
            .maybe_output_path(Some(PathBuf::from("captures/settings.png")))
            .unwrap()
            .maybe_frame(Some(2))
            .unwrap()
            .maybe_size(CaptureLaunchEnv::optional_size(Some(800), Some(600)).unwrap())
            .unwrap()
            .maybe_scenario_id(Some("led-on"))
            .unwrap()
            .build();

        assert_eq!(launch.route_id().as_str(), "settings");
        assert_eq!(
            launch.output_path().unwrap().as_path(),
            Path::new("captures/settings.png")
        );
        assert_eq!(launch.frame(), Some(CaptureFrame::new(2)));
        assert_eq!(launch.size(), Some(PixelSize::new(800, 600)));
        assert_eq!(launch.scenario_id().unwrap().as_str(), "led-on");
        assert_eq!(
            launch.env_map_lossy(),
            BTreeMap::from([
                ("FRAME_CAPTURE_HEIGHT".to_string(), "600".to_string(),),
                (
                    "FRAME_CAPTURE_PATH".to_string(),
                    "captures/settings.png".to_string(),
                ),
                ("FRAME_CAPTURE_ROUTE".to_string(), "settings".to_string(),),
                ("FRAME_CAPTURE_SCENARIO".to_string(), "led-on".to_string(),),
                ("FRAME_CAPTURE_FRAME".to_string(), "2".to_string()),
                ("FRAME_CAPTURE_WIDTH".to_string(), "800".to_string()),
            ])
        );
    }

    #[test]
    fn typed_capture_launch_env_uses_custom_env_names() {
        let launch = CaptureLaunchEnv::builder()
            .route_id(CaptureRouteId::new("settings").unwrap())
            .unwrap()
            .env(CaptureEnv::try_with_prefix("APP").unwrap())
            .build();
        let vars = launch.vars();

        assert_eq!(vars.len(), 1);
        assert_eq!(vars[0].name(), "APP_CAPTURE_ROUTE");
        assert_eq!(vars[0].value_string_lossy(), "settings");
    }

    #[test]
    fn typed_capture_launch_env_rejects_invalid_inputs() {
        assert!(matches!(
            CaptureLaunchEnv::builder().route_id("../settings"),
            Err(CaptureLaunchEnvError::InvalidRouteId { .. })
        ));
        assert!(matches!(
            CaptureLaunchEnv::builder()
                .route_id("settings")
                .unwrap()
                .output_path(PathBuf::from("captures/settings.jpg")),
            Err(CaptureLaunchEnvError::InvalidOutputPath { .. })
        ));
        assert!(matches!(
            CaptureLaunchEnv::builder()
                .route_id("settings")
                .unwrap()
                .frame(0),
            Err(CaptureLaunchEnvError::InvalidFrame { .. })
        ));
        assert_eq!(
            CaptureLaunchEnv::optional_size(Some(800), None),
            Err(CaptureLaunchEnvError::PartialSize)
        );
        assert_eq!(
            CaptureLaunchEnv::try_size(800, 0),
            Err(CaptureLaunchEnvError::InvalidSize {
                width: 800,
                height: 0,
            })
        );
        assert!(matches!(
            CaptureLaunchEnv::builder()
                .route_id("settings")
                .unwrap()
                .scenario_id("states/led-on"),
            Err(CaptureLaunchEnvError::InvalidScenarioId { .. })
        ));
    }
}
