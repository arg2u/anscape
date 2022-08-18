//! # Graphics Modes
//! Note: Some terminals may not support some of the graphic mode sequences listed above.

/// # Macros for building basic style constants
///
/// It generate 2 constants per provided style.
///
/// For example, `pub DIM:"2"` will generate:
///
/// `pub const DIM:&str = "\x1b[2m"`
///
/// `pub const DIM_RST:&str = "\x1b[22m"`
///
/// **_RST** - reset style sufix\
///
/// ### Usage example:
/// ```
/// use anscape::styles;
/// styles! {
///     pub DIM:"2"
/// }
/// ```
#[macro_export]
macro_rules! styles {
    ($($vis:vis $cname:ident:$cval:literal)*) => {
        use paste::paste;
        paste!{
            $(
                #[doc="Set " $cname " mode"]
                $vis const $cname:&str = concat!("\x1b[", $cval,"m");
                #[doc="Reset " $cname " mode"]
                $vis const [<$cname _RST>]:&'static str = concat!("\x1b[","2",$cval,"m");
            )*
        }
    };
}

#[doc = "Set BOLD mode"]
pub const BOLD: &str = "\x1b[1m";
#[doc = "Reset BOLD mode"]
pub const BOLD_RST: &str = "\x1b[22m";

styles! {
     pub DIM:"2"
     pub ITALIC:"3"
     pub UNDERLINE:"4"
     pub BLINKING:"5"
     pub INVERSE:"7"
     pub HIDDEN:"8"
     pub STRIKETHROUGH:"9"
}
