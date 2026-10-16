use std::marker::PhantomData;

use crate::types::paint::{
    effect::Effect,
    layer::{BaseLayer, PropLayerError},
    layers::grid_layer::GridLayer,
    output::Output,
};

pub type GridPainter<T> = Painter<T, GridLayer<T>>;

/// Paints onto an Output using a layer and effects.
/// The Layer informs the Effects how to draw onto an Output.
#[derive(Default)]
pub struct Painter<T: Clone + Copy, L: BaseLayer<T>> {
    layer: L,
    effect: Option<Box<dyn Effect<T, L>>>,

    phantom: PhantomData<T>,
}

pub trait DrawablePainter {
    fn draw_onto_output(&self, output: &mut Output);
}

impl<T: Clone + Copy, L: BaseLayer<T>> Painter<T, L> {
    pub fn get_layer_mut(&mut self) -> &mut L {
        return &mut self.layer;
    }

    pub fn set_effect(&mut self, effect: Box<dyn Effect<T, L>>) {
        self.effect = Some(effect);
    }
    pub fn clear_effect(&mut self) {
        self.effect = None;
    }
}

impl<T: Clone + Copy, L: BaseLayer<T>> DrawablePainter for Painter<T, L> {
    fn draw_onto_output(&self, output: &mut Output) {
        if let Some(effect) = &self.effect {
            effect.draw(&self.layer, output);
        }
    }
}

#[derive(Default)]
pub struct PainterStack {
    painters: Vec<Box<dyn DrawablePainter>>,
}

pub enum PainterType<T: Default + Clone + Copy> {
    Grid(GridPainter<T>),
}

impl PainterStack {
    pub fn push_painter(&mut self, painter: Box<dyn DrawablePainter>) {
        self.painters.push(painter);
    }
    pub fn pop_painter(&mut self) {
        self.painters.pop();
    }

    pub fn draw_onto_output(&self, output: &mut Output) {
        for painter in self.painters.iter() {
            painter.draw_onto_output(output);
        }
    }
}
