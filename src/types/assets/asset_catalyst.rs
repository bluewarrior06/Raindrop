use crate::types::{assets::{asset_manager::AssetManager, asset_processor::AssetProcessor, processors::image_processor::RGBA8ImageProcessor}, data::image::RGBA8Image};

#[derive(Default)]
pub struct AssetCatalyst {
	images: AssetManager<RGBA8Image, RGBA8ImageProcessor>
}

impl AssetCatalyst {
	pub fn get_images(&self) -> &AssetManager<RGBA8Image, RGBA8ImageProcessor> {
		&self.images
	}
	pub fn get_images_mut(&mut self) -> &mut AssetManager<RGBA8Image, RGBA8ImageProcessor> {
		&mut self.images
	}
}