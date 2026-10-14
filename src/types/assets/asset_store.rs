use crate::types::{assets::{asset_manager::AssetManager, asset_storage::AssetStorage, processors::image_processor::RGBA8ImageProcessor}, data::image::RGBA8Image, math::color::RGBA8};

/// Stores different asset managers that can be retrieved.
#[derive(Default)]
pub struct AssetStores {
	rgba8image: AssetManager<RGBA8Image, RGBA8ImageProcessor>
}

/// Used to access an asset manager.
pub trait AssetStore<M> {
	fn get(&self) -> &M;
	fn get_mut(&mut self) -> &mut M;
}

impl AssetStore<AssetManager<RGBA8Image, RGBA8ImageProcessor>> for AssetStores {
	fn get(&self) -> &AssetManager<RGBA8Image, RGBA8ImageProcessor> {
		&self.rgba8image
	}
	fn get_mut(&mut self) -> &mut AssetManager<RGBA8Image, RGBA8ImageProcessor> {
		&mut self.rgba8image
	}
}