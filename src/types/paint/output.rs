use crate::{
    traits::mapper::MapperCopy,
    types::{math::vec::Vec2Du, paint::image::Image},
};

#[derive(Default)]
pub struct Output {
    albedo: Image,
}

impl Output {
    pub fn get_albedo(&self) -> &Image {
        return &self.albedo;
    }
    pub fn get_albedo_mut(&mut self) -> &mut Image {
        return &mut self.albedo;
    }

    pub fn resize(&mut self, size: Vec2Du) {
        self.albedo.resize(size);
    }
}
