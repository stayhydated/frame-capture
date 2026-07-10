use frame_capture::{
    CaptureCatalogValidationError, CaptureItemSpec, CaptureItemVariant, CaptureRoute,
    CaptureRouteVariant, CaptureScenario, ParseRouteError, ParseScenarioError, PixelSize,
    RouteSpec, validate_capture_routes, validate_capture_scenarios,
};

macro_rules! define_route_catalog {
    (
        $name:ident,
        routes = $routes:expr,
        variants = $variants:expr,
        route_specs = $route_specs:expr,
        parse = $parse:expr
    ) => {
        #[allow(
            dead_code,
            reason = "catalog validation fixtures intentionally omit variants from selected slices"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {
            A,
            B,
        }

        impl CaptureRoute for $name {
            const DEFAULT: Self = Self::A;
            const ROUTES: &'static [Self] = $routes;
            const VARIANTS: &'static [RouteSpec] = $variants;
            const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>] = $route_specs;

            fn spec(self) -> RouteSpec {
                match self {
                    Self::A => Self::VARIANTS[0],
                    Self::B => Self::VARIANTS[1],
                }
            }

            fn from_id(value: &str) -> Result<Self, ParseRouteError> {
                ($parse)(value)
            }
        }
    };
}

define_route_catalog!(
    DuplicateRoutes,
    routes = &[Self::A, Self::B],
    variants = &[
        RouteSpec::new("same", "A", PixelSize::new(10, 10)),
        RouteSpec::new("same", "B", PixelSize::new(20, 20)),
    ],
    route_specs = &[
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[0]
        },
        CaptureRouteVariant {
            route: Self::B,
            spec: Self::VARIANTS[1]
        },
    ],
    parse = |value| match value {
        "same" => Ok(Self::A),
        _ => Err(ParseRouteError::new(value, ["same"])),
    }
);

define_route_catalog!(
    MismatchedParserRoute,
    routes = &[Self::A, Self::B],
    variants = &[
        RouteSpec::new("a", "A", PixelSize::new(10, 10)),
        RouteSpec::new("b", "B", PixelSize::new(20, 20)),
    ],
    route_specs = &[
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[0]
        },
        CaptureRouteVariant {
            route: Self::B,
            spec: Self::VARIANTS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::B),
        "b" => Ok(Self::B),
        _ => Err(ParseRouteError::new(value, ["a", "b"])),
    }
);

define_route_catalog!(
    UnknownParserRoute,
    routes = &[Self::A, Self::B],
    variants = &[
        RouteSpec::new("a", "A", PixelSize::new(10, 10)),
        RouteSpec::new("b", "B", PixelSize::new(20, 20)),
    ],
    route_specs = &[
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[0]
        },
        CaptureRouteVariant {
            route: Self::B,
            spec: Self::VARIANTS[1]
        },
    ],
    parse = |value| Err(ParseRouteError::new(value, ["a", "b"]))
);

define_route_catalog!(
    DuplicateRouteSpecs,
    routes = &[Self::A, Self::B],
    variants = &[
        RouteSpec::new("a", "A", PixelSize::new(10, 10)),
        RouteSpec::new("b", "B", PixelSize::new(20, 20)),
    ],
    route_specs = &[
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[0]
        },
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[0]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseRouteError::new(value, ["a", "b"])),
    }
);

define_route_catalog!(
    UnknownRouteSpec,
    routes = &[Self::A],
    variants = &[
        RouteSpec::new("a", "A", PixelSize::new(10, 10)),
        RouteSpec::new("other", "Other", PixelSize::new(20, 20)),
    ],
    route_specs = &[CaptureRouteVariant {
        route: Self::A,
        spec: Self::VARIANTS[1]
    }],
    parse = |value| match value {
        "a" => Ok(Self::A),
        _ => Err(ParseRouteError::new(value, ["a"])),
    }
);

define_route_catalog!(
    MismatchedRouteSpec,
    routes = &[Self::A, Self::B],
    variants = &[
        RouteSpec::new("a", "A", PixelSize::new(10, 10)),
        RouteSpec::new("b", "B", PixelSize::new(20, 20)),
    ],
    route_specs = &[
        CaptureRouteVariant {
            route: Self::A,
            spec: Self::VARIANTS[1]
        },
        CaptureRouteVariant {
            route: Self::B,
            spec: Self::VARIANTS[0]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseRouteError::new(value, ["a", "b"])),
    }
);

macro_rules! define_scenario_catalog {
    (
        $name:ident,
        ids = ($a_id:literal, $b_id:literal),
        scenarios = $scenarios:expr,
        variants = $variants:expr,
        specs = $specs:expr,
        scenario_specs = $scenario_specs:expr,
        parse = $parse:expr,
        spec = $spec:expr
    ) => {
        #[allow(
            dead_code,
            reason = "catalog validation fixtures intentionally omit variants from selected slices"
        )]
        #[derive(Clone, Copy, Debug, Eq, PartialEq)]
        enum $name {
            A,
            B,
        }

        impl CaptureScenario for $name {
            const SCENARIOS: &'static [Self] = $scenarios;
            const VARIANTS: &'static [&'static str] = $variants;
            const SPECS: &'static [CaptureItemSpec] = $specs;
            const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = $scenario_specs;

            fn id(self) -> &'static str {
                match self {
                    Self::A => $a_id,
                    Self::B => $b_id,
                }
            }

            fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
                ($parse)(value)
            }

            fn spec(self) -> CaptureItemSpec {
                ($spec)(self)
            }
        }
    };
}

