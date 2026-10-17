use crate::{traits::mapper::MapperCopy, types::{math::{color::RGBA8, vec::{Vec2Df, Vec2Du}}, paint::layers::{grid_layer::GridLayer, prop_layer::PropLayer}}};

#[derive(Default, Clone, Copy)]
pub struct GridLayerData {
	pub color: RGBA8
}
#[derive(Default, Clone)]
pub struct PropLayerData {
	pub pos: Vec2Df,
	pub rot: Vec2Df,
	pub scale: Vec2Df,
	pub id: String
}

/// Base type of any layer.
/// T is the type the layer stores.
pub trait BaseLayer<T>
where
    T: Clone,
{
}

#[derive(Debug)]
pub enum GridLayerError {
    SizeIsZero,
    IndexOutOfBounds,
}

pub trait GridBasedLayer: BaseLayer<GridLayerData> + MapperCopy<Vec2Du, GridLayerData, GridLayerError>
{
}

#[derive(Debug)]
pub enum PropLayerError {
    NothingToPop,
    IndexOutOfBounds,
}

pub trait PropBasedLayer : BaseLayer<PropLayerData>
{
    fn has_at(&self, at: usize) -> bool;
    fn push(&mut self, data: PropLayerData);
    fn pop(&mut self) -> Option<PropLayerError>;
    fn push_at(&mut self, data: PropLayerData, index: usize) -> Option<PropLayerError>;
    fn pop_at(&mut self, index: usize) -> Option<PropLayerError>;
    fn get(&mut self, index: usize) -> Result<PropLayerData, PropLayerError>;
}