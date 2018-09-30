pub extern crate clipboard;
pub extern crate num;

use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;

mod constants;
pub mod checkers;
pub mod coord;
pub mod graph;
pub mod math;
pub mod representations;
pub mod series;
pub mod sets;

/// Macro for printing answer to standard output, with newline, and paste it into clipboard
#[macro_export]
macro_rules! answer {
	() => (panic!("No answer given"));
	($fmt: expr) => ($crate::print_answer($fmt));
	($fmt: expr, $($arg:tt)*) => ($crate::print_answer(format!($fmt, $($arg)*)));
}

pub fn print_answer(s: String) {
    let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
    println!("{}", s);
    ctx.set_contents(s).unwrap();
}
