use crate::{TOTAL_COLS, TOTAL_ROWS, CHAR_PLAYER, frame::Drawable};

// Create the Player structure
pub struct Player {
    x: usize,
    y: usize,
}

// Implement basic functions of the construct
impl Player {

    // Implement the constructor function
    pub fn new() -> Self {
        Self {
            x: TOTAL_COLS / 2,
            y: TOTAL_ROWS -1 ,
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
}

// Implement the Drawable trait 
impl Drawable for Player {
    fn draw(&self, frame: &mut crate::frame::Frame) {
        frame[self.x][self.y] = CHAR_PLAYER;
    }
}