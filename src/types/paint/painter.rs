use std::marker::PhantomData;

use crate::types::{assets::asset_catalyst::AssetCatalyst, paint::{
    effect::Effect, layer::{BaseLayer, GridLayerData, PropLayerData, PropLayerError}, layers::{grid_layer::GridLayer, prop_layer::PropLayer}, output::Output,
}};

pub type GridPainter = Painter<GridLayerData, GridLayer>;
pub type PropPainter = Painter<PropLayerData, PropLayer>;

/// Paints onto an Output using a layer and effects.
/// The Layer informs the Effects how to draw onto an Output.
#[derive(Default)]
pub struct Painter<T: Clone, L: BaseLayer<T>> {
    layer: L,
    effect: Option<Box<dyn Effect<T, L>>>,

    phantom: PhantomData<T>,
}

impl<T: Clone, L: BaseLayer<T>> Painter<T, L> {
	pub fn set_effect(&mut self, effect: Box<dyn Effect<T, L>>) {
		self.effect = Some(effect);
	}
	pub fn clear_effect(&mut self) {
		self.effect = None;
	}

	pub fn get_layer(&self) -> &L {
		&self.layer
	}
	pub fn get_layer_mut(&mut self) -> &mut L {
		&mut self.layer
	}
}

impl<T: Clone, L: BaseLayer<T>> Painter<T, L>  {
    fn draw_onto_output(&self, assets: &AssetCatalyst, output: &mut Output) {
        if let Some(effect) = &self.effect {
            effect.draw(&self.layer, assets, output);
        }
    }
}

#[derive(Default)]
pub struct PainterStack {
    painters: Vec<PainterType>,
}

pub enum PainterType {
    Grid(GridPainter),
	Prop(PropPainter)

}

impl PainterStack {
    pub fn push_grid_painter(&mut self, grid_painter: GridPainter) {
        self.painters.push(PainterType::Grid(grid_painter));
    }
	pub fn push_prop_painter(&mut self, prop_painter: PropPainter) {
        self.painters.push(PainterType::Prop(prop_painter));
    }
	
    pub fn pop_painter(&mut self) {
        self.painters.pop();
    }

    pub fn draw_onto_output(&self, assets: &AssetCatalyst, output: &mut Output) {
        for painter in self.painters.iter() {
			match painter {
				PainterType::Grid(painter) => painter.draw_onto_output(assets, output),
				PainterType::Prop(painter) => painter.draw_onto_output(assets, output),
			}
        }
    }
}
