use std::{fs::File, io::BufWriter, path::Path};

use crate::{
    traits::mapper::MapperCopy,
    types::math::{color::RGBA8, vec::Vec2Du},
};

pub type RGBA8Image = Image<RGBA8>;
pub type IndexImage = Image<u32>;
pub type ByteImage = Image<u8>;

pub struct Image<T: Default + Clone + Copy> {
    size: Vec2Du,
    data: Vec<T>,
}

#[derive(Debug)]
pub enum ImageError {
    SizeBelowZero,
    IndexOutOfBounds,
}

impl<T: Default + Clone + Copy> Default for Image<T> {
    fn default() -> Self {
        Self {
            size: Vec2Du::new(1, 1),
            data: vec![Default::default()],
        }
    }
}

impl<T: Default + Clone + Copy> Image<T> {
    pub fn new(size: Vec2Du) -> Result<Image<T>, ImageError> {
        let mut image = Image::default();

        if let Some(error) = image.resize(size) {
            return Err(error);
        } else {
            return Ok(image);
        }
    }
}

impl Image<RGBA8> {
    /// This code needs a rewrite since the implementation is copied straight from the png cargo example, (I don't know what it does!)
    pub fn save_as_png(&self, path: &Path) -> bool {
        let file_result = File::create(path);
        let file = match file_result {
            Ok(file) => file,
            Err(_err) => return false,
        };

        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, self.size.x, self.size.y); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        let data = self.to_bytes();
        writer.write_image_data(&data).unwrap();

        return true;
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

impl<T: Default + Clone + Copy> MapperCopy<Vec2Du, T, ImageError> for Image<T> {
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
        } else {
            None
        }
    }

    fn set(&mut self, pos: Vec2Du, element: T) -> Option<ImageError> {
        if let Some(err) = self.is_in_bounds(pos) {
            return Some(err);
        } else {
            let index = self.index_of(pos);
            self.data[index] = element;
            return None;
        }
    }
    fn get(&self, pos: Vec2Du) -> Result<T, ImageError> {
        if let Some(err) = self.is_in_bounds(pos) {
            return Err(err);
        } else {
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
        self.data.resize(area, Default::default());

        return None;
    }
    fn get_size(&self) -> Vec2Du {
        return self.size;
    }
}
