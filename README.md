# colored_text

[![Crates.io](https://img.shields.io/crates/v/colored_text.svg)](https://crates.io/crates/colored_text)
[![Documentation](https://docs.rs/colored_text/badge.svg)](https://docs.rs/colored_text)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and intuitive library for adding colors and styles to terminal text in Rust.

## Features

- Simple method-call syntax for applying colors and styles
- Support for basic colors, bright colors, and background colors
- Text styling (bold, dim, italic, underline, inverse, strikethrough)
- RGB and HEX color support for both text and background
- Style chaining
- Works with string literals, owned strings, and format macros
- Zero dependencies
- Supports the `NO_COLOR` environment variable - if this is set, all colors are
  disabled and the text is returned uncolored
- Complete documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colored_text = "0.2.0"
```

## Usage

```rust
use colored_text::Colorize;

// Basic colors
println!("{}", "Red text".red());
println!("{}", "Blue text".blue());
println!("{}", "Green text".green());

// Background colors
println!("{}", "Red background".on_red());
println!("{}", "Blue background".on_blue());

// Text styles
println!("{}", "Bold text".bold());
println!("{}", "Italic text".italic());
println!("{}", "Underlined text".underline());

// RGB and Hex colors
println!("{}", "Custom color".rgb(255, 128, 0));
println!("{}", "Custom background".on_rgb(0, 128, 255));
println!("{}", "Hex color".hex("#ff8000"));
println!("{}", "Hex background".on_hex("#0080ff"));

// Chaining styles
println!("{}", "Bold red text".red().bold());
println!("{}", "Italic blue on yellow".blue().italic().on_yellow());

// Using with format! macro
let name = "World";
println!("{}", format!("Hello, {}!", name.blue().bold()));
```

## Available Methods

### Colors

- `.red()`
- `.green()`
- `.blue()`
- `.yellow()`
- `.magenta()`
- `.cyan()`
- `.white()`
- `.black()`

### Bright Colors

- `.bright_red()`
- `.bright_green()`
- `.bright_blue()`
- `.bright_yellow()`
- `.bright_magenta()`
- `.bright_cyan()`
- `.bright_white()`

### Background Colors

- `.on_red()`
- `.on_green()`
- `.on_blue()`
- `.on_yellow()`
- `.on_magenta()`
- `.on_cyan()`
- `.on_white()`
- `.on_black()`

### Styles

- `.bold()`
- `.dim()`
- `.italic()`
- `.underline()`
- `.inverse()` - Swap foreground and background colors
- `.strikethrough()` - Draw a line through the text

### RGB, HSL, and Hex Colors

- `.rgb(r, g, b)` - Custom text color using RGB values (0-255, compile-time enforced)
- `.on_rgb(r, g, b)` - Custom background color using RGB values (0-255, compile-time enforced)
- `.hsl(h, s, l)` - Custom text color using HSL values (hue: 0-360°, saturation: 0-100%, lightness: 0-100%)
- `.on_hsl(h, s, l)` - Custom background color using HSL values
- `.hex(code)` - Custom text color using HTML/CSS hex code (e.g., "#ff8000" or "ff8000")
- `.on_hex(code)` - Custom background color using HTML/CSS hex code

### Other

- `.clear()` - Remove all styling

## Input Handling and Validation

- RGB values must be in range 0-255 (enforced at compile time via `u8` type)
- Attempting to use RGB values > 255 will result in a compile error
- Hex color codes can be provided with or without the '#' prefix
- Invalid hex codes (wrong length, invalid characters) will result in uncolored text
- All color methods are guaranteed to return a valid string, never panicking

```rust
// RGB values are constrained to 0-255
println!("{}", "RGB color".rgb(255, 128, 0));

// HSL values (hue: 0-360°, saturation/lightness: 0-100%)
println!("{}", "Red".hsl(0.0, 100.0, 50.0));     // Pure red
println!("{}", "Green".hsl(120.0, 100.0, 50.0)); // Pure green
println!("{}", "Blue".hsl(240.0, 100.0, 50.0));  // Pure blue
println!("{}", "Gray".hsl(0.0, 0.0, 50.0));      // 50% gray

// Hex colors work with or without #
println!("{}", "Hex color".hex("#ff8000"));
println!("{}", "Also valid".hex("ff8000"));

// Invalid hex codes return uncolored text
println!("{}", "Invalid".hex("xyz")); // Returns uncolored text
println!("{}", "Too short".hex("#f8")); // Returns uncolored text
```

## NO_COLOR Support

This library respects the [NO_COLOR](https://no-color.org/) environment variable. If `NO_COLOR` is set (to any value), all color and style methods will return plain unformatted text. This makes it easy to disable all colors globally if needed.

```rust
// Colors enabled (NO_COLOR not set)
println!("{}", "Red text".red()); // Prints in red

// With NO_COLOR set
std::env::set_var("NO_COLOR", "1");
println!("{}", "Red text".red()); // Prints without color
```

## Terminal Detection Configuration

By default, this library checks if the output is going to a terminal and disables colors when it's not (e.g., when piping output to a file). This behavior can be controlled using `ColorizeConfig`:

```rust
use colored_text::{Colorize, ColorizeConfig};

// Disable terminal detection (colors will be enabled regardless of terminal status)
ColorizeConfig::set_terminal_check(false);
println!("{}", "Always colored".red());

// Re-enable terminal detection (default behavior)
ColorizeConfig::set_terminal_check(true);
println!("{}", "Only colored in terminal".red());
```

This is particularly useful in test environments where you might want to force-enable colors regardless of the terminal status. The configuration is thread-local, making it safe to use in parallel tests without affecting other threads.

Note: Even when terminal detection is disabled, the `NO_COLOR` environment variable still takes precedence - if it's set, colors will be disabled regardless of this setting.

## Terminal Compatibility

This library uses ANSI escape codes for coloring and styling text. Most modern terminals support these codes, but the actual appearance may vary depending on your terminal emulator and its configuration:

- Basic colors (codes 30-37) are widely supported
- Bright colors (codes 90-97) may appear the same as basic colors in some terminals
- RGB colors require true color support in your terminal
- Some styling options (like italic) might not work in all terminals

## Examples

Check out the [examples](examples/) directory for more usage examples.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
