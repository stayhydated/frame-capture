use std::{
    borrow::Cow,
    collections::BTreeMap,
    env,
    ffi::{OsStr, OsString},
    fmt,
    num::NonZeroU32,
    path::PathBuf,
    str::FromStr,
};

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use thiserror::Error;

use crate::ids::{
    CaptureEnvVar, CaptureRouteId, ParseCaptureEnvVarError, ParseCaptureRouteIdError,
};
use crate::{
    CaptureConfig, CaptureInputSession, CaptureOutputPath, CaptureOutputPathError, CaptureRoute,
    CaptureScenario, CaptureScenarioId, CaptureSession, CaptureStateId, DEFAULT_CAPTURE_FRAME,
    ParseCaptureStateIdError, ParseRouteError, ParseScenarioError, PixelSize,
};

mod frame;
mod gate;
mod launch;
mod names;
mod reads;
mod types;

pub use launch::CaptureLaunchEnvBuilder;
pub use types::{
    CaptureEnv, CaptureEnvBuilder, CaptureEnvError, CaptureFrame, CaptureFrameGate,
    CaptureLaunchEnv, CaptureLaunchEnvError, CaptureLaunchEnvVar, CaptureRouteSource,
    ParseCaptureFrameError,
};

#[cfg(test)]
mod tests;
