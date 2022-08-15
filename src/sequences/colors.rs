//! # Background and Foreground Colors

/// # Color Types
/// Fg - Foreground changes a color of a text, and Bg - Background changes an area behind the text.
/// Use `Type::Fg/Bg(true) for using an aixterm specification in a basic color builder.
pub enum Type {
    Fg(bool),
    Bg(bool),
}
/// # Color Codes
pub mod codes {
    pub const BLACK: &str = "0";
    pub const RED: &str = "1";
    pub const GREEN: &str = "2";
    pub const YELLOW: &str = "3";
    pub const BLUE: &str = "4";
    pub const MAGENTA: &str = "5";
    pub const CYAN: &str = "6";
    pub const WHITE: &str = "7";
    pub const DEFAULT: &str = "9";
}
/// # Basic сolors builder
/// Example: `color(CYAN, Type::Foreground(false))`
pub fn basic(color: &str, t: Type) -> String {
    let st;
    match t {
        Type::Fg(is_bright) => {
            if !is_bright {
                st = "3";
            } else {
                st = "9";
            }
        }
        Type::Bg(is_bright) => {
            if !is_bright {
                st = "4";
            } else {
                st = "10";
            }
        }
    }
    format!("{}{}m", st, color)
}
/// # Basic foreground сolor builder
/// Example: `color(CYAN, Type::Foreground(false))`
pub fn basic_fg(color: &str, is_bright: bool) -> String {
    return basic(color, Type::Fg(is_bright));
}
/// # Basic foreground сolor builder
/// Example: `color(CYAN, Type::Foreground(false))`
pub fn basic_bg(color: &str, is_bright: bool) -> String {
    return basic(color, Type::Bg(is_bright));
}
/// # RGB colors builder
/// Example: `rgb(255, 255, 255, Type::Foreground(false))`
pub fn rgb(r: u8, g: u8, b: u8, t: Type) -> String {
    let st;
    match t {
        Type::Fg(_) => {
            st = "3";
        }
        Type::Bg(_) => {
            st = "4";
        }
    }
    format!("{}8;2;{};{};{}m", st, r, g, b)
}
/// #Foreground RGB colors builder
/// Example: `rgb(255, 255, 255, Type::Foreground(false))`
pub fn rgb_fg(r: u8, g: u8, b: u8) -> String {
    return rgb(r, g, b, Type::Fg(false));
}
/// #Background RGB colors builder
/// Example: `rgb(255, 255, 255, Type::Foreground(false))`
pub fn rgb_bg(r: u8, g: u8, b: u8) -> String {
    return rgb(r, g, b, Type::Bg(false));
}
