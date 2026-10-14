use crate::types::{
    math::color::RGBA8,
    paint::{
        effect::{Effect, EffectError},
        layers::grid_layer::GridLayer,
        output::Output,
    },
};

pub struct GradientEffect {}

impl Effect<RGBA8, GridLayer<RGBA8>> for GradientEffect {
    fn draw(&self, _layer: &GridLayer<RGBA8>, _output: &mut Output) -> Option<EffectError> {
        None
    }
}
