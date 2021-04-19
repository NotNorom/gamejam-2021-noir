#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use bevy::render::{
    mesh::{shape, VertexAttributeValues},
    pipeline::{PipelineDescriptor, RenderPipeline},
    shader::{ShaderStage, ShaderStages},
};
use bevy::{app::AppBuilder, input::system::exit_on_esc_system};

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::ShapePlugin;

mod camera;
use camera::CameraPlugin;

mod mouse_pos;
use mouse_pos::MousePosPlugin;

mod consts;
use consts::*;

mod fanta;
use fanta::FantaPlugin;

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
            // shading shit
            .add_startup_system(setup_render_graph.system())
            .add_system(update_time.system())
            .add_system(update_resolution.system())
            // background shit
            .add_startup_system(setup_background_shader.system())
            .add_system(update_background_size.system())

            .add_startup_system(setup_fps_counter.system())
            .add_system(fps_counter_update_system.system())

            .add_system(exit_on_esc_system.system())

            .add_plugin(CameraPlugin)
            .add_plugin(UIPlugin)
            .add_plugin(ShapePlugin)
            .add_plugin(MousePosPlugin)
            .add_plugin(FantaPlugin)
            ;
    }
}
