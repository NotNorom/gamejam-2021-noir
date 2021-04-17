#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::{app::AppBuilder, input::system::exit_on_esc_system};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;


mod scoreboard;
use fps_counter::{fps_counter_update_system, setup_fps_counter};
use scoreboard::Scoreboard;

mod fps_counter;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    StartAndScoreMenu,
    InstructionMenu,
    Playing,
    GameOver,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .insert_resource(Scoreboard(0))
            .add_startup_system(setup.system())
            .add_startup_system(setup_fps_counter.system())
            .add_state(GameState::Loading)
            .add_system(exit_on_esc_system.system())
            .add_system(fps_counter_update_system.system())
            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands.spawn_bundle(UiCameraBundle::default());
}