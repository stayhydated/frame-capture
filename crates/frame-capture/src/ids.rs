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
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_route_id_const(value) {
            panic!("route id must be a relative route id");
        }

        Self(value)
    }

    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureRouteIdError> {
        validate_route_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureStateIdRef {
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_state_id_const(value) {
            panic!("capture state id must be an id, not a path");
        }

        Self(value)
    }

    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureStateIdError> {
        validate_state_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureScenarioIdRef {
    pub const fn new(value: &'static str) -> Self {
        if !is_valid_state_id_const(value) {
            panic!("capture scenario id must be an id, not a path");
        }

        Self(value)
    }

    pub fn try_new(value: &'static str) -> Result<Self, ParseCaptureStateIdError> {
        validate_state_id(value)?;
        Ok(Self(value))
    }

    pub const fn as_str(self) -> &'static str {
        self.0
    }
}

impl CaptureRouteId {
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
