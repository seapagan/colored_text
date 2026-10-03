//! A library for adding colors and styles to terminal text output.
//!
//! This library provides a simple and intuitive way to add colors and styles to
//! text in terminal applications. It works with both string literals and owned
//! strings, and supports various text colors, background colors, and text
//! styles.
//!
//! Styling is composed before rendering, so chained calls behave predictably:
//! the most recent foreground/background color wins, text styles accumulate, and
//! ANSI escape codes are emitted only once when the styled value is displayed.
//!
//! # Examples
//!
//! ```rust
//! use colored_text::Colorize;
//!
//! // Basic color usage
//! println!("{}", "Red text".red());
//! println!("{}", "Blue background".on_blue());
//! println!("{}", "Bright black text".bright_black());
//! println!("{}", "Bright black background".on_bright_black());
//!
//! // Combining styles
//! println!("{}", "Bold green text".green().bold());
//!
//! // Using with format! macro
//! let name = "World";
//! println!("{}", format!("Hello, {}!", name.blue().bold()));
//!
//! // ANSI 256, RGB, and Hex colors
//! println!("{}", "ANSI 256 color".ansi256(208));
//! println!("{}", "ANSI 256 background".on_ansi256(236));
//! println!("{}", "RGB color".rgb(255, 128, 0));
//! println!("{}", "Hex color".hex("#ff8000"));
//!
//! // Clearing styles
//! println!("{}", "Plain text".red().bold().clear());
//! ```
//!
//! # Features
//!
//! - Basic colors (red, green, blue, yellow, etc.)
//! - Background colors
//! - Bright color variants
//! - Text styles (bold, dim, italic, underline)
//! - ANSI 256-color foreground and background support
//! - RGB, HSL, and Hex color support
//! - Structured color resolution for custom renderers and TUIs
//! - Terminal color capability detection
//! - RGB, HSL, and Hex degradation when truecolor is unavailable
//! - Composed style chaining
//! - Works with format! macro
//! - Explicit runtime color and color-depth modes
//!
//! # Input Handling
//!
//! - RGB values must be in range 0-255 (enforced at compile time via `u8` type)
//! - Attempting to use RGB values > 255 will result in a compile error
//! - ANSI 256-color indexes must be in range 0-255 (enforced at compile time
//!   via `u8` type)
//! - `.color256(index)` and `.on_color256(index)` are aliases for
//!   `.ansi256(index)` and `.on_ansi256(index)`
//! - HSL values must be finite, with hue in `0..=360` degrees and saturation and
//!   lightness in `0..=100` percent; hue 360 equals 0
//! - Hex accepts ASCII `RGB`, `#RGB`, `RRGGBB`, and `#RRGGBB`, case-insensitively,
//!   with zero or one leading `#` and no whitespace
//! - Invalid HSL or hex passed to styling methods clears all styling and returns
//!   plain unstyled text, including for non-ASCII hex input
//! - Public conversion/resolution functions return [`ColorInputError`] on
//!   invalid HSL or hex input
//! - All color methods are guaranteed to return a valid string, never panicking
//!
//! ```rust
//! use colored_text::Colorize;
//!
//! // Valid hex codes (with or without #)
//! println!("{}", "Valid hex".hex("#ff8000"));
//! println!("{}", "Also valid".hex("ff8000"));
//! println!("{}", "Shorthand".hex("#f80"));
//!
//! // Invalid hex codes return plain text
//! println!("{}", "Invalid hex".hex("xyz")); // Returns plain text
//! println!("{}", "Wrong length".hex("#1234")); // Returns plain text
//! ```
//!
//! # Runtime Color Control
//!
//! The crate supports three runtime modes via [`ColorMode`]:
//!
//! - [`ColorMode::Auto`] enables styling only when stdout is a terminal
//! - [`ColorMode::Always`] forces styling on even when stdout is not a terminal
//! - [`ColorMode::Never`] disables styling completely
//!
//! The `NO_COLOR` environment variable disables styling in all modes, including
//! when `FORCE_COLOR` or an explicit [`ColorDepthMode`] is set.
//! [`ColorMode::Never`] also always disables color output.
//!
//! For normal targets such as [`RenderTarget::Stdout`],
//! [`RenderTarget::Stderr`], and [`RenderTarget::Terminal`], color depth is
//! resolved from environment variables and the current configuration after
//! color output is enabled. Positive [`ColorDepthMode`] values select the depth
//! to use; they do not by themselves bypass [`ColorMode::Auto`] non-terminal
//! suppression. For [`RenderTarget::Capabilities`], the supplied
//! [`TerminalCapabilities`] are used as-is except for the hard disables:
//! `NO_COLOR`, [`ColorMode::Never`], and [`ColorDepthMode::NoColor`].
//!
//! ```rust
//! use colored_text::{ColorDepthMode, ColorMode, Colorize, ColorizeConfig};
//!
//! ColorizeConfig::set_color_mode(ColorMode::Always);
//! println!("{}", "Always colored".red());
//!
//! ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi256);
//! println!("{}", "ANSI 256 orange".rgb(255, 128, 0));
//!
//! ColorizeConfig::set_color_mode(ColorMode::Never);
//! println!("{}", "Never colored".red());
//! ```
//!
//! When you need `Auto` mode to follow a destination other than stdout, use
//! [`StyledText::render`] with a [`RenderTarget`].
//!
//! ```rust
//! use colored_text::{ColorizeConfig, RenderTarget};
//!
//! let caps = ColorizeConfig::terminal_capabilities(RenderTarget::Stdout);
//! println!("stdout color level: {:?}", caps.color_level);
//! ```
//!
//! # Structured Color Resolution
//!
//! Custom renderers can use [`AnsiColor`], [`ResolvedColor`], and the public
//! resolvers without constructing ANSI strings. Capability detection is separate:
//!
//! ```
//! use colored_text::{ColorizeConfig, RenderTarget, ResolvedColor, resolve_rgb};
//!
//! let level = ColorizeConfig::color_level(RenderTarget::Stdout);
//! match resolve_rgb(215, 58, 74, level) {
//!     Some(ResolvedColor::Named(color)) => { /* map the terminal palette entry */ }
//!     Some(ResolvedColor::Ansi256(index)) => { /* map the indexed color */ }
//!     Some(ResolvedColor::Rgb(r, g, b)) => { /* use exact RGB channels */ }
//!     None => { /* leave color unset */ }
//! }
//! ```
//!
//! [`resolve_named`] preserves terminal palette identity at every enabled depth.
//! [`resolve_ansi256`] keeps indexed colors indexed above ANSI 16.
//! [`resolve_rgb`] selects named/indexed/exact RGB according to [`ColorLevel`].
//! [`hsl_to_rgb`] and [`hex_to_rgb`] validate and convert directly to RGB,
//! independently of terminal depth. [`resolve_hsl`] and [`resolve_hex`] delegate
//! to these helpers, then use the same RGB policy.
//! Invalid input returns [`ColorInputError`], even at `NoColor`; valid input
//! with color disabled returns `Ok(None)`. Styling methods use the same
//! validated conversions, clearing all styling on invalid input.
//!
//! # Minimum Supported Rust Version
//!
//! Rust 1.78.0 supports library compilation, all targets/features, examples,
//! dev dependencies, tests, doctests, documentation, builds, and packaging with
//! the checked-in lockfile. Exact stable checks from 1.70.0 upward proved that
//! 1.70.0 through 1.77.0 cannot read lockfile v4, while 1.78.0 passes the complete
//! compiler gate. The immediately preceding 1.77.0 fails because its Cargo
//! cannot read lockfile v4. The lockfile remains unchanged.
//! `Cargo.toml` is the source of truth; dependency updates must preserve the MSRV
//! or explicitly document a support-policy change.
//!
//! # Compatibility with 0.4.1
//!
//! Since `0.4.1`, [`Colorize`] has gained required trait methods for bright
//! foreground and bright background colors. Most users rely on the blanket
//! `impl<T: std::fmt::Display> Colorize for T` and are unaffected. Downstream
//! crates with manual `impl Colorize for ...` blocks must implement the new
//! methods.
//!
//! # Note
//!
//! Colors and styles are implemented using ANSI escape codes, which are
//! supported by most modern terminals. Capability detection is heuristic and
//! environment-based; the crate does not use terminfo, termcap, active terminal
//! queries, WinAPI console enablement, CLI parsing, or runtime dependencies. If
//! color output is disabled by policy, the text is displayed without styling.

mod color;
mod config;
mod resolution;
mod style;
mod terminal;

#[cfg(test)]
mod tests;

pub use color::AnsiColor;
pub use config::{ColorDepthMode, ColorMode, ColorizeConfig, RenderTarget};
pub use resolution::{
    hex_to_rgb, hsl_to_rgb, resolve_ansi256, resolve_hex, resolve_hsl, resolve_named, resolve_rgb,
    ColorInputError, HslComponent, ResolvedColor,
};
pub use style::{Colorize, StyledText};
pub use terminal::{ColorLevel, TerminalCapabilities};
