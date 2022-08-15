//! # Screen Modes
pub mod common {
    use super::super::base::ESC;
    /// 40 x 25 monochrome (text)
    pub const W40H25M: &str = "=0h";
    /// 40 x 25 color (text)
    pub const W40H25C: &str = "=1h";
    /// 80 x 25 monochrome (text)
    pub const W80H25M: &str = "=2h";
    /// 80 x 25 color (text)
    pub const W80H25C: &str = "=3h";
    /// 320 x 200 4-color (graphics)
    pub const W320H200C4: &str = "=4h";
    /// 320 x 200 monochrome (graphics)
    pub const W320H200M: &str = "=5h";
    /// 640 x 200 monochrome (graphics)
    pub const W640H200C4: &str = "=6h";
    /// Enables line wrapping
    pub const LINE_WRAPPING: &str = "=7h";
    /// 320 x 200 color (graphics)
    pub const W320H200C: &str = "=13h";
    /// 640 x 200 color (16-color graphics)
    pub const W640H200C: &str = "=14h";
    /// 640 x 350 monochrome (2-color graphics)
    pub const W640H350M: &str = "=15h";
    /// 640 x 350 color (16-color graphics)
    pub const W640H350C: &str = "=16h";
    /// 640 x 480 monochrome (2-color graphics)
    pub const W640H480M: &str = "=17h";
    /// 640 x 480 color (16-color graphics)
    pub const W640H480C: &str = "=18h";

    /// Resets the mode by using the same values that Set Mode uses, except for 7, which disables line wrapping
    pub fn reset(which: &str) -> String {
        format!("{}{}", ESC, which.replace("h", "l"))
    }
}

pub mod private {
    /// Makes cursor invisible
    pub const INVISIBLE_CURSOR: &str = "?25l";
    /// Makes cursor visible
    pub const VISIBLE_CURSOR: &str = "?25h";
    /// Restores screen
    pub const RESTORE_SCREEN: &str = "?47l";
    /// Saves screen
    pub const SAVE_SCREEN: &str = "?47h";
    /// Enables the alternative buffer
    pub const ENABLE_ALT_BUF: &str = "?1049h";
    /// Disables the alternative buffer
    pub const DISABLE_ALT_BUF: &str = "?1049l";
}
