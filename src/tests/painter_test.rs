use std::path::PathBuf;

use noise::{NoiseFn, Perlin};

use crate::{traits::mapper::MapperCopy, types::{assets::asset_catalyst::AssetCatalyst, math::{color::RGBA8, vec::Vec2Du}, paint::{effects::block_effect::BlockEffect, flatten::Flatten, flatteners::basic_flattener::BasicFlattener, layer::GridLayerData, output::Output, painter::{GridPainter, PainterStack}}}};

#[test]
pub fn painter_stack_test() {
	let grid_size = Vec2Du::new(100, 100);
	let physical_size = 20;
	let image_size = Vec2Du::new(grid_size.x * physical_size, grid_size.y * physical_size);
	
	let mut output = Output::default();
	output.resize(image_size);

	let mut painter = GridPainter::default();
	painter.get_layer_mut().resize(grid_size);
	painter.get_layer_mut().set_physical_size(physical_size);

	let noise = Perlin::default();
	for x in 0..grid_size.x {
		for y in 0..grid_size.y {
			let pos = Vec2Du::new(x, y);
			let intensity = noise.get([x as f64 / 30.0, y as f64 / 30.0]);
			let intensity_col = (intensity * u8::MAX as f64) as u8;
			let mut data = GridLayerData::default();
			data.color = RGBA8::new(intensity_col, intensity_col, intensity_col, intensity_col);
			painter.get_layer_mut().set(pos, data);
		}
	}

	let effect = BlockEffect::default();
	painter.set_effect(Box::new(effect));

	let mut painter_stack = PainterStack::default();
	painter_stack.push_grid_painter(painter);

	let mut assets = AssetCatalyst::default();

	painter_stack.draw_onto_output(&mut assets, &mut output);

	let mut flatten = Flatten::default();
	flatten.push_flattener(Box::new(BasicFlattener::default()));

	let image = flatten.flatten(&output);

	image.save_as_png(&PathBuf::from("painter_stack_test.png".to_string()));
}