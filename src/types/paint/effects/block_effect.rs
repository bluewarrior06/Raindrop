use crate::{
    traits::mapper::MapperCopy, types::{
        assets::asset_catalyst::AssetCatalyst, math::{color::RGBA8, vec::Vec2Du}, paint::{
            effect::{Effect, EffectError, GridEffect}, layer::GridLayerData, layers::grid_layer::GridLayer, output::Output,
        },
    },
};

#[derive(Default)]
pub struct BlockEffect {}

impl Effect<GridLayerData, GridLayer> for BlockEffect {
    fn draw(&self, layer: &GridLayer, assets: &AssetCatalyst, output: &mut Output) -> Option<EffectError> {
        let albedo = output.get_albedo_mut();

        for x in 0..layer.get_size().x {
            for y in 0..layer.get_size().y {
                let pos = Vec2Du::new(x, y);
                let Ok(color) = layer.get(pos) else {
                    return Some(EffectError::ReachedLayerOutOfBounds);
                };

                for px in 0..layer.get_physical_size() {
                    for py in 0..layer.get_physical_size() {
                        let ppos = Vec2Du::new(
                            pos.x * layer.get_physical_size() + px,
                            pos.y * layer.get_physical_size() + py,
                        );
                        let output_err = albedo.set(ppos, color.color);

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

impl GridEffect for BlockEffect {}
