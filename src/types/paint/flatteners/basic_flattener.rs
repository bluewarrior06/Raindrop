use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::vec::Vec2Du,
        paint::{
            flattener::Flattener,
            image::{Image, RGBA8Image},
            output::Output,
        },
    },
};

pub struct BasicFlattener;

impl Flattener for BasicFlattener {
    fn flatten(&self, output: &Output, image: &mut RGBA8Image) {
        let albedo = output.get_albedo();
        for x in 0..image.get_size().x {
            for y in 0..image.get_size().y {
                let pos = Vec2Du::new(x, y);

                if let Ok(pixel) = albedo.get(pos) {
                    image.set(pos, pixel);
                }
            }
        }
    }
}
