use crate::types::{
    math::color::RGBA8,
    paint::{
        flatten::Flatten,
        output::Output,
        painter::{GridPainter, PainterStack},
    },
};

/// Used to send messages and queries between to connect the physical editor with the logical editor.
/// The physical editor is the editor that the user uses. This is the GUI.
/// The logical editor is the editor that the computer uses. These are the Painters, Layers, Effect, Output, etc...
#[derive(Default)]
pub struct EditorState {
    pub name: String,
    pub output: Output,
    pub color_grid_painters: Vec<Box<GridPainter<RGBA8>>>,
    pub light_grid_painters: Vec<Box<GridPainter<u8>>>,
    pub painter_stack: PainterStack,
    pub flatten: Flatten,
}
