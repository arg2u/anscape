//! # Graphics Modes
//! Note: Some terminals may not support some of the graphic mode sequences listed above.
pub mod set {
    /// Sets bold mode
    pub const BOLD: &str = "1m";
    /// Sets dim/faint mode
    pub const DIM: &str = "2m";
    /// Sets italic mode
    pub const ITALIC: &str = "3m";
    /// Sets underline mode
    pub const UNDERLINE: &str = "4m";
    /// Sets blinking mode
    pub const BLINKING: &str = "5m";
    /// Sets inverse/reverse mode
    pub const INVERSE: &str = "7m";
    /// Sets hidden/invisible mode
    pub const HIDDEN: &str = "8m";
    /// Sets strikethrough mode
    pub const STRIKETHROUGH: &str = "9m";
}
pub mod reset {
    /// Resets bold mode
    pub const BOLD: &str = "22m";
    /// Resets dim/faint mode
    pub const DIM: &str = "22m";
    /// Resets italic mode
    pub const ITALIC: &str = "23m";
    /// Resets underline mode
    pub const UNDERLINE: &str = "24m";
    /// Resets blinking mode
    pub const BLINKING: &str = "25m";
    /// Resets inverse/reverse mode
    pub const INVERSE: &str = "27m";
    /// Resets hidden/invisible mode
    pub const HIDDEN: &str = "28m";
    /// Resets strikethrough mode
    pub const STRIKETHROUGH: &str = "29m";
}
