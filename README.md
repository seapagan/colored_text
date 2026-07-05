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
- Terminal color capability detection for no-color, ANSI 16, ANSI 256, and
  truecolor output
- Optional color-depth override for applications that know their output target
- Composed style chaining with predictable override behavior
- Works with string literals, owned strings, and format macros
- Zero dependencies
- Supports `NO_COLOR`, `FORCE_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE`, `TERM`,
  `COLORTERM`, `CI`, `WT_SESSION`, `ConEmuANSI`, and `ANSICON`
- Supports explicit runtime color modes: `Auto`, `Always`, and `Never`
- Detects if the output is NOT going to a terminal (e.g. is going to a file or a
  pipe) and disables colors in `Auto` mode unless color is force-enabled
- Supports explicit target-aware rendering for stdout, stderr, or custom
  terminal-aware destinations
- Complete documentation and examples

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colored_text = "0.4.1"
```

## Compatibility with 0.4.1

Since `0.4.1`, `Colorize` has gained required trait methods for bright
foreground and bright background colors. Most users rely on the blanket
`impl<T: Display> Colorize for T` and are unaffected. Downstream crates with
manual `impl Colorize for ...` blocks must implement the new methods.

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

- `.bright_black()`
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
> - `90` uses ANSI bright black / palette slot 8
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

### Bright Background Colors

Bright background methods are available by prefixing bright color names with
`on_`, for example `.on_bright_black()` and `.on_bright_red()`.

They use the standard bright background SGR codes `100-107`.

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

## Environment Color Control

This library respects the [NO_COLOR](https://no-color.org/) environment
variable. If `NO_COLOR` is set (to any value), all color and style methods will
return plain unformatted text. This makes it easy to disable all colors globally
if needed.

`NO_COLOR` is treated as an intentional user opt-out and always disables color
and style output. If `NO_COLOR` is not set, `ColorMode::Never` and
`ColorDepthMode::NoColor` also disable color and style output.

```rust
// Colors enabled (NO_COLOR not set)
println!("{}", "Red text".red()); // Prints in red

// With NO_COLOR set
std::env::set_var("NO_COLOR", "1");
println!("{}", "Red text".red()); // Prints without color
```

Detection is heuristic and environment-based. The crate does not use terminfo,
termcap, WinAPI console enablement, active terminal queries, CLI argument
parsing, or runtime dependencies.

## Runtime Color Modes

By default, this library uses `ColorMode::Auto`: it checks if stdout is going to
a terminal and disables colors when it is not, unless color is force-enabled.
Applications can override that behavior explicitly using `ColorizeConfig`:

```rust
use colored_text::{ColorMode, Colorize, ColorizeConfig};

ColorizeConfig::set_color_mode(ColorMode::Always);
println!("{}", "Always colored".red());

ColorizeConfig::set_color_mode(ColorMode::Never);
println!("{}", "Never colored".red());

ColorizeConfig::set_color_mode(ColorMode::Auto);
println!("{}", "Colored only in terminals".red());
```

Color depth is controlled separately with `ColorDepthMode`:

```rust
use colored_text::{ColorDepthMode, ColorizeConfig};

ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi256);
```

The runtime configuration is thread-local. This is useful in tests or
applications that want to force color on or off for a specific execution path.

Applications can inspect the resolved capability level:

```rust
use colored_text::{ColorizeConfig, RenderTarget};

let caps = ColorizeConfig::terminal_capabilities(RenderTarget::Stdout);
println!("stdout color level: {:?}", caps.color_level);
```

`ColorDepthMode` selects the color depth used after color output has been
enabled. It does not, by itself, force color output in `Auto` mode; use
`ColorMode::Always`, `FORCE_COLOR`, or `CLICOLOR_FORCE` to force-enable output.
When color output is enabled and no limiting depth signal is found, `Auto` on a
terminal and `Always` both preserve full-fidelity truecolor output.

For normal targets (`Stdout`, `Stderr`, and `Terminal(bool)`), color control
precedence is:

1. `NO_COLOR`
2. `ColorMode::Never`
3. `ColorDepthMode::NoColor`
4. `FORCE_COLOR`
5. `CLICOLOR_FORCE`
6. `CLICOLOR=0`, unless force-enabled
7. `Auto` non-terminal suppression, unless force-enabled
8. automatic terminal and environment detection
9. explicit `ColorDepthMode::{Ansi16, Ansi256, TrueColor}`

`NO_COLOR` is presence-based, so even `NO_COLOR=""` disables color. `FORCE_COLOR`
accepts false-like values to disable color and values such as `1`, `2`, `3`,
`ansi16`, `ansi256`, and `truecolor` to force a depth. Explicit positive
`ColorDepthMode` values apply only after color output is enabled and only when
`FORCE_COLOR` is not set. `CLICOLOR_FORCE` follows the common convention that
any non-empty value except `0` force-enables color, with a minimum level of ANSI
16 unless environment hints or explicit `ColorDepthMode` select a higher level.
`CLICOLOR=0` disables color unless overridden by `FORCE_COLOR` or
`CLICOLOR_FORCE`, including when `ColorMode::Always` is set. `TERM=dumb` is an
automatic capability hint, not a user opt-out; an explicit positive
`ColorDepthMode` can override it once color output is enabled.

For `RenderTarget::Capabilities`, the supplied `TerminalCapabilities` are exact:
`FORCE_COLOR`, `CLICOLOR`, and positive `ColorDepthMode` values do not raise or
lower the supplied color level. Only hard disables apply: `NO_COLOR`,
`ColorMode::Never`, and `ColorDepthMode::NoColor`.

For non-stdout destinations, use `StyledText::render` with a `RenderTarget` so
`Auto` mode evaluates the real output target:

```rust
use colored_text::{ColorLevel, Colorize, RenderTarget, TerminalCapabilities};

let warning = "Warning".yellow().bold();

eprintln!("{}", warning.render(RenderTarget::Stderr));

let captured = warning.render(RenderTarget::Terminal(false));
assert_eq!(captured, "Warning");

let exact = warning.render(RenderTarget::Capabilities(TerminalCapabilities {
    is_terminal: true,
    color_level: ColorLevel::Ansi256,
}));
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
- Named color methods such as `.red()` always emit named ANSI SGR codes when
  color is enabled
- RGB, hex, and HSL colors degrade to ANSI 256 or named ANSI colors when the
  resolved color level does not support truecolor
- ANSI 256 colors degrade to named ANSI colors when the resolved color level is
  ANSI 16
- Some styling options (like italic) might not work in all terminals

## Examples

Check out the [examples](examples/) directory for more usage examples.

Right now we only have one example `basic.rs`. You can run this using:

```bash
cargo run --example basic
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file
for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.
