[![Latest Version](https://img.shields.io/crates/v/anscape.svg)](https://crates.io/crates/anscape) | [Documentation](https://docs.rs/anscape)
===

**Anscape** provides a simple way to customize your terminal window with some styles and colors and manipulate with a cursor position using ANSI Escape Sequences.

## Features

* 256-color mode.
* Cursor movement.
* Text formatting.
* Console size.
* Erasing.

## Examples

### Style and colors

```rust
use anscape::{
    seq::colors::*,
    seq::styles::*, 
    seq::base::*, 
};

fn main() {
    // Basic foregroung
    println!("{}{}Red bold text, {}{} Green Italic{}", RED, BOLD, GREEN, ITALIC, RESET);

    // Basic background
    println!("{}{}Red BG for bold text, {}{} Green BG for italic text{}", RED_BG, BOLD, GREEN_BG, ITALIC, RESET);

    // RGB foreground
    println!("{}Here's red RGB color{}", rgb_fg(255, 0, 0), RESET);

    // RGB background
    println!("{}Here's red RGB BG color{}", rgb_bg(255, 0, 0), RESET);
}
```

### Moving the cursor

```rust
use anscape::seq::cursor;

fn main() {
    println!("{}Move to line 1 col 1", cursor::move_to(100, 100));
}

```

### Erase

```rust
use anscape::seq::erase::*;

fn main() {
    println!("{}Erase", FROM_CURSOR_TO_BEGINING_OF_SCREEN);
}

```

## License

MIT
