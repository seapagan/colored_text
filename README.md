# colored_text

[![Crates.io](https://img.shields.io/crates/v/colored_text.svg)](https://crates.io/crates/colored_text)
[![Documentation](https://docs.rs/colored_text/badge.svg)](https://docs.rs/colored_text)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Add colors and styles to terminal text in Rust.

## Features

- Method-call syntax for applying colors and styles
- Support for basic colors, bright colors, and background colors
- Text styling (bold, dim, italic, underline, inverse, strikethrough)
- ANSI 256, RGB, HSL, and hex color support for both text and background
- Terminal color capability detection for no-color, ANSI 16, ANSI 256, and
  truecolor output
- Optional color-depth override for applications that know their output target
- Composed style chaining with predictable override behavior
- Works with string literals, owned strings, and format macros
- Zero runtime dependencies
- Structured color resolution for custom renderers, TUIs, loggers, and adapters
- Supports `NO_COLOR`, `FORCE_COLOR`, `CLICOLOR`, `CLICOLOR_FORCE`, `TERM`,
  `COLORTERM`, `CI`, `WT_SESSION`, `ConEmuANSI`, and `ANSICON`
- Runtime color modes: `Auto`, `Always`, and `Never`; `Auto` disables colors
  for files and pipes unless color is forced
- Target-aware rendering for stdout, stderr, and custom destinations

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
colored_text = "0.6.0"
```

## Minimum Supported Rust Version

Minimum supported Rust version: **1.70.0**.

## Upgrading from versions before 0.5.0

In 0.5.0, `Colorize` gained required bright-background methods such as
`on_bright_red()`. Most users rely on the blanket `impl<T: Display> Colorize for
T` and are unaffected. Downstream crates with manual `impl Colorize for ...`
blocks written against earlier versions must implement the new methods.

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

// ANSI 256, RGB, HSL, and Hex colors
println!("{}", "ANSI 256 color".ansi256(208));
println!("{}", "ANSI 256 background".on_ansi256(236));
println!("{}", "Custom color".rgb(255, 128, 0));
println!("{}", "Custom background".on_rgb(0, 128, 255));
println!("{}", "HSL color".hsl(0.0, 100.0, 50.0));
println!("{}", "HSL background".on_hsl(200.0, 100.0, 50.0));
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

## Renderer-neutral Color Resolution

Use the public resolvers when a renderer needs color data instead of ANSI
escape sequences. Determine a `ColorLevel` once, then supply it explicitly:

```rust
use colored_text::{resolve_rgb, ColorizeConfig, RenderTarget, ResolvedColor};

let level = ColorizeConfig::color_level(RenderTarget::Stdout);
match resolve_rgb(0xd7, 0x3a, 0x4a, level) {
    Some(ResolvedColor::Named(color)) => {
        // Map AnsiColor to the renderer's terminal palette entry.
        println!("named palette entry: {color:?}");
    }
    Some(ResolvedColor::Ansi256(index)) => {
        println!("indexed palette entry: {index}");
    }
    Some(ResolvedColor::Rgb(r, g, b)) => {
        println!("RGB channels: {r}, {g}, {b}");
    }
    None => {
        // Leave the renderer's color unset.
    }
}
```

`AnsiColor` exposes all 16 standard and bright terminal palette entries.
Their visual appearance depends on the terminal theme. Named input preserves
that identity at every enabled depth; indexed input stays indexed at both
`Ansi256` and `TrueColor`. RGB input resolves to the nearest named/indexed
color or exact RGB, according to the level. `NoColor` returns `None`.

All supported input forms have public entry points:

```rust
use colored_text::{
    resolve_named, resolve_ansi256, resolve_rgb, resolve_hsl, resolve_hex,
    AnsiColor, ColorLevel,
};

fn main() -> Result<(), colored_text::ColorInputError> {
    let level = ColorLevel::Ansi256; // Or use ColorizeConfig::color_level(target).
    let named = resolve_named(AnsiColor::BrightRed, level);
    let indexed = resolve_ansi256(196, level);
    let rgb = resolve_rgb(215, 58, 74, level);
    let hsl = resolve_hsl(0.0, 100.0, 50.0, level)?;
    let hex = resolve_hex("#d73a4a", level)?;
    Ok(())
}
```

Use `hsl_to_rgb(h, s, l)` and `hex_to_rgb(input)` to convert colors without
selecting a terminal depth. Both return `Result<(u8, u8, u8), ColorInputError>`.
The converters and resolvers do not read configuration or the environment.

`ColorInputError::InvalidHsl { component, value }` identifies the rejected
component and value. Matches on `ColorInputError` need a wildcard arm because
it is non-exhaustive; `AnsiColor` and `ResolvedColor` are exhaustive.
See the [API reference](https://docs.rs/colored_text) for type and conversion
details.

```rust
use colored_text::{hex_to_rgb, hsl_to_rgb};

assert_eq!(hsl_to_rgb(360.0, 100.0, 50.0).unwrap(), (255, 0, 0));
assert_eq!(hex_to_rgb("#aBc").unwrap(), (170, 187, 204));
```

`resolve_hsl` and `resolve_hex` use the same color-depth conversion as
`StyledText`. Invalid input returns `ColorInputError`, even at `NoColor`;
valid input at `NoColor` returns `Ok(None)`. See
[input validation](#input-handling-and-validation) for accepted formats and the
styling methods' fallback behavior.

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

Bright colors use the terminal's palette and may resemble normal colors in
some themes. See [terminal compatibility](#terminal-compatibility).

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

- `.ansi256(index)` - Text color from an ANSI 256-color index
- `.on_ansi256(index)` - Background color from an ANSI 256-color index
- `.color256(index)` - Alias for `.ansi256(index)`
- `.on_color256(index)` - Alias for `.on_ansi256(index)`
- `.rgb(r, g, b)` - Text color from RGB values
- `.on_rgb(r, g, b)` - Background color from RGB values
- `.hsl(h, s, l)` - Text color from HSL values
- `.on_hsl(h, s, l)` - Background color from HSL values
- `.hex(code)` - Text color from a hex value
- `.on_hex(code)` - Background color from a hex value

### Other

- `.clear()` - Remove all styling

## Input Handling and Validation

- RGB channels and ANSI 256-color indexes use `u8`, which enforces the range
  `0..=255` at compile time
- HSL values must be finite, with hue in `0..=360` degrees and saturation and
  lightness in `0..=100` percent; hue 360 equals 0
- Hex accepts ASCII `RGB`, `#RGB`, `RRGGBB`, and `#RRGGBB`, case-insensitively,
  with exactly zero or one leading `#` and no whitespace
- Invalid HSL or hex passed to styling methods clears all styling and returns
  plain unstyled text, including for non-ASCII hex input
- Public conversion/resolution functions return `ColorInputError` on invalid
  HSL or hex input
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

Setting [`NO_COLOR`](https://no-color.org/) to any value, including an empty
string, disables all color and style output, even when color is forced.
`ColorMode::Never` and `ColorDepthMode::NoColor` also disable color and styles.

```rust
// Colors enabled (NO_COLOR not set)
println!("{}", "Red text".red()); // Prints in red

// With NO_COLOR set
std::env::set_var("NO_COLOR", "1");
println!("{}", "Red text".red()); // Prints without color
```

Detection is heuristic and environment-based. The crate does not use terminfo,
termcap, WinAPI console enablement, active terminal queries, CLI argument
parsing.

## Runtime Color Modes

The default `ColorMode::Auto` enables color for terminal output or when color
is forced. Use `ColorizeConfig` to select a mode:

```rust
use colored_text::{ColorMode, Colorize, ColorizeConfig};

ColorizeConfig::set_color_mode(ColorMode::Always);
println!("{}", "Always colored".red());

ColorizeConfig::set_color_mode(ColorMode::Never);
println!("{}", "Never colored".red());

ColorizeConfig::set_color_mode(ColorMode::Auto);
println!("{}", "Colored only in terminals".red());
```

Select color depth with `ColorDepthMode`:

```rust
use colored_text::{ColorDepthMode, ColorizeConfig};

ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi256);
```

Runtime configuration is thread-local, so changes affect only the current thread.

Applications can inspect the resolved capability level:

```rust
use colored_text::{ColorizeConfig, RenderTarget};

let caps = ColorizeConfig::terminal_capabilities(RenderTarget::Stdout);
println!("stdout color level: {:?}", caps.color_level);
```

`ColorDepthMode` selects depth after color output is enabled. To force color
in `Auto` mode, use `ColorMode::Always`, `FORCE_COLOR`, or `CLICOLOR_FORCE`.
Enabled output falls back to named ANSI 16 colors when no usable depth signal
is available.

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

- `FORCE_COLOR` accepts false-like values to disable color, or `1`, `2`, `3`,
  `ansi16`, `ansi256`, and `truecolor` to force a depth. It takes precedence over
  positive `ColorDepthMode` settings.
- `CLICOLOR_FORCE` enables color for any non-empty value except `0`, at ANSI 16
  or a higher depth selected by environment hints or `ColorDepthMode`.
- `CLICOLOR=0` disables color even with `ColorMode::Always`, unless
  `FORCE_COLOR` or `CLICOLOR_FORCE` overrides it.
- `TERM=dumb` is a capability hint. A positive `ColorDepthMode` can override
  it after color output is enabled.

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

Run the [basic example](examples/basic.rs):

```bash
cargo run --example basic
```

## License

[MIT](LICENSE).

## Contributing

See [CONTRIBUTING.md](https://github.com/seapagan/colored_text/blob/main/CONTRIBUTING.md)
for development setup and contribution guidelines.
