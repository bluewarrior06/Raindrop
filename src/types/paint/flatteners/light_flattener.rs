use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::{color::RGBA, vec::Vec2Du},
        paint::{flattener::Flattener, image::RGBA8Image, output::Output},
    },
};

pub struct LightFlattener;

impl Flattener for LightFlattener {
    fn flatten(&self, output: &Output, image: &mut RGBA8Image) {
        let light = output.get_light();

        for x in 0..image.get_size().x {
            for y in 0..image.get_size().y {
                let pos = Vec2Du::new(x, y);

                let Ok(pixel) = image.get(pos) else {
                    return;
                };

                let Ok(light) = light.get(pos) else {
                    return;
                };

                let mut final_pixel = pixel;
                final_pixel.r = (pixel.r as f32 * light as f32).floor() as u8;
                final_pixel.g = (pixel.g as f32 * light as f32).floor() as u8;
                final_pixel.b = (pixel.b as f32 * light as f32).floor() as u8;
                final_pixel.r = pixel.a;

                image.set(pos, final_pixel);
            }
        }
    }
}
