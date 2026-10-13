use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::{color::RGBA8, vec::Vec2Du},
        paint::{
            effect::{Effect, EffectError, GridEffect},
            layers::grid_layer::GridLayer,
            output::Output,
        },
    },
};

#[derive(Default)]
pub struct BlockEffect {
    pub physical_size: u32,
}

impl Effect<RGBA8, GridLayer<RGBA8>> for BlockEffect {
    fn draw(&self, layer: &GridLayer<RGBA8>, output: &mut Output) -> Option<EffectError> {
        let albedo = output.get_albedo_mut();

        for x in 0..layer.get_size().x {
            for y in 0..layer.get_size().y {
                let pos = Vec2Du::new(x, y);
                let Ok(color) = layer.get(pos) else {
                    return Some(EffectError::ReachedLayerOutOfBounds);
                };

                for px in 0..self.physical_size {
                    for py in 0..self.physical_size {
                        let ppos = Vec2Du::new(
                            pos.x * self.physical_size + px as u32,
                            pos.y * self.physical_size + py as u32,
                        );
                        let output_err = albedo.set(ppos, color);

                        if let Some(_) = output_err {
                            return Some(EffectError::ReachedOutputOutOfBounds);
                        }
                    }
                }
            }
        }
        return None;
    }
}

impl GridEffect<RGBA8, GridLayer<RGBA8>> for BlockEffect {}
