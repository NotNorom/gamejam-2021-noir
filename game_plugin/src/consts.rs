use bevy::prelude::Color;

pub const SCORE_TEXT_SIZE: f32 = 14.0;
pub const SCORE_TEXT_COLOR: Color = Color::hsla(0.0, 0.0, 1.0, 0.9);

pub const FPS_TEXT_SIZE: f32 = 14.0;
pub const FPS_TEXT_COLOR: Color = Color::hsla(90.0, 1.0, 0.5, 0.9);

/// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    StartAndScoreMenu,
    InstructionMenu,
    Playing,
    GameOver,
}
