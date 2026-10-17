use iced::wgpu::Label;

use crate::types::{assets::asset_catalyst::AssetCatalyst, paint::{
    layer::{BaseLayer, GridBasedLayer, GridLayerData, PropBasedLayer, PropLayerData}, layers::{grid_layer::GridLayer, prop_layer::PropLayer}, output::Output,
}};

pub enum EffectError {
    ReachedLayerOutOfBounds,
    ReachedOutputOutOfBounds,
}

/// Base type for all effects.
/// Reads from a layer, and draws onto an output.
pub trait Effect<T: Clone, L: BaseLayer<T>> {
    // Reads from a layer and writes onto an output.
    fn draw(&self, layer: &L, assets: &AssetCatalyst, output: &mut Output) -> Option<EffectError>;
}

//
pub trait GridEffect: Effect<GridLayerData, GridLayer> {}

pub trait PropEffect: Effect<PropLayerData, PropLayer> {}
