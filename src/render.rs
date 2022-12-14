use crossterm::{
    cursor::MoveTo,
    style::{Color, SetBackgroundColor},
    terminal::{Clear, ClearType},
    QueueableCommand,
};
use std::io::{Stdout, Write};

use crate::{frame::Frame, OFFSET_COLS, OFFSET_ROWS};

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    // If a complete redraw is forced, all the frame is cleared to blue background
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }

    // then iterate across all the playfield and draw/print the contents
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout
                    .queue(MoveTo((x + OFFSET_COLS) as u16, (y + OFFSET_ROWS) as u16))
                    .unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}
