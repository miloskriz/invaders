use crate::{TOTAL_COLS, TOTAL_ROWS};

pub type Frame = Vec<Vec<&'static str>>;

pub fn new_frame() -> Frame {
    let mut cols = Vec::with_capacity(TOTAL_COLS);
    for _ in 0..TOTAL_COLS {
        let mut col = Vec::with_capacity(TOTAL_ROWS);
        for _ in 0..TOTAL_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}
