use crate::types::{
    assets::{asset_catalyst::AssetCatalyst, asset_manager::AssetManager}, math::color::RGBA8, paint::{
        flatten::Flatten, layers::grid_layer::GridLayer, output::Output, painter::PainterStack,
    },
};

#[derive(Default)]
pub struct EditorState {
	pub assets: AssetCatalyst,
	
	pub name: String,

    pub output: Output,
	pub painters: PainterStack,
    pub flatten: Flatten,
}
