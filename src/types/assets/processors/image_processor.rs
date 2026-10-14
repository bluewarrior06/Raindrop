use std::{fs::File, io::BufReader};

use png::{Decoder, Info};

use crate::{traits::mapper::MapperCopy, types::{assets::asset_processor::{AssetProcessor, AssetProcessorError}, data::image::{self, RGBA8Image}, math::{color::RGBA8, vec::Vec2Du}}};

#[derive(Default)]
pub struct RGBA8ImageProcessor;

impl AssetProcessor<RGBA8Image> for RGBA8ImageProcessor {
	fn process(&self, buf: &mut BufReader<File>) -> Result<RGBA8Image, AssetProcessorError> {
		let mut decoder = Decoder::new(buf);
		let info = match decoder.read_header_info() {
			Ok(info) => info.clone(),
			Err(err) => return Err(AssetProcessorError::PngDecodeError(err))
		};

		let mut image = match RGBA8Image::new(Vec2Du::new(1, 1)) {
			Ok(image) => image,
			Err(_) => return Err(AssetProcessorError::BadData)
		};

		let mut reader = match decoder.read_info() {
			Ok(reader) => reader,
			Err(err) => return Err(AssetProcessorError::PngDecodeError(err))
		};

		let buf_size = match reader.output_buffer_size() {
				Some(buf_size) => buf_size,
				None => return Err(AssetProcessorError::BadData)
			};

			let mut buf = vec![0; buf_size];

			let frame = match reader.next_frame(&mut buf) {
				Ok(frame) => frame,
				Err(err) => return Err(AssetProcessorError::PngDecodeError(err))
			};

			{
				Self::decode_buf_onto_image(&buf, &info, &mut image);
			}

		return Ok(image);
	}
}

impl RGBA8ImageProcessor {
	fn decode_buf_onto_image(buf: &[u8], info: &Info, image: &mut RGBA8Image) {
		let image_size = Vec2Du::new(info.width, info.height);
		image.resize(image_size);

		let mut cursor = 0;
		for x in 0..info.width {
			for y in 0..info.height {
				let color = RGBA8::new(
					buf[cursor + 0],
					buf[cursor + 1],
					buf[cursor + 2],
					buf[cursor + 3],
				);
				cursor+=4;

				let pos = Vec2Du::new(x, y);
				image.set(pos, color);
			}
		}
	}
}