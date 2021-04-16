#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::{app::AppBuilder, input::system::exit_on_esc_system};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;


mod scoreboard;
use scoreboard::Scoreboard;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Scoreboard(0))
            .add_startup_system(setup.system())
            .add_state(GameState::Loading)
            .add_system(exit_on_esc_system.system())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}


fn setup(mut commands: Commands) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
}