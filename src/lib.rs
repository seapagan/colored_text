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
//!
//! // Combining styles
//! println!("{}", "Bold green text".green().bold());
//!
//! // Using with format! macro
//! let name = "World";
//! println!("{}", format!("Hello, {}!", name.blue().bold()));
//!
//! // RGB and Hex colors
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
//! - RGB, HSL, and Hex color support
//! - Composed style chaining
//! - Works with format! macro
//! - Explicit runtime color modes
//!
//! # Input Handling
//!
//! - RGB values must be in range 0-255 (enforced at compile time via `u8` type)
//! - Attempting to use RGB values > 255 will result in a compile error
//! - Hex color codes can be provided with or without the `#` prefix in 3-digit
//!   shorthand or 6-digit full form
//! - Invalid hex codes (wrong length or invalid characters) return plain
//!   unstyled text
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
//! The `NO_COLOR` environment variable disables styling in `Auto` and
//! `Always`.
//!
//! ```rust
//! use colored_text::{ColorMode, Colorize, ColorizeConfig};
//!
//! ColorizeConfig::set_color_mode(ColorMode::Always);
//! println!("{}", "Always colored".red());
//!
//! ColorizeConfig::set_color_mode(ColorMode::Never);
//! println!("{}", "Never colored".red());
//! ```
//!
//! When you need `Auto` mode to follow a destination other than stdout, use
//! [`StyledText::render`] with a [`RenderTarget`].
//!
//! # Note
//!
//! Colors and styles are implemented using ANSI escape codes, which are
//! supported by most modern terminals. If your terminal does not support ANSI
//! escape codes, or if color output is disabled by policy, the text is
//! displayed without styling.

mod color;
mod config;
mod style;

#[cfg(test)]
mod tests;

pub use config::{ColorMode, ColorizeConfig, RenderTarget};
pub use style::{Colorize, StyledText};
