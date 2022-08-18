//! # Background and Foreground Colors

/// # Color Types
/// Fg (foreground) changes a color of a text, and Bg (background) changes an area behind the text.
pub enum Type {
    Fg,
    Bg,
}

/// # Macros for building basic color constants
/// It generates 4 constants per provided color.
///
/// For example, `pub BLACK:"0"` will generate:
///
/// `pub const BLACK:&str = "\x1b[30m";`
///
/// `pub const BLACK_BG:&str = "\x1b[40m"`
///
/// `pub const BR_BLACK:&str = "\x1b[90m"`
///
/// `pub const BR_BLACK_BG:&str = "\x1b[100m"`
///
/// **BR_** - bright colors prefix
///
/// **_BG** - backround colors suffix
///
/// ### Usage example:
/// ```
/// use anscape::colors;
/// colors! {
///     pub BLACK:"0"
///     pub RED:"1"
///     pub GREEN:"2"
///     pub YELLOW:"3"
///     pub BLUE:"4"
///     pub MAGENTA:"5"
///     pub CYAN:"6"
///     pub WHITE:"7"
///     pub DEFAULT:"9"
/// }
/// ```
#[macro_export]
macro_rules! colors {
    ($($vis:vis $cname:ident:$cval:literal)*) => {
            use paste::paste;
            paste!{
                $(
                    #[doc="Set " $cname " foreground color"]
                    $vis const $cname:&str = concat!("\x1b[", "3",$cval,"m");
                    #[doc="Set " $cname " background color"]
                    $vis const [<$cname _BG>]:&str = concat!("\x1b[", "4",$cval,"m");
                    #[doc="Set bright " $cname " foreground color"]
                    $vis const [<BR_ $cname>]:&str = concat!("\x1b[","9",$cval,"m");
                    #[doc="Set bright " $cname " background color"]
                    $vis const [<BR_ $cname _BG>]:&str = concat!("\x1b[","10",$cval,"m");
                )*
            }
        };
}

colors! {
    pub BLACK:"0"
    pub RED:"1"
    pub GREEN:"2"
    pub YELLOW:"3"
    pub BLUE:"4"
    pub MAGENTA:"5"
    pub CYAN:"6"
    pub WHITE:"7"
    pub DEFAULT:"9"
}

/// RGB colors builder
///
/// **Example:**
/// ```
/// use anscape::sequences::colors::Type;
/// use anscape::sequences::colors::rgb;
/// use anscape::sequences::base::ESC;
/// println!("{}{} Hello!",ESC,rgb(255, 255, 255, Type::Fg));
/// ```
pub fn rgb(r: u8, g: u8, b: u8, t: Type) -> String {
    let st;
    match t {
        Type::Fg => {
            st = "3";
        }
        Type::Bg => {
            st = "4";
        }
    }
    format!("\x1b[{}8;2;{};{};{}m", st, r, g, b)
}
/// Foreground RGB colors builder
///
/// **Example:**
/// ```
/// use anscape::sequences::colors::rgb_fg;
/// println!("{}Hello!",rgb_fg(255, 255, 255));
/// ```
pub fn rgb_fg(r: u8, g: u8, b: u8) -> String {
    return rgb(r, g, b, Type::Fg);
}
/// Background RGB colors builder
///
/// **Example:**
/// ```
/// use anscape::sequences::colors::rgb_bg;
/// println!("{}Hello!",rgb_bg(255, 255, 255));
/// ```
pub fn rgb_bg(r: u8, g: u8, b: u8) -> String {
    return rgb(r, g, b, Type::Bg);
}
