use crate::types::{
    math::color::RGBA8,
    paint::{
        flatten::Flatten,
        output::Output,
        painter::{GridPainter, PainterStack},
    },
};

#[derive(Default)]
pub struct EditorState {
    pub name: String,
    pub output: Output,
    pub color_grid_painters: Vec<Box<GridPainter<RGBA8>>>,
    pub light_grid_painters: Vec<Box<GridPainter<u8>>>,
    pub painter_stack: PainterStack,
    pub flatten: Flatten,
}
