use std::collections::HashMap;

/// Aren't generic so beautiful.
use iced::Element;

use crate::{
    traits::messagable::{Messagable, MutMessagable},
    types::screen::screen::Screen,
};

/// Holds multiple scenes that can be switched between, along with messaged.
/// Scenes are not immediately changed, and are swapped when flush() is called.
pub struct ScreenMaintainer<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr> {
    scenes: HashMap<String, Box<dyn Screen<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>>>,
    cur_scene: String,
    nxt_scene: String,
}

impl<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>
    ScreenMaintainer<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>
{
    pub fn add_scene(
        &mut self,
        name: String,
        scene: Box<dyn Screen<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>>,
    ) {
        self.scenes.insert(name, scene);
    }
    pub fn remove_scene(&mut self, name: String) {
        self.scenes.remove(&name);
    }

    pub fn swap_scene(&mut self) {
        self.cur_scene = self.nxt_scene.clone();
    }

    pub fn change_scene(&mut self, scene: String) {
        self.nxt_scene = scene.clone();
    }

    pub fn get_element(&self) -> Option<Element<'_, RootMsg>> {
        if let Some(scene) = self.scenes.get(&self.cur_scene) {
            Some(scene.get_element())
        } else {
            None
        }
    }
}

impl<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr> Messagable<State, Msg, MsgErr>
    for ScreenMaintainer<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>
{
    fn message(&self, state: &State, message: Msg) -> Option<MsgErr> {
        if let Some(cur_scene) = self.scenes.get(&self.cur_scene) {
            return cur_scene.message(state, message);
        } else {
            return None;
        }
    }
}

impl<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr> MutMessagable<State, MutMsg, MutMsgErr>
    for ScreenMaintainer<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>
{
    fn message_mut(&mut self, state: &mut State, message: MutMsg) -> Option<MutMsgErr> {
        if let Some(cur_scene) = self.scenes.get_mut(&self.cur_scene) {
            return cur_scene.message_mut(state, message);
        } else {
            return None;
        }
    }
}

impl<'a, State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr> Default
    for ScreenMaintainer<State, RootMsg, Msg, MsgErr, MutMsg, MutMsgErr>
{
    fn default() -> Self {
        Self {
            scenes: Default::default(),
            cur_scene: String::default(),
            nxt_scene: Default::default(),
        }
    }
}
