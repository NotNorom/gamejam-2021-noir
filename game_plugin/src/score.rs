use crate::consts::*;

#[derive(Default)]
pub struct ScoreResource {
    score: u32,
}

impl ScoreResource {
    /// Get a reference to the scoreboard's score.
    pub fn score(&self) -> &u32 {
        &self.score
    }

    /// Increase the scoreboard's score.
    pub fn increase(&mut self) {
        self.score += 1;
    }
}
