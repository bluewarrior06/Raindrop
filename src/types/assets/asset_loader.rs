use std::{fmt::Error, fs::OpenOptions, io::{BufReader, Read}, marker::PhantomData, path::{Path, PathBuf}};

use crate::types::assets::asset_processor::{AssetProcessor, AssetProcessorError};

#[derive(Debug)]
pub enum AssetLoadError {
	FileError(std::io::Error),
	ProcessError(AssetProcessorError)
}

/// Manages loading in an asset from various locations.
/// The asset loader loads in the data of an asset, while an asset processor turns that data into a potentially usable asset.
#[derive(Default)]
pub struct AssetLoader<T, P: AssetProcessor<T>> {
	processor: P,
	_p: PhantomData<T>
}

impl<T, P: AssetProcessor<T>> AssetLoader<T, P> {
	pub fn load_from_disk(&self, path: &PathBuf) -> Result<T, AssetLoadError> {
		let file = OpenOptions::new().read(true).open(path);
		
		match file {
			Ok(file) => {
				let mut buf = BufReader::new(file);
				let processed = self.processor.process(&mut buf);
				match processed {
					Ok(asset) => return Ok(asset),
					Err(err) => return Err(AssetLoadError::ProcessError(err))
				}
			},
			Err(err) => return Err(AssetLoadError::FileError(err))
		};
	}
}