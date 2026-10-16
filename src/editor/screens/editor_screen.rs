use iced::widget::{button, column, text};

use crate::{editor::{editor::{Message, ScreenMsg, ScreenMsgErr, ScreenMutMsg, ScreenMutMsgErr}, editor_state::{self, EditorState}}, traits::messagable::{Messagable, MutMessagable}, types::screen::screen::Screen};

#[derive(Default)]
pub struct EditorScreen
{
	
}

impl Messagable<EditorState, ScreenMsg, ScreenMsgErr> for EditorScreen {
	fn message(&self, state: &EditorState, message: ScreenMsg) -> Option<ScreenMsgErr> {
		None
	}
}

impl MutMessagable<EditorState, ScreenMutMsg, ScreenMutMsgErr> for EditorScreen {
	fn message_mut(&mut self, state: &mut EditorState, message: ScreenMutMsg) -> Option<ScreenMutMsgErr> {
		None
	}
}

impl Screen<EditorState, Message, ScreenMsg, ScreenMsgErr, ScreenMutMsg, ScreenMutMsgErr> for EditorScreen {

	fn get_element(&self) -> iced::Element<'_, Message> {
		column![
			button("to_scene").on_press(Message::ChangeScene("create_scene".to_string()))
		].into()
	}
}