use crate::{traits::mapper::MapperCopy, types::math::vec::Vec2Du};

/// Base type of any layer.
/// T is the type the layer stores.
pub trait BaseLayer<T>
where
    T: Clone + Copy,
{
}

#[derive(Debug)]
pub enum GridLayerError {
    SizeIsZero,
    IndexOutOfBounds,
}

/// Stores T in a grid.
pub trait GridBasedLayer<T>: BaseLayer<T> + MapperCopy<Vec2Du, T, GridLayerError>
where
    T: Clone + Copy,
{
}

#[derive(Debug)]
pub enum PropLayerError {
    NothingToPop,
    IndexOutOfBounds,
}

/// Stores T in arbitrary positions.
pub trait PropBasedError<T>: BaseLayer<T>
where
    T: Clone + Copy,
{
	fn has_at(&self, at: usize) -> bool;
    fn push(&mut self, data: T);
    fn pop(&mut self) -> Option<PropLayerError>;
    fn push_at(&mut self, data: T, index: usize) -> Option<PropLayerError>;
    fn pop_at(&mut self, index: usize) -> Option<PropLayerError>;
    fn get(&mut self, index: usize) -> Result<T, PropLayerError>;
}
