use std::io::{Stdout, Write};
use crossterm::QueueableCommand;
use crossterm::cursor::MoveTo;
use crossterm::style::{SetBackgroundColor, Color};
use crossterm::terminal::{Clear, ClearType};

use crate::frame::Frame;

pub fn render(stdout: &mut Stdout, last_frame: &Frame, curr_frame: &Frame, force: bool) {
    
    // If a complete redraw is forced, all the frame is cleared to blue background
    if force {
        stdout.queue(SetBackgroundColor(Color::Blue)).unwrap();
        stdout.queue(Clear(ClearType::All)).unwrap();
        stdout.queue(SetBackgroundColor(Color::Black)).unwrap();
    }
    
    //then 
    for (x, col) in curr_frame.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_frame[x][y] || force {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s);
            }
        }
    }
    stdout.flush().unwrap();
}