use std::ops::Deref;
use std::fmt::Display;

use bevy::prelude::*;


/// The location of the mouse in screenspace.
#[derive(Clone, Copy, PartialEq, PartialOrd, Default, Debug)]
pub struct MousePos(Vec2);

impl Deref for MousePos {
    type Target = Vec2;
    fn deref(&self) -> &Vec2 {
        &self.0
    }
}

impl Display for MousePos {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}


fn update_mouse_position(
    mut cursor_move_events: EventReader<CursorMoved>,
    mut mouse_pos: ResMut<MousePos>,
) {
    for event in cursor_move_events.iter() {
        mouse_pos.0 = event.position;
    }
}

pub struct MousePosPlugin;
impl Plugin for MousePosPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<MousePos>().add_system(update_mouse_position.system());
    }
}