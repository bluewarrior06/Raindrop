use crate::{editor::editor::Editor};

pub mod editor;
pub mod traits;
pub mod types;

#[cfg(test)]
pub mod tests;

pub fn main() -> iced::Result {
    iced::application(Editor::boot, Editor::update, Editor::view)
        .theme(iced::Theme::KanagawaWave)
        .title("Raindrop")
        .antialiasing(true)
        .run()
}
