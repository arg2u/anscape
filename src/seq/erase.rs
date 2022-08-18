//! # Erase Functions
//! Note: Erasing the line won't move the cursor, meaning that the cursor will stay at the last position it was at before the line was erased. You can use \r after erasing the line, to return the cursor to the start of the current line.

/// Erases from cursor until end of screen
pub const FROM_CURSOR_TO_END_OF_SCREEN: &str = "\x1b[0J";
/// Erases from cursor to beginning of screen
pub const FROM_CURSOR_TO_BEGINING_OF_SCREEN: &str = "\x1b[1J";
/// Erases entire screen
pub const ENTIRE_SCREEN: &str = "\x1b[2J";
/// Erases saved lines
pub const SAVED_LINE: &str = "\x1b[3J";
/// Erases from cursor to end of line
pub const FROM_CURSOR_TO_END_OF_LINE: &str = "\x1b[0K";
/// Erases start of line to the cursor
pub const FROM_START_OF_LINE_TO_CURSOR: &str = "\x1b[1K";
/// Erases the entire line
pub const ENTIRE_LINE: &str = "\x1b[2K";
