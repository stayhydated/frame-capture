use std::path::{Path, PathBuf};

use bevy::{prelude::*, render::view::screenshot::ScreenshotCaptured};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum CaptureSaveError {
    #[error("failed to convert Bevy screenshot image: {source}")]
    ImageConversion {
        #[from]
        source: bevy::image::IntoDynamicImageError,
    },
    #[error("unknown image format for {path}: {source}")]
    UnknownImageFormat {
        path: PathBuf,
        source: image::ImageError,
    },
    #[error("failed to save screenshot to {path}: {source}")]
    Save {
        path: PathBuf,
        source: image::ImageError,
    },
}

pub(crate) fn save_capture_to_disk(
    path: impl AsRef<Path>,
    captured: On<ScreenshotCaptured>,
) -> Result<(), CaptureSaveError> {
    let path = path.as_ref().to_owned();
    let image = captured.image.clone();
    let dynamic = image.try_into_dynamic()?;
    let image = dynamic.to_rgb8();

    let format = image::ImageFormat::from_path(&path).map_err(|source| {
        CaptureSaveError::UnknownImageFormat {
            path: path.clone(),
            source,
        }
    })?;
    image
        .save_with_format(&path, format)
        .map_err(|source| CaptureSaveError::Save {
            path: path.clone(),
            source,
        })?;
    println!("Screenshot saved to {}", path.display());

    Ok(())
}
