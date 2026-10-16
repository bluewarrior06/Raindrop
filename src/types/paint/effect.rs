use crate::types::paint::{
    layer::{BaseLayer, GridBasedLayer, PropBasedError},
    output::Output,
};

pub enum EffectError {
    ReachedLayerOutOfBounds,
    ReachedOutputOutOfBounds,
}

/// Base type for all effects.
/// Reads from a layer, and draws onto an output.
pub trait Effect<T: Clone + Copy, L: BaseLayer<T>> {
    // Reads from a layer and writes onto an output.
    fn draw(&self, layer: &L, output: &mut Output) -> Option<EffectError>;
}

pub enum EffectType<T: Clone + Copy, L: BaseLayer<T>> {
	Grid(Box<dyn GridEffect<T, L>>),
	Prop(Box<dyn PropEffect<T, L>>)
}

//
pub trait GridEffect<T: Clone + Copy, L: BaseLayer<T>>: Effect<T, L>
where
    T: Clone + Copy,
    L: GridBasedLayer<T>,
{
}

pub trait PropEffect<T: Clone + Copy, L: BaseLayer<T>>: Effect<T, L>
where
    T: Clone + Copy,
    L: PropBasedError<T>,
{
}
