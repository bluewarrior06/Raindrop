use std::io::BufWriter;

use noise::{NoiseFn, Perlin};

use crate::{
    traits::mapper::MapperCopy,
    types::{
        math::{
            color::RGBA8,
            vec::{Vec2D, Vec2Du},
        },
        paint::{
            effects::block_effect::BlockEffect,
            flattened::Flattened,
            flatteners::basic_flattener::BasicFlattener,
            layers::grid_layer::GridLayer,
            output::Output,
            painter::{DrawablePainter, Painter},
        },
    },
};

use std::fs::File;
use std::path::Path;

pub mod traits;
pub mod types;

/// Splash is an editor for generating images for games.
/// Images are created through a collection of stages linearly:
/// 1. Draw onto layers.
/// There are two different types of layers: Tiled layers and Prop layers. Tiled layers store information in a fixed grid; Prop layers store information that isn't locked onto a grid.
/// 2. Feed these layers into effects to draw onto an image.
/// 3. Save the resulting image onto the disk.

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let size = Vec2Du::new(48, 27);
    let physical_size: u32 = 20;
    let image_size = Vec2Du::new(size.x * physical_size as u32, size.y * physical_size as u32);

    let mut output = Output::default();
    output.resize(image_size);

    let mut block_effect = BlockEffect::default();
    block_effect.physical_size = physical_size;

    let mut painter = Painter::<RGBA8, GridLayer<RGBA8>>::default();
    painter.push_effect(Box::new(block_effect));

    let layer = painter.get_layer_mut();
    layer.resize(size);

    let noise_gen = Perlin::new(1);

    for x in 0..size.x {
        for y in 0..size.y {
            let value = noise_gen.get([x as f64 / 32.0, y as f64 / 32.0, 0.0]);
            let intensity: u8 = (value * u8::MAX as f64) as u8;
            println!("{intensity}");

            layer.set(
                Vec2D::new(x, y),
                RGBA8 {
                    r: intensity,
                    g: intensity,
                    b: intensity,
                    a: intensity,
                },
            );
        }
    }

    painter.draw_onto_output(&mut output);

    let mut flattened = Flattened::default();
    flattened.add_flattener(Box::new(BasicFlattener));
    let flattened_image = flattened.flatten(&output);

    {
        // For reading and opening files
        let path = Path::new(r"image.png");
        let file = File::create(path).unwrap();
        let ref mut w = BufWriter::new(file);

        let mut encoder = png::Encoder::new(w, image_size.x, image_size.y); // Width is 2 pixels and height is 1.
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        encoder.set_source_gamma(png::ScaledFloat::from_scaled(45455)); // 1.0 / 2.2, scaled by 100000
        encoder.set_source_gamma(png::ScaledFloat::new(1.0 / 2.2)); // 1.0 / 2.2, unscaled, but rounded
        let source_chromaticities = png::SourceChromaticities::new(
            // Using unscaled instantiation here
            (0.31270, 0.32900),
            (0.64000, 0.33000),
            (0.30000, 0.60000),
            (0.15000, 0.06000),
        );
        encoder.set_source_chromaticities(source_chromaticities);
        let mut writer = encoder.write_header().unwrap();

        let data = flattened_image.to_bytes();
        writer.write_image_data(&data).unwrap(); // Save
    }

    return Ok(());
}
