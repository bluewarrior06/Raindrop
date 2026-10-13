use std::marker::PhantomData;

use crate::types::paint::{effect::Effect, layer::BaseLayer, output::Output};

pub type GridPainter<T, L> = Painter<T, L>;

/// Paints onto an Output using a layer and effects.
#[derive(Default)]
pub struct Painter<T, L>
where
    T: Clone + Copy,
    L: BaseLayer<T>,
{
    layer: L,
    effects: Vec<Box<dyn Effect<T, L>>>,

    phantom: PhantomData<T>,
}

pub trait DrawablePainter {
    fn draw_onto_output(&self, _output: &mut Output) {}
}

impl<T, L> DrawablePainter for Painter<T, L>
where
    T: Clone + Copy,
    L: BaseLayer<T>,
{
    fn draw_onto_output(&self, output: &mut Output) {
        for effect in self.effects.iter() {
            effect.draw(&self.layer, output);
        }
    }
}

impl<T, L> Painter<T, L>
where
    T: Clone + Copy,
    L: BaseLayer<T>,
{
    pub fn get_layer_mut(&mut self) -> &mut L {
        return &mut self.layer;
    }

    pub fn push_effect(&mut self, effect: Box<dyn Effect<T, L>>) {
        self.effects.push(effect);
    }
}
