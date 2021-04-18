// set to  windows to windows
// set to console to see logs!!!
#![windows_subsystem = "console"]

#[cfg(target_arch = "wasm32")]
use bevy_webgl2;

use bevy::diagnostic::LogDiagnosticsPlugin;
use bevy::DefaultPlugins;
use bevy::{
    diagnostic::FrameTimeDiagnosticsPlugin,
    prelude::{App, ClearColor, Color, Msaa, WindowDescriptor},
};
use game_plugin::GamePlugin;

fn main() {
    let mut app = App::build();
    app.insert_resource(Msaa { samples: 4 })
        .insert_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .insert_resource(WindowDescriptor {
            width: 800.,
            height: 600.,
            title: "gamejam-2021-noir floating".to_string(),
            vsync: true,
            // mode: bevy::window::WindowMode::BorderlessFullscreen,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(FrameTimeDiagnosticsPlugin::default())
        .add_plugin(LogDiagnosticsPlugin::default())
        .add_plugin(GamePlugin);

    #[cfg(target_arch = "wasm32")]
    app.add_plugin(bevy_webgl2::WebGL2Plugin);

    app.run();
}
