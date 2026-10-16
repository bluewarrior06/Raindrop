use iced::widget::{button, column, text, text_editor};

use crate::{
    editor::{
        editor::{Message, ScreenMsg, ScreenMsgErr, ScreenMutMsg, ScreenMutMsgErr},
        editor_state::{self, EditorState},
    },
    traits::messagable::{Messagable, MutMessagable},
    types::screen::screen::Screen,
};

#[derive(Default)]
pub struct CreateSceneScreen {
    name_content: text_editor::Content,
}

impl Messagable<EditorState, ScreenMsg, ScreenMsgErr> for CreateSceneScreen {
    fn message(&self, state: &EditorState, message: ScreenMsg) -> Option<ScreenMsgErr> {
        None
    }
}

impl MutMessagable<EditorState, ScreenMutMsg, ScreenMutMsgErr> for CreateSceneScreen {
    fn message_mut(
        &mut self,
        state: &mut EditorState,
        message: ScreenMutMsg,
    ) -> Option<ScreenMutMsgErr> {
        match message {
            ScreenMutMsg::RenameScene(action) => {
                self.name_content.perform(action);
                return None;
            }

            ScreenMutMsg::Entered => {
                return None;
            }
            ScreenMutMsg::Exited => {
                state.name = self.name_content.text();
                return None;
            }

            _ => {
                return None;
            }
        }
    }
}

impl Screen<EditorState, Message, ScreenMsg, ScreenMsgErr, ScreenMutMsg, ScreenMutMsgErr>
    for CreateSceneScreen
{
    fn get_element(&self) -> iced::Element<'_, Message> {
        column![
            text_editor(&self.name_content).on_action(|action| {
                Message::MutMessageInterface(ScreenMutMsg::RenameScene(action))
            }),
            button("Edit Level").on_press(Message::ChangeScene("edit".to_string()))
        ]
        .into()
    }
}
