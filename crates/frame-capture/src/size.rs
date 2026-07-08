use std::{fmt, num::NonZeroU32, str::FromStr};

use frame_capture_toml::ParseSizeError;
use serde::Serialize;
use thiserror::Error;

#[derive(
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    PartialEq,
    serde_with::DeserializeFromStr,
    serde_with::SerializeDisplay,
)]
pub struct PixelSize {
    width: NonZeroU32,
    height: NonZeroU32,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
#[cfg_attr(feature = "json-schema", derive(schemars::JsonSchema))]
pub struct CapturePixelSizeInfo {
    width: u32,
    height: u32,
    label: String,
}

impl CapturePixelSizeInfo {
    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParsePixelSizeError {
    #[error("capture size `{value}` must use WIDTHxHEIGHT")]
    MissingSeparator { value: String },
    #[error("capture size `{value}` has an invalid width")]
    InvalidWidth { value: String },
    #[error("capture size `{value}` has an invalid height")]
    InvalidHeight { value: String },
    #[error("capture size `{value}` width must be greater than zero")]
    ZeroWidth { value: String },
    #[error("capture size `{value}` height must be greater than zero")]
    ZeroHeight { value: String },
}

impl PixelSize {
    /// Creates a positive pixel size.
    ///
    /// # Panics
    ///
    /// Panics when either dimension is zero. Use [`Self::try_new`] when zero is
    /// recoverable input.
    pub const fn new(width: u32, height: u32) -> Self {
        match Self::try_new(width, height) {
            Some(size) => size,
            None => panic!("pixel size dimensions must be greater than zero"),
        }
    }

    pub const fn from_nonzero(width: NonZeroU32, height: NonZeroU32) -> Self {
        Self { width, height }
    }

    pub const fn try_new(width: u32, height: u32) -> Option<Self> {
        let Some(width) = NonZeroU32::new(width) else {
            return None;
        };
        let Some(height) = NonZeroU32::new(height) else {
            return None;
        };

        Some(Self { width, height })
    }

    pub const fn width(self) -> u32 {
        self.width.get()
    }

    pub const fn height(self) -> u32 {
        self.height.get()
    }

    pub const fn dimensions(self) -> (u32, u32) {
        (self.width(), self.height())
    }

    /// Scales `default_size` so its longest edge equals `long_edge`.
    ///
    /// # Panics
    ///
    /// This function relies on `long_edge` being nonzero and clamps scaled
    /// dimensions to at least one pixel, so it does not panic for valid inputs.
    pub fn from_long_edge(default_size: Self, long_edge: NonZeroU32) -> Self {
        let long_edge = long_edge.get();
        if default_size.width() >= default_size.height() {
            Self {
                width: NonZeroU32::new(long_edge).expect("long edge must be greater than zero"),
                height: NonZeroU32::new(scaled_dimension(
                    long_edge,
                    default_size.height(),
                    default_size.width(),
                ))
                .expect("scaled dimension must be greater than zero"),
            }
        } else {
            Self {
                width: NonZeroU32::new(scaled_dimension(
                    long_edge,
                    default_size.width(),
                    default_size.height(),
                ))
                .expect("scaled dimension must be greater than zero"),
                height: NonZeroU32::new(long_edge).expect("long edge must be greater than zero"),
            }
        }
    }
}

impl FromStr for PixelSize {
    type Err = ParsePixelSizeError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let size = frame_capture_toml::parse_size(value)
            .map_err(|error| parse_pixel_size_error(value, error))?;
        Ok(Self::new(size.width(), size.height()))
    }
}

fn parse_pixel_size_error(value: &str, error: ParseSizeError) -> ParsePixelSizeError {
    let value = value.to_owned();
    match error {
        ParseSizeError::MissingSeparator => ParsePixelSizeError::MissingSeparator { value },
        ParseSizeError::InvalidWidth => ParsePixelSizeError::InvalidWidth { value },
        ParseSizeError::InvalidHeight => ParsePixelSizeError::InvalidHeight { value },
        ParseSizeError::ZeroWidth => ParsePixelSizeError::ZeroWidth { value },
        ParseSizeError::ZeroHeight => ParsePixelSizeError::ZeroHeight { value },
    }
}

impl fmt::Display for PixelSize {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(formatter, "{}x{}", self.width(), self.height())
    }
}

impl From<PixelSize> for CapturePixelSizeInfo {
    fn from(size: PixelSize) -> Self {
        Self {
            width: size.width(),
            height: size.height(),
            label: size.to_string(),
        }
    }
}

fn scaled_dimension(long_edge: u32, dimension: u32, base: u32) -> u32 {
    ((u64::from(long_edge) * u64::from(dimension) + u64::from(base / 2)) / u64::from(base)).max(1)
        as u32
}
