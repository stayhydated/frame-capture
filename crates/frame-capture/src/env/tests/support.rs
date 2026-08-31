use super::super::*;
use crate::{CaptureItemSpec, CaptureItemVariant, CaptureRouteVariant, RouteSpec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(super) enum Route {
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
pub(super) enum Scenario {
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

pub(super) fn env(prefix: &str) -> CaptureEnv {
    CaptureEnv::try_with_prefix(prefix).unwrap()
}

pub(super) fn clear(env: &CaptureEnv) {
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
