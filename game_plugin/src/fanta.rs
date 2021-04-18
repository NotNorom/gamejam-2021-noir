use crate::consts::*;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

/// because it's not a sprite... get it?
struct Fanta;
struct Target;
struct Decoy;


fn spawn_decoys(
    mut commands: Commands,
    window: Res<WindowDescriptor>,
) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(30.0),
        ..shapes::RegularPolygon::default()
    };

    for _ in 0..FANTA_DECOY_COUNT {
        let coords = random_coords(window.width, window.height, GAME_AREA_PADDING);

        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(FANTA_DECOY_FILL_COLOR, FANTA_DECOY_OUTLINE_COLOR),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(4.0),
            },
            Transform::from_xyz(coords.0, coords.1, 0.0),
        ))
        .insert(Fanta)
        .insert(Decoy);
    }

    for _ in 0..FANTA_TARGET_COUNT {
        let coords = random_coords(window.width, window.height, GAME_AREA_PADDING);

        commands.spawn_bundle(GeometryBuilder::build_as(
            &shape,
            ShapeColors::outlined(FANTA_TARGET_FILL_COLOR, FANTA_TARGET_OUTLINE_COLOR),
            DrawMode::Outlined {
                fill_options: FillOptions::default(),
                outline_options: StrokeOptions::default().with_line_width(4.0),
            },
            Transform::from_xyz(coords.0, coords.1, 0.0),
        ))
        .insert(Fanta)
        .insert(Target);
    }
}

/// generate random coords between height and width
/// note: assumes that (0, 0) is in the center
fn random_coords(width: f32, height: f32, padding: f32) -> (f32, f32) {
    let mut rng = rand::thread_rng();
    let random_width = rng.gen_range(0.0 + padding .. width - padding) - width/2.0;
    let random_height = rng.gen_range(0.0 + padding .. height - padding) - height/2.0;
    (random_width, random_height)
}

pub struct FantaPlugin;
impl Plugin for FantaPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(spawn_decoys.system());
    }
}
