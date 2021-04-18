use bevy::prelude::Color;

pub const SCORE_TEXT_SIZE: f32 = 14.0;
pub const SCORE_TEXT_COLOR: Color = Color::hsla(0.0, 0.0, 1.0, 0.9);

pub const FPS_TEXT_SIZE: f32 = 14.0;
pub const FPS_TEXT_COLOR: Color = Color::hsla(0.0, 1.0, 1.0, 0.9);

pub const DEFAULT_TEXT_FONT: &str = "fonts/FiraMono-Regular.ttf";


// Fanta configuration
// Yes I call them fanta, because they are not sprites

/// number of decoys to spawn
pub const FANTA_DECOY_COUNT: u32 = 10;
pub const FANTA_TARGET_COUNT: u32 = 1;

// decoy colors
pub const FANTA_DECOY_FILL_COLOR: Color = Color::hsla(0.0, 0.0, 0.0, 0.0);
pub const FANTA_DECOY_OUTLINE_COLOR: Color = Color::hsla(0.0, 0.0, 1.0, 1.0);

// target colors
pub const FANTA_TARGET_FILL_COLOR: Color = Color::hsla(0.0, 0.0, 0.0, 0.0);
pub const FANTA_TARGET_OUTLINE_COLOR: Color = Color::hsla(0.0, 1.0, 0.5, 1.0);

/// a border where assets are allowed to spawn in
pub const GAME_AREA_PADDING: f32 = 60.0;


/// States
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    Loading,
    StartAndScoreMenu,
    InstructionMenu,
    Playing,
    GameOver,
}
