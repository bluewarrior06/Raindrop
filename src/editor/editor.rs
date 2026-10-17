use std::rc::Rc;

use iced::{
    Element,
    alignment::Horizontal,
    widget::{button, column, text, text_editor},
};

use crate::{
    editor::{
        editor_state::EditorState, screens::{create_scene_screen::CreateSceneScreen, editor_screen::EditorScreen},
    }, traits::messagable::{Messagable, MutMessagable}, types::screen::{screen_maintainer::ScreenMaintainer},
};

#[derive(Default)]
pub struct Editor {
    screens: ScreenMaintainer<
        EditorState,
        Message,
        ScreenMsg,
        ScreenMsgErr,
        ScreenMutMsg,
        ScreenMutMsgErr,
    >,
	
    state: EditorState,
}

#[derive(Clone)]
pub enum Message {
    MessageInterface(ScreenMsg),
    MutMessageInterface(ScreenMutMsg),
    ChangeScene(String),
    Exit,
}

#[derive(Clone)]
pub enum ScreenMsg {
    Empty,
}

#[derive(Clone)]
pub enum ScreenMsgErr {
    None,
}

#[derive(Clone)]
pub enum ScreenMutMsg {
    RenameScene(text_editor::Action),
    Exited,
    Entered,
}
#[derive(Clone)]
pub enum ScreenMutMsgErr {
    None,
}

impl Editor {
    pub fn boot() -> Self {
        let mut editor = Editor::default();

        editor.screens.add_scene(
            "create_scene".to_string(),
            Box::new(CreateSceneScreen::default()),
        );
        editor
            .screens
            .add_scene("edit".to_string(), Box::new(EditorScreen::default()));

        editor.screens.change_scene("create_scene".to_string());

        editor.screens.swap_scene();

        return editor;
    }

    pub fn update(this: &mut Self, message: Message) {
        match message {
            Message::MessageInterface(msg) => {
                this.screens.message(&this.state, msg);
            }
            Message::MutMessageInterface(msg) => {
                this.screens.message_mut(&mut this.state, msg);
            }
            Message::ChangeScene(scene) => {
                this.screens
                    .message_mut(&mut this.state, ScreenMutMsg::Exited);
                this.screens.change_scene(scene);
                this.screens.swap_scene();
                this.screens
                    .message_mut(&mut this.state, ScreenMutMsg::Entered);
            }
            Message::Exit => {}
        }
    }
    pub fn view(this: &Self) -> Element<'_, Message> {
        if let Some(scene) = this.screens.get_element() {
            return scene;
        } else {
            return column![button("Hello").on_press(Message::ChangeScene("scene".to_string()))]
                .into();
        }
    }
}
