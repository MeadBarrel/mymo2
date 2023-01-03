use std::{collections::HashMap, fs::DirEntry, io::Read, path::PathBuf};
use eframe::epaint::Color32;
use state::Storage;
use egui_extras::RetainedImage;
use error_stack::{ResultExt, Result, IntoReport};
use mymo::model::Skill;
use mymo::indexmap::IndexMap;

pub struct Colors {
    pub frame_color_std: Color32,
    pub frame_color_impossible: Color32,
    pub frame_color_ineffective: Color32,
}

pub static IMAGES: Storage<HashMap<String, RetainedImage>> = Storage::new();
pub static COLORS: Storage<Colors> = Storage::new();
pub static SKILLS: Storage<IndexMap<String, Skill>> = Storage::new();

impl Default for Colors {
    fn default() -> Self {
        Self {
            frame_color_std: Color32::from_additive_luminance(25),
            frame_color_impossible: Color32::from_rgb_additive(80, 0, 0),
            frame_color_ineffective: Color32::from_rgb_additive(80, 80, 0)
        }
    }
}

#[derive(Clone, Copy, Debug, thiserror::Error)]
#[error("Failed to load images")]
pub struct LoadImagesError;

#[derive(Clone, Debug, thiserror::Error)]
#[error("Failed to load an image at {0}")]
pub struct LoadImageError(PathBuf);

pub fn load_images(path: &str) -> Result<HashMap<String, RetainedImage>, LoadImagesError> {
    std::fs::read_dir(path)
        .into_report().change_context(LoadImagesError)?
        .collect::<std::result::Result<Vec<DirEntry>, std::io::Error>>()
        .into_report().change_context(LoadImagesError)?
        .into_iter()
        .filter(|entry| entry.path().is_file())
        .filter(|entry| entry.path().extension().unwrap_or_default() == "png" )
        .map(|entry| {
            let name = entry.file_name().into_string()
                .map_err(|_| LoadImagesError).into_report()?;
            let file_path = entry.path();
            let image = load_image(&name, &file_path)
                .change_context(LoadImagesError)?;

            Ok((name, image))

        }).collect()
}

pub fn load_image(name: &str, path: &PathBuf) -> Result<RetainedImage, LoadImageError> {
    let mut bytes: Vec<u8> = Vec::default();
    let err = LoadImageError(path.clone());

    std::fs::File::open(path)
        .into_report().change_context(err.clone())?
        .read_to_end(&mut bytes)
        .into_report().change_context(err.clone())?;
    let result = RetainedImage::from_image_bytes(name, &bytes)
        .map_err(|_| err)
        .into_report()
        .attach_printable_lazy(|| "Could not load image from bytes")?;

    Ok( result )
}