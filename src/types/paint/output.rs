use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::vec::Vec2Du,
        paint::image::{ByteImage, Image, RGBA8Image},
    },
};

#[derive(Default)]
pub struct Output {
    albedo: RGBA8Image,
    light: ByteImage,
}

impl Output {
    pub fn get_albedo(&self) -> &RGBA8Image {
        &self.albedo
    }
    pub fn get_albedo_mut(&mut self) -> &mut RGBA8Image {
        &mut self.albedo
    }

    pub fn get_light(&self) -> &ByteImage {
        &self.light
    }
    pub fn get_light_mut(&mut self) -> &mut ByteImage {
        &mut self.light
    }

    pub fn resize(&mut self, size: Vec2Du) {
        self.albedo.resize(size);
    }
}
