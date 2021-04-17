use bevy::window::WindowResized;

use super::*;


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
        vertex: shaders.add(Shader::from_glsl(ShaderStage::Vertex, include_str!("background.vert"))),
        fragment: Some(shaders.add(Shader::from_glsl(ShaderStage::Fragment, include_str!("background.frag")))),
    }));

    // create a plane
    let plane = Mesh::from(shape::Quad { size: Vec2::new(1.0, 1.0), flip: false  });

    // place plane
    commands.spawn_bundle(MeshBundle {
        mesh: meshes.add(plane),
        render_pipelines: RenderPipelines::from_pipelines(vec![RenderPipeline::new(
            pipeline_handle,
        )]),
        transform: Transform::from_scale(Vec3::new(window.width, window.height, 1.0)),
        ..Default::default()
    })
    .insert(Background);
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