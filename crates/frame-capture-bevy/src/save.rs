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
    save_image_to_disk(path, captured.image.clone())
}

fn save_image_to_disk(path: PathBuf, image: Image) -> Result<(), CaptureSaveError> {
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

#[cfg(test)]
mod tests {
    use bevy::{
        asset::RenderAssetUsages,
        render::render_resource::{Extent3d, TextureDimension, TextureFormat},
    };

    use super::*;

    fn test_image() -> Image {
        Image::new_fill(
            Extent3d {
                width: 1,
                height: 1,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            &[255, 0, 0, 255],
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        )
    }

    #[test]
    fn save_reports_image_conversion_errors() {
        let mut image = Image::new_target_texture(1, 1, TextureFormat::Rgba8UnormSrgb, None);
        image.data = None;
        assert!(matches!(
            save_image_to_disk(PathBuf::from("capture.png"), image),
            Err(CaptureSaveError::ImageConversion { .. })
        ));
    }

    #[test]
    fn save_reports_unknown_formats_and_io_errors() {
        assert!(matches!(
            save_image_to_disk(PathBuf::from("capture.unknown"), test_image()),
            Err(CaptureSaveError::UnknownImageFormat { .. })
        ));

        let path = std::env::temp_dir()
            .join("frame-capture-missing-parent")
            .join("capture.png");
        let _ = std::fs::remove_dir_all(path.parent().unwrap());
        assert!(matches!(
            save_image_to_disk(path, test_image()),
            Err(CaptureSaveError::Save { .. })
        ));
    }
}
