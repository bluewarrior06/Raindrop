use crate::{traits::mapper::MapperCopy, types::math::{color::RGBA8, vec::Vec2Du}};

pub struct Image {
	size: Vec2Du,

	data: Vec<RGBA8>
}

pub enum ImageError {
	SizeBelowZero,
	IndexOutOfBounds
}

impl Default for Image {
	fn default() -> Self {
		Self {
			size: Vec2Du::new(1, 1),
			data: vec![RGBA8::default()]
		}
	}
}

impl Image {
	pub fn new(size: Vec2Du) -> Result<Image, ImageError> {
		let mut image = Image::default();
		
		if let Some(error) = image.resize(size) {
			return Err(error);
		}
		else {
			return Ok(image);
		}
	}

	pub fn to_bytes(&self) -> Vec<u8> {
		let area = (self.size.x * self.size.y) as usize;

		let mut bytes = Vec::with_capacity(area);

		for i in 0..area {
			let color = self.data[i];
			bytes.push(color.r);
			bytes.push(color.g);
			bytes.push(color.b);
			bytes.push(color.a);
		}

		return bytes;
	}
}

impl MapperCopy<Vec2Du, RGBA8, ImageError> for Image {
	fn is_in_bounds(&self, pos: Vec2Du) -> Option<ImageError> {
		if pos.x >= self.size.x || pos.y >= self.size.y {
			Some(ImageError::IndexOutOfBounds)
		} else {
			None
		}
	}
	fn is_size_valid(size: Vec2Du) -> Option<ImageError> {
		if size.x == 0 || size.y == 0 {
			Some(ImageError::SizeBelowZero)
		}
		else {
			None
		}
	}

	fn set(&mut self, pos: Vec2Du, element: RGBA8) -> Option<ImageError> {
		if let Some(err) = self.is_in_bounds(pos) {
			return Some(err);
		}
		else {
			let index = self.index_of(pos);
			self.data[index] = element;
			return None;
		}
	}
	fn get(&self, pos: Vec2Du) -> Result<RGBA8, ImageError> {
		if let Some(err) = self.is_in_bounds(pos) {
			return Err(err);
		}
		else {
			let index = self.index_of(pos);
			return Ok(self.data[index]);
		}
	}

	fn index_of(&self, pos: Vec2Du) -> usize {
		return (pos.x + pos.y * self.size.x) as usize;
	}
	fn resize(&mut self, size: Vec2Du) -> Option<ImageError> {
		if let Some(err) = Self::is_size_valid(size) {
			return Some(err);
		}

		self.size = size;
		let area: usize = (self.size.x * self.size.y) as usize;
		self.data.resize(area, RGBA8::default());

		return None;
	}
	fn get_size(&self) -> Vec2Du {
		return self.size;
	}
} 