# colored_text

[![Crates.io](https://img.shields.io/crates/v/colored_text.svg)](https://crates.io/crates/colored_text)
[![Documentation](https://docs.rs/colored_text/badge.svg)](https://docs.rs/colored_text)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A simple and intuitive library for adding colors and styles to terminal text in
Rust.

## Features

- Simple method-call syntax for applying colors and styles
- Support for basic colors, bright colors, and background colors
- Text styling (bold, dim, italic, underline, inverse, strikethrough)
- ANSI 256, RGB, and HEX color support for both text and background
- Composed style chaining with predictable override behavior
- Works with string literals, owned strings, and format macros
- Zero dependencies
- Supports the `NO_COLOR` environment variable - if this is set, all colors are
  disabled and the text is returned uncolored
- Supports explicit runtime color modes: `Auto`, `Always`, and `Never`
- Detects if the output is NOT going to a terminal (e.g. is going to a file or a
  pipe) and disables colors in `Auto` mode
- Supports explicit target-aware rendering for stdout, stderr, or custom
  terminal-aware destinations
- Complete documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colored_text = "0.4.1"
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

// ANSI 256, RGB, and Hex colors
println!("{}", "ANSI 256 color".ansi256(208));
println!("{}", "ANSI 256 background".on_ansi256(236));
println!("{}", "Custom color".rgb(255, 128, 0));
println!("{}", "Custom background".on_rgb(0, 128, 255));
println!("{}", "Hex color".hex("#ff8000"));
println!("{}", "Hex background".on_hex("#0080ff"));

// Chaining styles
println!("{}", "Bold red text".red().bold());
println!("{}", "Italic blue on yellow".blue().italic().on_yellow());

// Using with format! macro
let name = "World";
println!("Hello, {}!", name.blue().bold());

// Removing all styles
println!("{}", "Back to plain text".red().bold().clear());
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

> [!IMPORTANT]
>
> Bright ANSI colours use the standard SGR codes `90–97`. Their final appearance depends on the terminal emulator’s active colour palette.
>
> Some themes, especially soft/pastel themes such as Catppuccin, may make bright colours appear very close to the normal ANSI colours. This does not mean the escape codes are wrong; it means the terminal palette maps those colour slots similarly.
>
> For example:
>
> - `31` uses ANSI red / palette slot 1
> - `91` uses bright ANSI red / palette slot 9
> - `38;5;1` uses 256-colour index 1
> - `38;5;9` uses 256-colour index 9

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

### ANSI 256, RGB, HSL, and Hex Colors

- `.ansi256(index)` - Custom text color using an ANSI 256-color index (0-255,
  compile-time enforced)
- `.on_ansi256(index)` - Custom background color using an ANSI 256-color index
  (0-255, compile-time enforced)
- `.color256(index)` - Alias for `.ansi256(index)`
- `.on_color256(index)` - Alias for `.on_ansi256(index)`
- `.rgb(r, g, b)` - Custom text color using RGB values (0-255, compile-time
  enforced)
- `.on_rgb(r, g, b)` - Custom background color using RGB values (0-255,
  compile-time enforced)
- `.hsl(h, s, l)` - Custom text color using HSL values (hue: 0-360°, saturation:
  0-100%, lightness: 0-100%)
- `.on_hsl(h, s, l)` - Custom background color using HSL values
- `.hex(code)` - Custom text color using HTML/CSS hex code (e.g., "#ff8000" or
  "ff8000")
- `.on_hex(code)` - Custom background color using HTML/CSS hex code

### Other

- `.clear()` - Remove all styling

## Input Handling and Validation

- RGB values must be in range 0-255 (enforced at compile time via `u8` type)
- Attempting to use RGB values > 255 will result in a compile error
- ANSI 256-color indexes must be in range 0-255 (enforced at compile time via
  `u8` type)
- Hex color codes can be provided with or without the '#' prefix in either
  3-character shorthand or 6-character full form
- Invalid hex codes (wrong length, invalid characters) will result in plain
  unstyled text
- All color methods are guaranteed to return a valid string, never panicking

```rust
// RGB values are constrained to 0-255
println!("{}", "RGB color".rgb(255, 128, 0));

// HSL values (hue: 0-360°, saturation/lightness: 0-100%)
println!("{}", "Red".hsl(0.0, 100.0, 50.0));     // Pure red
println!("{}", "Green".hsl(120.0, 100.0, 50.0)); // Pure green
println!("{}", "Blue".hsl(240.0, 100.0, 50.0));  // Pure blue
println!("{}", "Gray".hsl(0.0, 0.0, 50.0));      // 50% gray

// ANSI 256-color indexes use SGR 38;5/48;5 output
println!("{}", "Orange".ansi256(208));
println!("{}", "Dark background".on_ansi256(236));
println!("{}", "Alias".color256(208).on_color256(236));

// Hex colors work with or without #
println!("{}", "Hex color".hex("#ff8000"));
println!("{}", "Also valid".hex("ff8000"));
println!("{}", "Shorthand".hex("#f80"));

// Invalid hex codes return uncolored text
println!("{}", "Invalid".hex("xyz")); // Returns uncolored text
println!("{}", "Wrong length".hex("#1234")); // Returns uncolored text
```

## NO_COLOR Support

This library respects the [NO_COLOR](https://no-color.org/) environment
variable. If `NO_COLOR` is set (to any value), all color and style methods will
return plain unformatted text. This makes it easy to disable all colors globally
if needed.

```rust
// Colors enabled (NO_COLOR not set)
println!("{}", "Red text".red()); // Prints in red

// With NO_COLOR set
std::env::set_var("NO_COLOR", "1");
println!("{}", "Red text".red()); // Prints without color
```

## Runtime Color Modes

By default, this library uses `ColorMode::Auto`: it checks if stdout is going to
a terminal and disables colors when it is not. Applications can override that
behavior explicitly using `ColorizeConfig`:

```rust
use colored_text::{ColorMode, Colorize, ColorizeConfig};

ColorizeConfig::set_color_mode(ColorMode::Always);
println!("{}", "Always colored".red());

ColorizeConfig::set_color_mode(ColorMode::Never);
println!("{}", "Never colored".red());

ColorizeConfig::set_color_mode(ColorMode::Auto);
println!("{}", "Colored only in terminals".red());
```

The runtime configuration is thread-local. This is useful in tests or
applications that want to force color on or off for a specific execution path.

`NO_COLOR` still takes precedence in `Auto` and `Always` mode. If `NO_COLOR` is
set, output is plain text.

ANSI 256-color methods use the same runtime policy as the named, RGB, HSL, and
hex color methods. `ColorMode` and `NO_COLOR` control whether ANSI 256 SGR
output is emitted.

For non-stdout destinations, use `StyledText::render` with a `RenderTarget` so
`Auto` mode evaluates the real output target:

```rust
use colored_text::{Colorize, RenderTarget};

let warning = "Warning".yellow().bold();

eprintln!("{}", warning.render(RenderTarget::Stderr));

let captured = warning.render(RenderTarget::Terminal(false));
assert_eq!(captured, "Warning");
```

## Terminal Compatibility

This library uses ANSI escape codes for coloring and styling text. Most modern
terminals support these codes, but the actual appearance may vary depending on
your terminal emulator and its configuration:

- Basic colors (codes 30-37) are widely supported
- Bright colors (codes 90-97) may appear the same as basic colors in some
  terminals or themes (pastel themes like Catppuccin especially)
- ANSI 256 colors use 256-color palette indexes with `38;5` and `48;5` SGR
  sequences
- RGB colors require true color support in your terminal
- Some styling options (like italic) might not work in all terminals

## Examples

Check out the [examples](examples/) directory for more usage examples.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
