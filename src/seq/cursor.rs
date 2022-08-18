//! # Cursor Controls
//! Note: Some sequences, like saving and restoring cursors, are private sequences and are not standardized. While some terminal emulators (i.e. xterm and derived) support both SCO and DEC sequences, they are likely to have different functionality. It is therefore recommended to use DEC sequences.

use super::base::ESC;
/// Moves cursor to home position (0, 0)
pub const HOME: &str = "H";
pub const UP: &str = "A";
pub const DOWN: &str = "B";
pub const RIGHT: &str = "C";
pub const LEFT: &str = "D";
pub const BNL: &str = "E";
pub const BPL: &str = "F";
pub const COL: &str = "G";
/// Moves cursor one line up, scrolling if needed
pub const OLUP: &str = "M";
/// Saves cursor position (DEC)
pub const DSCP: &str = "7";
/// Restores cursor position (DEC)
pub const DRSCP: &str = "8";
/// Saves cursor position (SCO)
pub const SSCP: &str = "s";
/// Restores cursor position (SCO)
pub const SRSCP: &str = "u";
/// Requests cursor position
pub const POS: &str = "6n";

/// Moves cursor to home position (0, 0)
pub fn move_to_home() -> String {
    return format!("{}{}", ESC, HOME);
}
/// Moves cursor to line #, column #
pub fn move_to(line: u64, column: u64) -> String {
    return format!("{}{};{}{}", ESC, line, column, HOME);
}
/// Moves cursor up # lines
pub fn move_up(line: u64) -> String {
    return format!("{}{}{}", ESC, line, UP);
}
/// Moves cursor down # lines
pub fn move_down(line: u64) -> String {
    return format!("{}{}{}", ESC, line, DOWN);
}
/// Moves cursor left # columns
pub fn move_left(column: u64) -> String {
    return format!("{}{}{}", ESC, column, LEFT);
}
/// Moves cursor right # columns
pub fn move_right(column: u64) -> String {
    return format!("{}{}{}", ESC, column, RIGHT);
}
/// Moves cursor to beginning of next line, # lines down
pub fn move_to_bnl(column: u64) -> String {
    return format!("{}{}{}", ESC, column, BNL);
}
/// Moves cursor to beginning of previous line, # lines up
pub fn move_to_bpl(column: u64) -> String {
    return format!("{}{}{}", ESC, column, BPL);
}
/// Requests cursor position
pub fn request_position() -> String {
    return format!("{}{}", ESC, POS);
}

/// Moves cursor to column #
pub fn move_to_col(column: u64) -> String {
    return format!("{}{}{}", ESC, column, COL);
}
/// Moves cursor one line up, scrolling if needed
pub fn move_one_line_up() -> String {
    return format!("{} {}", ESC.replace("[", ""), OLUP);
}
/// Saves cursor position (DEC)
pub fn dec_save_position() -> String {
    return format!("{} {}", ESC.replace("[", ""), DSCP);
}
/// Restores cursor position (DEC)
pub fn dec_restore_position() -> String {
    return format!("{} {}", ESC.replace("[", ""), DRSCP);
}
/// Saves cursor position (SCO)
pub fn sco_save_position() -> String {
    return format!("{}{}", ESC, SSCP);
}
/// Restores cursor position (SCO)
pub fn sco_restore_position() -> String {
    return format!("{}{}", ESC, SRSCP);
}
