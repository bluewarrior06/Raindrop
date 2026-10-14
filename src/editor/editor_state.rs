use iced::{
    Element,
    widget::{button, column, text, text_editor},
};

use crate::types::{
    math::color::{RGBA8, Red8},
    paint::{
        flatten::Flatten,
        flatteners::basic_flattener::BasicFlattener,
        output::Output,
        painter::{GridPainter, PainterSet},
    },
};

#[derive(Default)]
pub struct EditorState {
    output: Output,
    painter_set: PainterSet,
    flattener: Flatten,

    state: State,

    name_context: text_editor::Content,
}

#[derive(Default)]
pub enum State {
    #[default]
    BeginCreating,
    Editing,
}

#[derive(Clone)]
pub enum Message {
    UpdatedCreatingName(text_editor::Action),

    BeginCreating,
    StopCreating,
    Exit,
}

impl EditorState {
    pub fn boot() -> Self {
        let mut editor = EditorState::default();

        let albedo_painter: GridPainter<RGBA8> = GridPainter::default();
        editor.painter_set.push_painter(Box::new(albedo_painter));

        let light_painter: GridPainter<Red8> = GridPainter::default();
        editor.painter_set.push_painter(Box::new(light_painter));

        let basic_flattener = BasicFlattener;

        editor.flattener.add_flattener(Box::new(basic_flattener));

        return editor;
    }
    pub fn update(this: &mut Self, message: Message) {
        match message {
            Message::UpdatedCreatingName(action) => {
                this.name_context.perform(action);
            }

            Message::BeginCreating => {
                this.state = State::Editing;
            }
            Message::StopCreating => {}
            Message::Exit => {}
        }
    }
    pub fn view(this: &Self) -> Element<'_, Message> {
        let widget = match this.state {
            State::BeginCreating => {
                column![
                    text("Create new level:"),
                    text_editor(&this.name_context)
                        .placeholder("Level name: ")
                        .on_action(Message::UpdatedCreatingName),
                    button("Begin").on_press(Message::BeginCreating)
                ]
            }
            State::Editing => {
                column![text("undefined!")]
            }
        };

        return widget.into();
    }
}
