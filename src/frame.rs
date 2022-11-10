use crate::{T_COLS, T_ROWS};

pub type Frame = Vec< Vec<&'static str> >;

pub fn new_frame() -> Frame {
    let mut cols= Vec::with_capacity(T_COLS);
    for _ in 0..T_COLS {
        let mut col = Vec::with_capacity(T_ROWS);
        for _ in 0..T_ROWS {
            col.push(" ");
        }
        cols.push(col);
    }
    cols
}

pub trait Drawable {
    fn draw(&self, frame: &mut Frame);
}