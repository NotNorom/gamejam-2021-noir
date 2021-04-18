#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::render::{
    mesh::{shape, VertexAttributeValues},
    pipeline::{PipelineDescriptor, RenderPipeline},
    shader::{ShaderStage, ShaderStages},
};
use bevy::{app::AppBuilder, input::system::exit_on_esc_system};
// use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod consts;
use consts::*;
mod background;
//mod time;
mod ui;
use ui::UIPlugin;
use background::*;
mod fps_counter;
use fps_counter::{fps_counter_update_system, setup_fps_counter};
mod score;
use score::ScoreResource;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<ScoreResource>()
            //.insert_resource(Scoreboard(0))
            .add_startup_system(setup.system())
            // shading shit
            // .add_startup_system(setup_render_graph.system())
            // .add_system(update_time.system())
            // .add_system(update_resolution.system())
            // .add_system(update_mouse_position.system())
            // background shit
            // .add_startup_system(setup_background_shader.system())
            // .add_system(update_background_size.system())

            .add_startup_system(setup_fps_counter.system())

            .add_system(exit_on_esc_system.system())
            .add_system(fps_counter_update_system.system())

            .add_plugin(UIPlugin)
            .add_plugin(ShapePlugin)

            // .add_plugin(FrameTimeDiagnosticsPlugin::default())
            // .add_plugin(LogDiagnosticsPlugin::default())
            ;
    }
}

fn setup(mut commands: Commands) {
    info!("Setting up cameras");
    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .commands()
        .spawn_bundle(UiCameraBundle::default());

    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(30.0),
        ..shapes::RegularPolygon::default()
    };

    info!("Setting up shapes");
    commands.spawn_bundle(GeometryBuilder::build_as(
        &shape,
        ShapeColors::outlined(Color::hsla(0.0, 0.0, 0.0, 0.0), Color::WHITE),
        DrawMode::Outlined {
            fill_options: FillOptions::default(),
            outline_options: StrokeOptions::default().with_line_width(4.0),
        },
        Transform::default(),
    ));
}
