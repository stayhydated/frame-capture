use thiserror::Error;

use crate::{CaptureItemSpec, CaptureScenarioIdRef, RouteSpec};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureRouteVariant<R> {
    pub route: R,
    pub spec: RouteSpec,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureItemVariant<T> {
    pub value: T,
    pub spec: CaptureItemSpec,
}

pub trait CaptureRoute: Copy + Eq + Sized + 'static {
    const DEFAULT: Self;
    const ROUTES: &'static [Self];
    const VARIANTS: &'static [RouteSpec];
    const ROUTE_SPECS: &'static [CaptureRouteVariant<Self>];

    fn spec(self) -> RouteSpec;

    /// Parses a route id into a typed route value.
    ///
    /// # Errors
    ///
    /// Returns [`ParseRouteError`] when `value` is not one of the route ids
    /// exposed by this route catalog.
    fn from_id(value: &str) -> Result<Self, ParseRouteError>;

    fn id(self) -> &'static str {
        self.spec().id()
    }
}

pub trait CaptureScenario: Copy + Eq + Sized + 'static {
    const SCENARIOS: &'static [Self];
    const VARIANTS: &'static [&'static str];
    const SPECS: &'static [CaptureItemSpec];
    const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>];

    fn id(self) -> &'static str;

    fn id_ref(self) -> CaptureScenarioIdRef {
        CaptureScenarioIdRef::new(self.id())
    }

    /// Returns metadata for this scenario.
    ///
    /// # Panics
    ///
    /// Panics when [`Self::SCENARIO_SPECS`] does not contain metadata for this
    /// scenario value.
    fn spec(self) -> CaptureItemSpec {
        for variant in Self::SCENARIO_SPECS {
            if variant.value == self {
                return variant.spec;
            }
        }

        panic!("capture scenario is missing from SCENARIO_SPECS")
    }

    /// Parses a scenario id into a typed scenario value.
    ///
    /// # Errors
    ///
    /// Returns [`ParseScenarioError`] when `value` is not one of the scenario
    /// ids exposed by this scenario catalog.
    fn from_id(value: &str) -> Result<Self, ParseScenarioError>;
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NoCaptureScenario {}

macro_rules! define_parse_id_error {
    ($name:ident, $message:literal) => {
        #[derive(Clone, Debug, Eq, Error, PartialEq)]
        #[error($message, .expected.join(", "))]
        pub struct $name {
            value: String,
            expected: Vec<&'static str>,
        }

        impl $name {
            pub fn new(
                value: impl Into<String>,
                expected: impl IntoIterator<Item = &'static str>,
            ) -> Self {
                Self {
                    value: value.into(),
                    expected: expected.into_iter().collect(),
                }
            }

            pub fn value(&self) -> &str {
                &self.value
            }

            pub fn expected(&self) -> &[&'static str] {
                &self.expected
            }
        }
    };
}

define_parse_id_error!(
    ParseRouteError,
    "unknown capture route `{value}`; expected one of: {}"
);
define_parse_id_error!(
    ParseScenarioError,
    "unknown capture scenario `{value}`; expected one of: {}"
);

impl CaptureScenario for NoCaptureScenario {
    const SCENARIOS: &'static [Self] = &[];
    const VARIANTS: &'static [&'static str] = &[];
    const SPECS: &'static [CaptureItemSpec] = &[];
    const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[];

    fn id(self) -> &'static str {
        match self {}
    }

    fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
        Err(ParseScenarioError::new(
            value,
            Self::VARIANTS.iter().copied(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone, Copy, Debug, Eq, PartialEq)]
    enum MissingScenarioSpec {
        Value,
    }

    impl CaptureScenario for MissingScenarioSpec {
        const SCENARIOS: &'static [Self] = &[Self::Value];
        const VARIANTS: &'static [&'static str] = &["value"];
        const SPECS: &'static [CaptureItemSpec] = &[];
        const SCENARIO_SPECS: &'static [CaptureItemVariant<Self>] = &[];

        fn id(self) -> &'static str {
            "value"
        }

        fn from_id(value: &str) -> Result<Self, ParseScenarioError> {
            (value == "value")
                .then_some(Self::Value)
                .ok_or_else(|| ParseScenarioError::new(value, ["value"]))
        }
    }

    #[test]
    fn scenario_spec_panics_when_metadata_is_missing() {
        assert_eq!(MissingScenarioSpec::Value.id(), "value");
        assert_eq!(MissingScenarioSpec::Value.id_ref().as_str(), "value");
        assert_eq!(
            MissingScenarioSpec::from_id("value"),
            Ok(MissingScenarioSpec::Value)
        );
        assert!(MissingScenarioSpec::from_id("missing").is_err());
        assert!(std::panic::catch_unwind(|| MissingScenarioSpec::Value.spec()).is_err());
    }
}
