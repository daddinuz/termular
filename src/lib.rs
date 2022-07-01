#![feature(deadline_api)]

pub mod cursor;
pub mod nio;
pub mod printer;
pub mod screen;
pub mod stream;
pub mod vector;

mod term;

pub use term::{size, with_mode, Mode, Term};
