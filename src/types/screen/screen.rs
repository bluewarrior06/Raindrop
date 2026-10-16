use iced::Element;

use crate::traits::messagable::{Messagable, MutMessagable, MutQueryable, Queryable};

/// A screen that can be sent messages to.
pub trait Screen<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>: Messagable<State, Msg, MsgErr> + MutMessagable<State, MutMsg, MutMsgErr> {
	fn get_element(&self) -> Element<'_, RootMsg>;
}

