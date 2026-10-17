use crate::types::{
    assets::asset_catalyst::AssetCatalyst, math::color::RGBA8, paint::{
        effect::{Effect, EffectError}, layer::GridLayerData, layers::grid_layer::GridLayer, output::Output,
    },
};

pub struct GradientEffect {}

impl Effect<GridLayerData, GridLayer> for GradientEffect {
    fn draw(&self, _layer: &GridLayer, assets: &AssetCatalyst, _output: &mut Output) -> Option<EffectError> {
        None
    }
}
