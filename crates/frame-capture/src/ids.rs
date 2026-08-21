use std::{borrow::Borrow, str::FromStr};

use koruma::Validate as _;
use serde::{Serialize, Serializer};
use thiserror::Error;

#[derive(
    Clone,
    Copy,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureRouteIdRef(&'static str);

#[derive(
    Clone,
    Copy,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureStateIdRef(&'static str);

#[derive(
    Clone,
    Copy,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    Serialize,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureScenarioIdRef(&'static str);

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureRouteId(String);

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureStateId(String);

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureScenarioId(String);

#[derive(
    Clone,
    Debug,
    derive_more::AsRef,
    derive_more::Deref,
    derive_more::Display,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
#[as_ref(forward)]
#[deref(forward)]
#[display("{}", _0)]
pub struct CaptureEnvVar(String);

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseCaptureEnvVarError {
    #[error("capture env var name must not be empty")]
    Empty,
    #[error("capture env var name `{value}` must not contain `=`")]
    ContainsEquals { value: String },
    #[error("capture env var name must not contain NUL")]
    ContainsNul,
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseCaptureRouteIdError {
    #[error("capture route id must not be empty")]
    Empty,
    #[error("capture route id `{value}` must be a relative route id")]
    InvalidPath { value: String },
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseCaptureStateIdError {
    #[error("capture state id must not be empty")]
    Empty,
    #[error("capture state id `{value}` must be an id, not a path")]
    PathComponent { value: String },
}

#[derive(Clone, Copy, Debug, Default)]
pub(crate) struct StateIdValueValidator;

impl koruma::Validate<&str> for StateIdValueValidator {
    fn validate(&self, value: &&str) -> bool {
        !is_path_like_id(value)
    }
}

#[derive(Clone, Copy, Debug, Default)]
struct RouteIdValueValidator;

impl koruma::Validate<&str> for RouteIdValueValidator {
    fn validate(&self, value: &&str) -> bool {
        !is_invalid_route_id(value)
    }
}

impl CaptureRouteIdRef {
    /// Creates a static relative route id.
    ///
    /// # Panics
    ///
    /// Panics when `value` is empty, absolute, contains an empty component,
    /// uses `.` or `..`, or contains a backslash. Use [`Self::try_new`] when
    /// invalid ids are recoverable input.
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_route_id_const(value) {
            panic!("route id must be a relative route id");
        }

        Self(value)
    }

    /// Creates a static relative route id after validation.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureRouteIdError`] when `value` is not a valid
    /// relative route id.
    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureRouteIdError> {
        validate_route_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureStateIdRef {
    /// Creates a static state id.
    ///
    /// # Panics
    ///
    /// Panics when `value` is empty, `.`, `..`, or contains a path separator.
    /// Use [`Self::try_new`] when invalid ids are recoverable input.
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_state_id_const(value) {
            panic!("capture state id must be an id, not a path");
        }

        Self(value)
    }

    /// Creates a static state id after validation.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureStateIdError`] when `value` is empty or path-like.
    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureStateIdError> {
        validate_state_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureScenarioIdRef {
    /// Creates a static scenario id.
    ///
    /// # Panics
    ///
    /// Panics when `value` is empty, `.`, `..`, or contains a path separator.
    /// Use [`Self::try_new`] when invalid ids are recoverable input.
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_state_id_const(value) {
            panic!("capture scenario id must be an id, not a path");
        }

        Self(value)
    }

    /// Creates a static scenario id after validation.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureStateIdError`] when `value` is empty or path-like.
    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureStateIdError> {
        validate_state_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureRouteId {
    /// Creates an owned relative route id.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureRouteIdError`] when `value` is not a valid
    /// relative route id.
    pub fn new(value: impl Into<String>) -> Result<Self, ParseCaptureRouteIdError> {
        Self::try_from(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl CaptureStateId {
    /// Creates an owned state id.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureStateIdError`] when `value` is empty or path-like.
    pub fn new(value: impl Into<String>) -> Result<Self, ParseCaptureStateIdError> {
        Self::try_from(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl CaptureScenarioId {
    /// Creates an owned scenario id.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureStateIdError`] when `value` is empty or path-like.
    pub fn new(value: impl Into<String>) -> Result<Self, ParseCaptureStateIdError> {
        Self::try_from(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl CaptureEnvVar {
    /// Creates an owned environment variable name.
    ///
    /// # Errors
    ///
    /// Returns [`ParseCaptureEnvVarError`] when `value` is empty, contains `=`,
    /// or contains NUL.
    pub fn new(value: impl Into<String>) -> Result<Self, ParseCaptureEnvVarError> {
        Self::try_from(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl Borrow<str> for CaptureRouteId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<CaptureRouteId> for String {
    fn from(value: CaptureRouteId) -> Self {
        value.into_string()
    }
}

impl FromStr for CaptureRouteId {
    type Err = ParseCaptureRouteIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Borrow<str> for CaptureRouteIdRef {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl TryFrom<String> for CaptureRouteId {
    type Error = ParseCaptureRouteIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_route_id(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CaptureRouteId {
    type Error = ParseCaptureRouteIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Borrow<str> for CaptureStateId {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<CaptureStateId> for String {
    fn from(value: CaptureStateId) -> Self {
        value.into_string()
    }
}

impl FromStr for CaptureStateId {
    type Err = ParseCaptureStateIdError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl Borrow<str> for CaptureStateIdRef {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

macro_rules! impl_capture_state_id_like {
    ($owned:ident, $ref:ident) => {
        impl Borrow<str> for $owned {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$owned> for String {
            fn from(value: $owned) -> Self {
                value.into_string()
            }
        }

        impl From<$owned> for CaptureStateId {
            fn from(value: $owned) -> Self {
                Self(value.into_string())
            }
        }

        impl From<CaptureStateId> for $owned {
            fn from(value: CaptureStateId) -> Self {
                Self(value.into_string())
            }
        }

        impl FromStr for $owned {
            type Err = ParseCaptureStateIdError;

            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }

        impl Borrow<str> for $ref {
            fn borrow(&self) -> &str {
                self.as_str()
            }
        }

        impl From<$ref> for CaptureStateIdRef {
            fn from(value: $ref) -> Self {
                Self(value.as_str())
            }
        }

        impl TryFrom<String> for $owned {
            type Error = ParseCaptureStateIdError;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                validate_state_id(&value)?;
                Ok(Self(value))
            }
        }

        impl TryFrom<&str> for $owned {
            type Error = ParseCaptureStateIdError;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Self::try_from(value.to_owned())
            }
        }
    };
}

impl_capture_state_id_like!(CaptureScenarioId, CaptureScenarioIdRef);

impl TryFrom<String> for CaptureStateId {
    type Error = ParseCaptureStateIdError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_state_id(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CaptureStateId {
    type Error = ParseCaptureStateIdError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Borrow<str> for CaptureEnvVar {
    fn borrow(&self) -> &str {
        self.as_str()
    }
}

impl From<CaptureEnvVar> for String {
    fn from(value: CaptureEnvVar) -> Self {
        value.into_string()
    }
}

impl FromStr for CaptureEnvVar {
    type Err = ParseCaptureEnvVarError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        Self::new(value)
    }
}

impl TryFrom<String> for CaptureEnvVar {
    type Error = ParseCaptureEnvVarError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        validate_env_var(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for CaptureEnvVar {
    type Error = ParseCaptureEnvVarError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::try_from(value.to_owned())
    }
}

impl Serialize for CaptureRouteIdRef {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

fn validate_state_id(value: &str) -> Result<(), ParseCaptureStateIdError> {
    if value.is_empty() {
        return Err(ParseCaptureStateIdError::Empty);
    }
    if !StateIdValueValidator.validate(&value) {
        return Err(ParseCaptureStateIdError::PathComponent {
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_route_id(value: &str) -> Result<(), ParseCaptureRouteIdError> {
    if value.is_empty() {
        return Err(ParseCaptureRouteIdError::Empty);
    }
    if !RouteIdValueValidator.validate(&value) {
        return Err(ParseCaptureRouteIdError::InvalidPath {
            value: value.to_owned(),
        });
    }

    Ok(())
}

fn validate_env_var(value: &str) -> Result<(), ParseCaptureEnvVarError> {
    if value.is_empty() {
        return Err(ParseCaptureEnvVarError::Empty);
    }
    if value.contains('=') {
        return Err(ParseCaptureEnvVarError::ContainsEquals {
            value: value.to_owned(),
        });
    }
    if value.contains('\0') {
        return Err(ParseCaptureEnvVarError::ContainsNul);
    }

    Ok(())
}

fn is_path_like_id(value: &str) -> bool {
    value == "." || value == ".." || value.contains('/') || value.contains('\\')
}

const fn is_valid_state_id_const(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() {
        return false;
    }
    if bytes.len() == 1 && bytes[0] == b'.' {
        return false;
    }
    if bytes.len() == 2 && bytes[0] == b'.' && bytes[1] == b'.' {
        return false;
    }

    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'/' || bytes[index] == b'\\' {
            return false;
        }
        index += 1;
    }

    true
}

fn is_invalid_route_id(value: &str) -> bool {
    value.contains('\\')
        || value.starts_with('/')
        || value.ends_with('/')
        || value
            .split('/')
            .any(|component| component.is_empty() || component == "." || component == "..")
}

const fn is_valid_route_id_const(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.is_empty() || bytes[0] == b'/' || bytes[bytes.len() - 1] == b'/' {
        return false;
    }

    let mut component_start = 0;
    let mut index = 0;
    while index < bytes.len() {
        if bytes[index] == b'\\' {
            return false;
        }
        if bytes[index] == b'/' {
            if is_invalid_route_component_const(bytes, component_start, index) {
                return false;
            }
            component_start = index + 1;
        }
        index += 1;
    }

    !is_invalid_route_component_const(bytes, component_start, bytes.len())
}

const fn is_invalid_route_component_const(bytes: &[u8], start: usize, end: usize) -> bool {
    let len = end - start;

    len == 0
        || (len == 1 && bytes[start] == b'.')
        || (len == 2 && bytes[start] == b'.' && bytes[start + 1] == b'.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn static_id_refs_validate_and_serialize() {
        let route = CaptureRouteIdRef::new("settings/tool");
        let state = CaptureStateIdRef::new("loaded");
        let scenario = CaptureScenarioIdRef::new("review");

        let route_borrowed: &str = route.borrow();
        let state_borrowed: &str = state.borrow();
        let scenario_borrowed: &str = scenario.borrow();
        assert_eq!(route_borrowed, "settings/tool");
        assert_eq!(state_borrowed, "loaded");
        assert_eq!(scenario_borrowed, "review");
        assert_eq!(serde_json::to_string(&route).unwrap(), r#""settings/tool""#);
        assert_eq!(serde_json::to_string(&state).unwrap(), r#""loaded""#);
        assert_eq!(serde_json::to_string(&scenario).unwrap(), r#""review""#);
        assert_eq!(CaptureScenarioIdRef::try_new("review").unwrap(), scenario);

        assert_eq!(
            CaptureRouteIdRef::try_new(""),
            Err(ParseCaptureRouteIdError::Empty)
        );
        assert!(CaptureRouteIdRef::try_new("settings//tool").is_err());
        assert_eq!(
            CaptureStateIdRef::try_new(""),
            Err(ParseCaptureStateIdError::Empty)
        );
        assert!(CaptureStateIdRef::try_new("states/loaded").is_err());
        assert!(CaptureScenarioIdRef::try_new("../review").is_err());
    }

    #[test]
    fn const_id_constructors_reject_every_path_shape() {
        for invalid in [
            "",
            "/root",
            "root/",
            "root//tool",
            "root/./tool",
            "root/../tool",
            "root\\tool",
        ] {
            assert!(std::panic::catch_unwind(|| CaptureRouteIdRef::new(invalid)).is_err());
        }
        for invalid in ["", ".", "..", "states/loaded", "states\\loaded"] {
            assert!(std::panic::catch_unwind(|| CaptureStateIdRef::new(invalid)).is_err());
            assert!(std::panic::catch_unwind(|| CaptureScenarioIdRef::new(invalid)).is_err());
        }
    }

    #[test]
    fn owned_ids_support_parse_borrow_and_conversion_contracts() {
        let route = CaptureRouteId::try_from("settings/tool").unwrap();
        let route_borrowed: &str = route.borrow();
        assert_eq!(route_borrowed, "settings/tool");
        assert_eq!(String::from(route.clone()), "settings/tool");
        assert_eq!("settings/tool".parse::<CaptureRouteId>().unwrap(), route);

        let state = CaptureStateId::try_from("loaded").unwrap();
        let state_borrowed: &str = state.borrow();
        assert_eq!(state_borrowed, "loaded");
        assert_eq!(String::from(state.clone()), "loaded");
        assert_eq!("loaded".parse::<CaptureStateId>().unwrap(), state);

        let scenario = CaptureScenarioId::try_from("review").unwrap();
        let scenario_borrowed: &str = scenario.borrow();
        assert_eq!(scenario_borrowed, "review");
        assert_eq!(String::from(scenario.clone()), "review");
        assert_eq!("review".parse::<CaptureScenarioId>().unwrap(), scenario);

        let state_from_scenario = CaptureStateId::from(scenario.clone());
        assert_eq!(state_from_scenario.as_str(), "review");
        assert_eq!(CaptureScenarioId::from(state_from_scenario), scenario);
        assert_eq!(
            CaptureStateIdRef::from(CaptureScenarioIdRef::new("review")).as_str(),
            "review"
        );

        assert_eq!(
            CaptureRouteId::new(""),
            Err(ParseCaptureRouteIdError::Empty)
        );
        assert_eq!(
            CaptureStateId::new(""),
            Err(ParseCaptureStateIdError::Empty)
        );
        assert_eq!(
            CaptureScenarioId::new(""),
            Err(ParseCaptureStateIdError::Empty)
        );
    }

    #[test]
    fn env_var_supports_owned_conversions_and_all_validation_errors() {
        let env_var = CaptureEnvVar::try_from("APP_CAPTURE_ROUTE").unwrap();
        let borrowed: &str = env_var.borrow();
        assert_eq!(borrowed, "APP_CAPTURE_ROUTE");
        assert_eq!(String::from(env_var.clone()), "APP_CAPTURE_ROUTE");
        assert_eq!(
            "APP_CAPTURE_ROUTE".parse::<CaptureEnvVar>().unwrap(),
            env_var
        );
        assert_eq!(
            CaptureEnvVar::new("A\0B"),
            Err(ParseCaptureEnvVarError::ContainsNul)
        );
    }
}
