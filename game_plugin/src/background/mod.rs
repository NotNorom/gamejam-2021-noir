use bevy::{
    render::render_graph::{base, RenderGraph, RenderResourcesNode},
    window::WindowResized,
};

use super::*;

use bevy::reflect::TypeUuid;
use bevy::render::renderer::RenderResources;

#[derive(RenderResources, Default, TypeUuid)]
#[uuid = "0320b9b8-b3a3-4baa-8bfa-c94008177b17"]
/// Resource that will be passed to shaders
pub struct ShaderInputs {
    time: f32,
    resolution: Vec2,
}

/// Updates time in ShaderInputs every frame
pub fn update_time(time: Res<Time>, mut nodes: Query<&mut ShaderInputs>) {
    let time = time.seconds_since_startup();
    for mut node in nodes.iter_mut() {
        node.time = time as f32;
    }
}

/// Updates resolution in ShaderInputs if window size changes
pub fn update_resolution(
    mut event_reader: EventReader<WindowResized>,
    mut background: Query<&mut ShaderInputs>,
) {
    for event in event_reader.iter() {
        for mut node in background.iter_mut() {
            node.resolution = Vec2::new(event.width, event.height);
        }
    }
}

/// Adds ShaderInputs as an edge in the render graph
pub fn setup_render_graph(mut render_graph: ResMut<RenderGraph>) {
    render_graph.add_system_node("inputs", RenderResourcesNode::<ShaderInputs>::new(true));
    render_graph
        .add_node_edge("inputs", base::node::MAIN_PASS)
        .unwrap();
}

pub struct Background;

pub fn setup_background_shader(
    mut commands: Commands,
    mut pipelines: ResMut<Assets<PipelineDescriptor>>,
    mut shaders: ResMut<Assets<Shader>>,
    mut meshes: ResMut<Assets<Mesh>>,
    window: Res<WindowDescriptor>,
) {
    // Create a new shader pipeline
    let pipeline_handle = pipelines.add(PipelineDescriptor::default_config(ShaderStages {
        vertex: shaders.add(Shader::from_glsl(
            ShaderStage::Vertex,
            include_str!("background.vert"),
        )),
        fragment: Some(shaders.add(Shader::from_glsl(
            ShaderStage::Fragment,
            include_str!("background.frag"),
        ))),
    }));

    // create a plane
    let plane = Mesh::from(shape::Quad {
        size: Vec2::new(1.0, 1.0),
        flip: false,
    });

    // place plane
    commands
        .spawn_bundle(MeshBundle {
            mesh: meshes.add(plane),
            render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
                pipeline_handle,
            )]),
            transform: Transform::from_scale(Vec3::new(window.width, window.height, 1.0)),
            ..Default::default()
        })
        .insert(Background)
        .insert(ShaderInputs {
            time: 0.0,
            resolution: Vec2::new(window.width, window.height),
        });
}

pub fn update_background_size(
    mut event_reader: EventReader<WindowResized>,
    mut background: Query<(&mut Transform, &Background)>,
) {
    for event in event_reader.iter() {
        for (mut transform, _) in background.iter_mut() {
            transform.scale = Vec3::new(event.width, event.height, 1.);
        }
    }
}
