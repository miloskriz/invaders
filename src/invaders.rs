use std::{time::Duration, cmp::max};

use rusty_time::timer::Timer;

use crate::{TOTAL_COLS, TOTAL_ROWS, INVADERS_TIMER, SHOT_TIMER, frame::Drawable, CHAR_INVDR_A, CHAR_INVDR_B};

// Create the struct of a single invader
pub struct Invader {
    pub x: usize,
    pub y: usize,
}

// Create an army of invaders
pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    direction: i32,
}

//Implement the basic logic
impl Invaders {

    // Implement the construstor function for the invaders
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..TOTAL_COLS {
            for y in 0..TOTAL_ROWS {
                if (x > 1)
                    && (x < TOTAL_COLS - 2)
                    && (y > 0)
                    && (y < TOTAL_ROWS / 2)
                    && (x % 2 == 0)
                    && (y % 2 == 0) {
                        army.push(Invader {x, y});
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(INVADERS_TIMER as u64),
            direction: 1,
        }
    } 

    // Update the position of all invaders
    pub fn update(&mut self, delta: Duration) -> bool {
        // check the time elapsed
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            
            //check if the army can move sideways or needs to move downward
            let mut downwards = false;

            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            }

            if self.direction == 1 {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                if max_x == TOTAL_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            // now move accordingly
            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() as u64 * 80 / 100, SHOT_TIMER as u64 * 5);
                self.move_timer = Timer::from_millis(new_duration);
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = (invader.x as i32 + self.direction) as usize;

                }
            }
            return true;
        }
        false
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32()) > 0.5 {
                    CHAR_INVDR_A
                } else {
                    CHAR_INVDR_B
                };
        }
    }
}