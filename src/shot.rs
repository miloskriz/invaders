use rusty_time::timer::Timer;
use std::time::Duration;

use crate::{frame::Drawable, CHAR_EXPLODE, CHAR_SHOT, SHOT_TIMER};

// Create the Shot structure
pub struct Shot {
    pub x: usize,
    pub y: usize,
    pub exploding: bool,
    timer: Timer,
}

// Declare basic functions for the structure
impl Shot {
    // Implement the constructor function
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            exploding: false,
            timer: Timer::from_millis(SHOT_TIMER as u64),
        }
    }

    // move the shot every timed event
    pub fn update(&mut self, delta: Duration) {
        self.timer.update(delta);
        if self.timer.ready && !self.exploding {
            if self.y > 0 {
                self.y -= 1;
            }
            self.timer.reset();
        }
    }

    // Initiate explosion of the shot
    pub fn explode(&mut self) {
        self.exploding = true;
        self.timer = Timer::from_millis(SHOT_TIMER as u64 * 5);
    }

    // check if the shot is dead
    pub fn dead(&self) -> bool {
        (self.exploding && self.timer.ready) || (self.y == 0)
    }
}

// Implement the Drawable trait
impl Drawable for Shot {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = if self.exploding {
            CHAR_EXPLODE
        } else {
            CHAR_SHOT
        };
    }
}
