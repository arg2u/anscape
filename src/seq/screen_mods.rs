//! # Screen Modes
pub mod common {
    use super::super::base::ESC;
    /// 40 x 25 monochrome (text)
    pub const W40H25M: &str = "\x1b[=0h";
    /// 40 x 25 color (text)
    pub const W40H25C: &str = "\x1b[=1h";
    /// 80 x 25 monochrome (text)
    pub const W80H25M: &str = "\x1b[=2h";
    /// 80 x 25 color (text)
    pub const W80H25C: &str = "\x1b[=3h";
    /// 320 x 200 4-color (graphics)
    pub const W320H200C4: &str = "\x1b[=4h";
    /// 320 x 200 monochrome (graphics)
    pub const W320H200M: &str = "\x1b[=5h";
    /// 640 x 200 monochrome (graphics)
    pub const W640H200C4: &str = "\x1b[=6h";
    /// Enables line wrapping
    pub const LINE_WRAPPING: &str = "\x1b[=7h";
    /// 320 x 200 color (graphics)
    pub const W320H200C: &str = "\x1b[=13h";
    /// 640 x 200 color (16-color graphics)
    pub const W640H200C: &str = "\x1b[=14h";
    /// 640 x 350 monochrome (2-color graphics)
    pub const W640H350M: &str = "\x1b[=15h";
    /// 640 x 350 color (16-color graphics)
    pub const W640H350C: &str = "\x1b[=16h";
    /// 640 x 480 monochrome (2-color graphics)
    pub const W640H480M: &str = "\x1b[=17h";
    /// 640 x 480 color (16-color graphics)
    pub const W640H480C: &str = "\x1b[=18h";

    /// Resets the mode by using the same values that Set Mode uses, except for 7, which disables line wrapping
    pub fn reset(which: &str) -> String {
        format!("{}{}", ESC, which.replace("h", "l"))
    }
}

pub mod private {
    /// Makes cursor invisible
    pub const INVISIBLE_CURSOR: &str = "\x1b[?25l";
    /// Makes cursor visible
    pub const VISIBLE_CURSOR: &str = "\x1b[?25h";
    /// Restores screen
    pub const RESTORE_SCREEN: &str = "\x1b[?47l";
    /// Saves screen
    pub const SAVE_SCREEN: &str = "\x1b[?47h";
    /// Enables the alternative buffer
    pub const ENABLE_ALT_BUF: &str = "\x1b[?1049h";
    /// Disables the alternative buffer
    pub const DISABLE_ALT_BUF: &str = "\x1b[?1049l";
}
