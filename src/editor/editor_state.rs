use iced::{
    Element,
    widget::{button, column, text},
};

use crate::types::{
    math::color::{RGBA8, Red8},
    paint::{
        output::Output,
        painter::{GridPainter, PainterSet},
    },
};

#[derive(Default)]
pub struct EditorState {
    output: Output,
    painter_set: PainterSet,
}

#[derive(Clone)]
pub enum Message {
    Exit,
}

impl EditorState {
    pub fn boot() -> Self {
        let mut editor = EditorState::default();

        let albedo_painter: GridPainter<RGBA8> = GridPainter::default();
        editor.painter_set.push_painter(Box::new(albedo_painter));

        let light_painter: GridPainter<Red8> = GridPainter::default();
        editor.painter_set.push_painter(Box::new(light_painter));

        return editor;
    }
    pub fn update(_this: &mut Self, _message: Message) {}
    pub fn view(_this: &Self) -> Element<'_, Message> {
        column![
            text("hello"),
            button("button").on_press(Message::Exit),
            iced::widget::checkbox(false).on_toggle(|_toggle| { Message::Exit })
        ]
        .into()
    }
}
