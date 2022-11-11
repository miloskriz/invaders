use std::time::Duration;

use crate::{TOTAL_COLS, TOTAL_ROWS, CHAR_PLAYER, SHOTS_MAX, frame::Drawable, shot::Shot};

// Create the Player structure
pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

// Implement basic functions of the construct
impl Player {

    // Implement the constructor function
    pub fn new() -> Self {
        Self {
            x: TOTAL_COLS / 2,
            y: TOTAL_ROWS -1 ,
            shots: Vec::new(),
        }
    }

    // Move to the left
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    // Move to the right
    pub fn move_right(&mut self) {
        if self.x < TOTAL_COLS - 1 {
            self.x += 1;
        }
    }

    // Shoot a shot if available
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < SHOTS_MAX {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }

    // Update all shots of the Player
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

}

// Implement the Drawable trait 
impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
       
        // draw the player
        frame[self.x][self.y] = CHAR_PLAYER;

        // draw all shots of the player
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}