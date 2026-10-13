use crate::types::{math::color::{RGB, RGBA, RGBA8}, paint::{effect::{Effect, EffectError}, layers::grid_layer::GridLayer, output::Output}};

pub struct GradientEffect {

}

impl Effect<RGBA8, GridLayer<RGBA8>> for GradientEffect {
	fn draw(&self, layer: &GridLayer<RGBA8>, output: &mut Output) -> Option<EffectError> {
		None
	}
}