use iced::Theme;
use noise::{NoiseFn, Perlin};

use crate::{
    editor::editor_state::EditorState,
    traits::mapper::MapperCopy,
    types::{
        math::{
            color::RGBA8,
            vec::{Vec2D, Vec2Du},
        },
        paint::{
            effects::{block_effect::BlockEffect, gradient_effect::GradientEffect},
            flattened::Flattened,
            flatteners::{basic_flattener::BasicFlattener, light_flattener::LightFlattener},
            layers::grid_layer::GridLayer,
            output::Output,
            painter::{DrawablePainter, Painter},
        },
    },
};

pub mod editor;
pub mod traits;
pub mod types;

#[cfg(test)]
pub mod tests;

/// Splash is an editor for generating images for games.
/// Images are created through a collection of stages linearly:
/// 1. Draw onto layers.
/// There are two different types of layers: Tiled layers and Prop layers. Tiled layers store information in a fixed grid; Prop layers store information that isn't locked onto a grid.
/// 2. Feed these layers into effects to draw onto an image.
/// 3. Save the resulting image onto the disk.
pub fn main() -> iced::Result {
	return iced::application(EditorState::boot, EditorState::update, EditorState::view)
        .theme(Theme::KanagawaDragon)
        .run();
}
