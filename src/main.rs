use crate::editor::editor::Editor;

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
    iced::application(Editor::boot, Editor::update, Editor::view)
        .theme(iced::Theme::KanagawaWave)
        .title("Raindrop")
        .antialiasing(true)
        .run()
}
