use crate::traits::mapper::MapperCopy;

#[test]
pub fn test_image_loading() {
    use std::path::PathBuf;

use crate::types::{assets::{asset_loader::AssetLoader, asset_processor::AssetProcessor, processors::image_processor::RGBA8ImageProcessor}, data::image::RGBA8Image};

	let image_loader: AssetLoader<RGBA8Image, RGBA8ImageProcessor> = AssetLoader::default();

	let image = image_loader.load_from_disk(&PathBuf::from("image.png"));
	println!("{:?}", image.unwrap().get_size());
}