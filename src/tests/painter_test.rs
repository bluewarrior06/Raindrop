#[cfg(test)]
pub fn test_grid_painter() {
    use crate::types::{
        math::{color::RGBA8, vec::Vec2Du},
        paint::{
            effects::block_effect::BlockEffect,
            output::Output,
            painter::{DrawablePainter, GridPainter},
        },
    };

    let grid_size = Vec2Du::new(100, 100);
    let physical_size = 10;
    let image_size = Vec2Du::new(grid_size.x * physical_size, grid_size.y * physical_size);

    let mut output: Output = Output::default();
    output.resize(image_size);

    let mut painter: GridPainter<RGBA8> = GridPainter::default();
    painter.get_layer_mut().set_physical_size(20);

    let mut effect = BlockEffect::default();

    painter.set_effect(Box::new(effect));

    painter.draw_onto_output(&mut output);
}
