use crate::{consts::*, score::ScoreResource};
//use crate::time::ControlledTime;

use bevy::prelude::*;

fn setup_ui(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn_bundle(TextBundle {
        style: Style {
            position_type: PositionType::Absolute,
            position: Rect {
                right: Val::Px(2.0),
                top: Val::Px(2.0),
                ..Default::default()
            },
            ..Default::default()
        },
        text: Text {
            sections: vec![
                TextSection {
                    value: "SCORE: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load(DEFAULT_TEXT_FONT),
                        font_size: SCORE_TEXT_SIZE,
                        color: SCORE_TEXT_COLOR,
                    },
                },
                TextSection {
                    value: "0".to_string(),
                    style: TextStyle {
                        font: asset_server.load(DEFAULT_TEXT_FONT),
                        font_size: SCORE_TEXT_SIZE,
                        color: SCORE_TEXT_COLOR,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    });
}

struct ScoreText;
fn update_score_text(score: Res<ScoreResource>, query: Query<(&mut Text, &ScoreText)>) {
    // don't do shit if there is no change to the score
    if !score.is_changed() {
        return;
    }

    // update score text
    query.for_each_mut(|(mut text, _marker)| {
        text.sections[1].value = format!("{}", score.score());
    });
}

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system(setup_ui.system());
        app.add_system(update_score_text.system());
        // app.add_system_set(SystemSet::on_enter(AppState::Game).with_system(setup_ui.system()))
        //     .add_system_set(
        //         SystemSet::on_update(AppState::Game)
        //             .with_system(update_time_text.system())
        //             .with_system(update_score_text.system()),
        //     );
    }
}
