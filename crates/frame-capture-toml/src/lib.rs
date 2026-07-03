//! TOML parsing helpers for shared `frame-capture` defaults.
//!
//! This crate parses package-local `frame-capture.toml` files for proc macros.
//! The accepted shape is an explicit `[default_size]` table with positive
//! integer `width` and `height` values.

use std::num::NonZeroU32;

use serde::Deserialize;
use thiserror::Error;

pub const DEFAULT_FILE_NAME: &str = "frame-capture.toml";

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PixelSize {
    width: NonZeroU32,
    height: NonZeroU32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct CaptureToml {
    pub default_size: PixelSize,
}

#[derive(Debug, Default, Deserialize)]
struct RawCaptureToml {
    default_size: Option<RawCaptureSize>,
}

#[derive(Debug, Default, Deserialize)]
struct RawCaptureSize {
    width: Option<u32>,
    height: Option<u32>,
}

#[derive(Debug, Error)]
pub enum CaptureTomlError {
    #[error("{0}")]
    Toml(#[from] toml::de::Error),
    #[error("expected `default_size.width` and `default_size.height`")]
    MissingDefaultSize,
    #[error("{key} must be explicit `width` and `height` keys")]
    InvalidSizeFormat { key: &'static str },
    #[error("{key} must be a positive integer")]
    InvalidWidth { key: &'static str },
    #[error("{key} must be a positive integer")]
    InvalidHeight { key: &'static str },
}

#[derive(Clone, Debug, Eq, Error, PartialEq)]
pub enum ParseSizeError {
    #[error("size must use WIDTHxHEIGHT")]
    MissingSeparator,
    #[error("size width must be a positive integer")]
    InvalidWidth,
    #[error("size height must be a positive integer")]
    InvalidHeight,
    #[error("size width must be greater than zero")]
    ZeroWidth,
    #[error("size height must be greater than zero")]
    ZeroHeight,
}

impl PixelSize {
    pub const fn new(width: u32, height: u32) -> Self {
        match Self::try_new(width, height) {
            Some(size) => size,
            None => panic!("pixel size dimensions must be greater than zero"),
        }
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
}

impl CaptureToml {
    pub fn parse(source: &str) -> Result<Self, CaptureTomlError> {
        let raw = toml::from_str(source)?;
        let default_size = find_size(&raw)?;

        Ok(Self { default_size })
    }
}

fn find_size(value: &RawCaptureToml) -> Result<PixelSize, CaptureTomlError> {
    let size = value
        .default_size
        .as_ref()
        .ok_or(CaptureTomlError::MissingDefaultSize)?;

    let width = size.width.ok_or(CaptureTomlError::InvalidWidth {
        key: "default_size.width",
    })?;
    let height = size.height.ok_or(CaptureTomlError::InvalidHeight {
        key: "default_size.height",
    })?;

    if width == 0 {
        return Err(CaptureTomlError::InvalidWidth {
            key: "default_size.width",
        });
    }
    if height == 0 {
        return Err(CaptureTomlError::InvalidHeight {
            key: "default_size.height",
        });
    }

    Ok(PixelSize::new(width, height))
}

pub fn parse_size(value: &str) -> Result<PixelSize, ParseSizeError> {
    let Some((width, height)) = value.split_once('x').or_else(|| value.split_once('X')) else {
        return Err(ParseSizeError::MissingSeparator);
    };

    let width = width
        .parse::<u32>()
        .map_err(|_| ParseSizeError::InvalidWidth)?;
    let height = height
        .parse::<u32>()
        .map_err(|_| ParseSizeError::InvalidHeight)?;

    if width == 0 {
        return Err(ParseSizeError::ZeroWidth);
    }
    if height == 0 {
        return Err(ParseSizeError::ZeroHeight);
    }

    Ok(PixelSize::new(width, height))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_root_default_size() {
        let config = CaptureToml::parse(
            r#"
                [default_size]
                width = 1600
                height = 900
            "#,
        )
        .unwrap();
        assert_eq!(config.default_size, PixelSize::new(1600, 900));
    }

    #[test]
    fn rejects_missing_default_size() {
        assert!(matches!(
            CaptureToml::parse(""),
            Err(CaptureTomlError::MissingDefaultSize)
        ));
    }

    #[test]
    fn parses_size_with_uppercase_separator() {
        assert_eq!(parse_size("1600X900"), Ok(PixelSize::new(1600, 900)));
    }

    #[test]
    fn rejects_malformed_size() {
        assert_eq!(parse_size("1600"), Err(ParseSizeError::MissingSeparator));
        assert_eq!(parse_size("wide x 900"), Err(ParseSizeError::InvalidWidth));
        assert_eq!(parse_size("1600xhigh"), Err(ParseSizeError::InvalidHeight));
        assert_eq!(parse_size("0x900"), Err(ParseSizeError::ZeroWidth));
        assert_eq!(parse_size("1600x0"), Err(ParseSizeError::ZeroHeight));
    }

    #[test]
    fn rejects_missing_size_fields() {
        assert!(matches!(
            CaptureToml::parse(
                r#"
[default_size]
height = 720
"#
            ),
            Err(CaptureTomlError::InvalidWidth {
                key: "default_size.width"
            })
        ));
        assert!(matches!(
            CaptureToml::parse(
                r#"
[default_size]
width = 1280
"#
            ),
            Err(CaptureTomlError::InvalidHeight {
                key: "default_size.height"
            })
        ));
    }

    #[test]
    fn maps_zero_sizes_to_existing_toml_errors() {
        assert!(matches!(
            CaptureToml::parse(
                r#"
                    [default_size]
                    width = 0
                    height = 900
                "#
            ),
            Err(CaptureTomlError::InvalidWidth {
                key: "default_size.width"
            })
        ));
        assert!(matches!(
            CaptureToml::parse(
                r#"
                    [default_size]
                    width = 1600
                    height = 0
                "#
            ),
            Err(CaptureTomlError::InvalidHeight {
                key: "default_size.height"
            })
        ));
    }
}
