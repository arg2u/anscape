//! # Anscape
//!
//! It provides a simple ways to customize your terminal window with styles and a cursor manipulations using ANSI Escape Sequences.

pub mod sequences;
use sequences::base::{ESC, RESET};

/// # Sequence builder which join args with ESC delimiter
/// Example: `build(&[sequences::colors::foreground::RED,sequences::styles::set::BOLD])`
pub fn build(args: &[&str]) -> String {
    ESC.to_string() + &args.join(ESC)
}

/// # Stylish println
/// Example: `styled_print("hello", &[sequences::colors::foreground::RED,sequences::styles::set::BOLD])`
pub fn sprint(message: &str, style: &[&str]) {
    let builded = build(style);
    println!("{}{}{}{}", builded, message, ESC, RESET);
}
