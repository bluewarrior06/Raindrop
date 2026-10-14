use std::{fs::File, io::BufReader};

#[derive(Debug)]
pub enum AssetProcessorError {
	PngDecodeError(png::DecodingError),
	BadData
}

pub trait AssetProcessor<T> {
	fn process(&self, buf: &mut BufReader<File>) -> Result<T, AssetProcessorError>;
}