pub mod frame;
pub mod invaders;
pub mod player;
pub mod render;
pub mod shot;

pub const TOTAL_ROWS: usize = 20;
pub const TOTAL_COLS: usize = 20;

pub const OFFSET_ROWS: usize = 5;
pub const OFFSET_COLS: usize = 5;

pub const SHOT_TIMER: usize = 50;
pub const SHOTS_MAX: usize = 4;

pub const INVADERS_TIMER: usize = 2000;

pub const CHAR_PLAYER: &str = "Δ";
pub const CHAR_SHOT: &str = "↑";
pub const CHAR_EXPLODE: &str = "▓";
pub const CHAR_INVDR_A: &str = "﴾";
pub const CHAR_INVDR_B: &str = "﴿";