define_scenario_catalog!(
    InvalidTypedScenarioId,
    ids = ("states/a", "b"),
    scenarios = &[Self::A],
    variants = &["states/a"],
    specs = &[CaptureItemSpec::new("a", "A")],
    scenario_specs = &[CaptureItemVariant {
        value: Self::A,
        spec: Self::SPECS[0]
    }],
    parse = |value| match value {
        "states/a" => Ok(Self::A),
        _ => Err(ParseScenarioError::new(value, ["states/a"])),
    },
    spec = |_| Self::SPECS[0]
);

define_scenario_catalog!(
    UnknownScenarioParser,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::B),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    DuplicateScenarioVariants,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "a"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    InvalidScenarioVariant,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "states/b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    UnknownScenarioVariant,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "c"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    DuplicateScenarioSpecs,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("a", "A again")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: CaptureItemSpec::new("b", "B")
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => CaptureItemSpec::new("b", "B"),
    }
);

define_scenario_catalog!(
    UnknownScenarioSpec,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("c", "C")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: CaptureItemSpec::new("b", "B")
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => CaptureItemSpec::new("b", "B"),
    }
);

define_scenario_catalog!(
    MismatchedTypedScenarioSpec,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[1]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[0]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    UnknownTypedScenarioSpec,
    ids = ("a", "c"),
    scenarios = &[Self::A],
    variants = &["a"],
    specs = &[CaptureItemSpec::new("a", "A")],
    scenario_specs = &[CaptureItemVariant {
        value: Self::B,
        spec: CaptureItemSpec::new("c", "C")
    }],
    parse = |value| match value {
        "a" => Ok(Self::A),
        _ => Err(ParseScenarioError::new(value, ["a"])),
    },
    spec = |_| Self::SPECS[0]
);

define_scenario_catalog!(
    DuplicateTypedScenarioSpecs,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    MissingTypedScenarioSpec,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[CaptureItemVariant {
        value: Self::A,
        spec: Self::SPECS[0]
    }],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[0],
        Self::B => Self::SPECS[1],
    }
);

define_scenario_catalog!(
    MismatchedScenarioAccessor,
    ids = ("a", "b"),
    scenarios = &[Self::A, Self::B],
    variants = &["a", "b"],
    specs = &[
        CaptureItemSpec::new("a", "A"),
        CaptureItemSpec::new("b", "B")
    ],
    scenario_specs = &[
        CaptureItemVariant {
            value: Self::A,
            spec: Self::SPECS[0]
        },
        CaptureItemVariant {
            value: Self::B,
            spec: Self::SPECS[1]
        },
    ],
    parse = |value| match value {
        "a" => Ok(Self::A),
        "b" => Ok(Self::B),
        _ => Err(ParseScenarioError::new(value, ["a", "b"])),
    },
    spec = |value| match value {
        Self::A => Self::SPECS[1],
        Self::B => Self::SPECS[1],
    }
);

#[test]
fn route_validation_reports_each_catalog_inconsistency() {
    assert!(matches!(
        validate_capture_routes::<DuplicateRoutes>(),
        Err(CaptureCatalogValidationError::DuplicateId { kind: "route", .. })
    ));
    assert!(matches!(
        validate_capture_routes::<MismatchedParserRoute>(),
        Err(CaptureCatalogValidationError::MismatchedId { kind: "route", .. })
    ));
    assert!(matches!(
        validate_capture_routes::<UnknownParserRoute>(),
        Err(CaptureCatalogValidationError::UnknownId { kind: "route", .. })
    ));
    assert!(matches!(
        validate_capture_routes::<DuplicateRouteSpecs>(),
        Err(CaptureCatalogValidationError::DuplicateId { kind: "route", .. })
    ));
    assert!(matches!(
        validate_capture_routes::<UnknownRouteSpec>(),
        Err(CaptureCatalogValidationError::UnknownId { kind: "route", .. })
    ));
    assert!(matches!(
        validate_capture_routes::<MismatchedRouteSpec>(),
        Err(CaptureCatalogValidationError::MismatchedId { kind: "route", .. })
    ));
}

#[test]
fn scenario_validation_reports_each_catalog_inconsistency() {
    for result in [
        validate_capture_scenarios::<InvalidTypedScenarioId>(),
        validate_capture_scenarios::<UnknownScenarioParser>(),
        validate_capture_scenarios::<DuplicateScenarioVariants>(),
        validate_capture_scenarios::<InvalidScenarioVariant>(),
        validate_capture_scenarios::<UnknownScenarioVariant>(),
        validate_capture_scenarios::<DuplicateScenarioSpecs>(),
        validate_capture_scenarios::<UnknownScenarioSpec>(),
        validate_capture_scenarios::<MismatchedTypedScenarioSpec>(),
        validate_capture_scenarios::<UnknownTypedScenarioSpec>(),
        validate_capture_scenarios::<DuplicateTypedScenarioSpecs>(),
        validate_capture_scenarios::<MissingTypedScenarioSpec>(),
        validate_capture_scenarios::<MismatchedScenarioAccessor>(),
    ] {
        assert!(result.is_err());
    }
}
