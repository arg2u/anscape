**Anscape** provides a simple ways to customize your terminal window with styles and a cursor manipulations using ANSI Escape Sequences.

## Features

* 256-color mode.
* Cursor movement.
* Text formatting.
* Console size.
* Erasing.

## Examples

### Style and colors.

```rust
use anscape::{
    sequences::colors,
    sequences::colors::codes::*, 
    sequences::styles::set::*, 
    styled_print,
};

fn main() {
    styled_print("Red text", &[&colors::basic_fg(RED, false)[..], BOLD]);
    println!(
        "{}Alternative with println",
        anscape::build(&[&colors::basic_fg(RED, false), BOLD])
    )
}
```

### Moving the cursor

```rust
use anscape::sequences::cursor;

fn main() {
    println!("{}Move to line 1 col 1", cursor::move_to(100, 100));
}

```

### Erase

```rust
use anscape::sequences::erase;

fn main() {
    println!("{}Erase", anscape::build(&[FROM_CURSOR_TO_BEGINING_OF_SCREEN]));
}

```

## License

MIT
