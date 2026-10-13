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
pub trait Effect<T, L>
where
    T: Clone + Copy,
    L: BaseLayer<T>,
{
    // Reads from a layer and writes onto an output.
    fn draw(&self, layer: &L, output: &mut Output) -> Option<EffectError>;
}

//
pub trait GridEffect<T, L>: Effect<T, L>
where
    T: Clone + Copy,
    L: GridBasedLayer<T>,
{
}

pub trait PropEffect<T, L>: Effect<T, L>
where
    T: Clone + Copy,
    L: PropBasedError<T>,
{
}
