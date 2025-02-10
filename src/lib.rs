//! A library for adding colors and styles to terminal text output.
//!
//! This library provides a simple and intuitive way to add colors and styles to text
//! in terminal applications. It works with both string literals and owned strings,
//! and supports various text colors, background colors, and text styles.
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
//! // RGB colors
//! println!("{}", "Custom color".rgb(255, 128, 0));
//! ```
//!
//! # Features
//!
//! - Basic colors (red, green, blue, yellow, etc.)
//! - Background colors
//! - Bright color variants
//! - Text styles (bold, dim, italic, underline)
//! - RGB color support
//! - Style chaining
//! - Works with format! macro
//!
//! # Note
//!
//! Colors and styles are implemented using ANSI escape codes, which are supported
//! by most modern terminals. If your terminal doesn't support ANSI escape codes,
//! the text will be displayed without styling.

/// Helper function to convert a hex color string to RGB values
fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
        return None;
    }

    let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
    let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
    let b = u8::from_str_radix(&hex[4..6], 16).ok()?;

    Some((r, g, b))
}

/// Trait for adding color and style methods to strings.
///
/// This trait provides methods to colorize and style text for terminal output.
/// It can be used with both string literals and owned strings.
pub trait Colorize {
    /// Returns a colored version of the string
    fn colorize(&self, color_code: &str) -> String;

    // Basic colors
    fn red(&self) -> String;
    fn green(&self) -> String;
    fn yellow(&self) -> String;
    fn blue(&self) -> String;
    fn magenta(&self) -> String;
    fn cyan(&self) -> String;
    fn white(&self) -> String;
    fn black(&self) -> String;

    // Bright colors
    fn bright_red(&self) -> String;
    fn bright_green(&self) -> String;
    fn bright_yellow(&self) -> String;
    fn bright_blue(&self) -> String;
    fn bright_magenta(&self) -> String;
    fn bright_cyan(&self) -> String;
    fn bright_white(&self) -> String;

    // Styles
    fn bold(&self) -> String;
    fn dim(&self) -> String;
    fn italic(&self) -> String;
    fn underline(&self) -> String;

    // Background colors
    fn on_red(&self) -> String;
    fn on_green(&self) -> String;
    fn on_yellow(&self) -> String;
    fn on_blue(&self) -> String;
    fn on_magenta(&self) -> String;
    fn on_cyan(&self) -> String;
    fn on_white(&self) -> String;
    fn on_black(&self) -> String;

    // RGB and Hex color support
    fn rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> String;
    fn hex(&self, hex: &str) -> String;
    fn on_hex(&self, hex: &str) -> String;

    // Clear all formatting
    fn clear(&self) -> String;
}

impl<T: std::fmt::Display> Colorize for T {
    fn colorize(&self, color_code: &str) -> String {
        format!("\x1b[{}m{}\x1b[0m", color_code, self)
    }

    fn red(&self) -> String {
        self.colorize("31")
    }
    fn green(&self) -> String {
        self.colorize("32")
    }
    fn yellow(&self) -> String {
        self.colorize("33")
    }
    fn blue(&self) -> String {
        self.colorize("34")
    }
    fn magenta(&self) -> String {
        self.colorize("35")
    }
    fn cyan(&self) -> String {
        self.colorize("36")
    }
    fn white(&self) -> String {
        self.colorize("37")
    }
    fn black(&self) -> String {
        self.colorize("30")
    }

    fn bright_red(&self) -> String {
        self.colorize("91")
    }
    fn bright_green(&self) -> String {
        self.colorize("92")
    }
    fn bright_yellow(&self) -> String {
        self.colorize("93")
    }
    fn bright_blue(&self) -> String {
        self.colorize("94")
    }
    fn bright_magenta(&self) -> String {
        self.colorize("95")
    }
    fn bright_cyan(&self) -> String {
        self.colorize("96")
    }
    fn bright_white(&self) -> String {
        self.colorize("97")
    }

    fn bold(&self) -> String {
        self.colorize("1")
    }
    fn dim(&self) -> String {
        self.colorize("2")
    }
    fn italic(&self) -> String {
        self.colorize("3")
    }
    fn underline(&self) -> String {
        self.colorize("4")
    }

    fn on_red(&self) -> String {
        self.colorize("41")
    }
    fn on_green(&self) -> String {
        self.colorize("42")
    }
    fn on_yellow(&self) -> String {
        self.colorize("43")
    }
    fn on_blue(&self) -> String {
        self.colorize("44")
    }
    fn on_magenta(&self) -> String {
        self.colorize("45")
    }
    fn on_cyan(&self) -> String {
        self.colorize("46")
    }
    fn on_white(&self) -> String {
        self.colorize("47")
    }
    fn on_black(&self) -> String {
        self.colorize("40")
    }

    fn rgb(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn on_rgb(&self, r: u8, g: u8, b: u8) -> String {
        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn hex(&self, hex: &str) -> String {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.rgb(r, g, b)
        } else {
            self.clear() // Return uncolored text if hex code is invalid
        }
    }

    fn on_hex(&self, hex: &str) -> String {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.on_rgb(r, g, b)
        } else {
            self.clear() // Return uncolored text if hex code is invalid
        }
    }

    fn clear(&self) -> String {
        format!("\x1b[0m{}\x1b[0m", self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_colors() {
        assert_eq!("test".red(), "\x1b[31mtest\x1b[0m");
        assert_eq!("test".blue(), "\x1b[34mtest\x1b[0m");
        assert_eq!("test".green(), "\x1b[32mtest\x1b[0m");
    }

    #[test]
    fn test_bright_colors() {
        assert_eq!("test".bright_red(), "\x1b[91mtest\x1b[0m");
        assert_eq!("test".bright_blue(), "\x1b[94mtest\x1b[0m");
    }

    #[test]
    fn test_background_colors() {
        assert_eq!("test".on_red(), "\x1b[41mtest\x1b[0m");
        assert_eq!("test".on_blue(), "\x1b[44mtest\x1b[0m");
    }

    #[test]
    fn test_styles() {
        assert_eq!("test".bold(), "\x1b[1mtest\x1b[0m");
        assert_eq!("test".italic(), "\x1b[3mtest\x1b[0m");
        assert_eq!("test".underline(), "\x1b[4mtest\x1b[0m");
    }

    #[test]
    fn test_rgb_colors() {
        assert_eq!("test".rgb(255, 128, 0), "\x1b[38;2;255;128;0mtest\x1b[0m");
        assert_eq!(
            "test".on_rgb(255, 128, 0),
            "\x1b[48;2;255;128;0mtest\x1b[0m"
        );
    }

    #[test]
    fn test_string_and_str() {
        let string = String::from("test");
        assert_eq!(string.red(), "test".red());
        assert_eq!(string.blue(), "test".blue());
    }

    #[test]
    fn test_format_macro() {
        assert_eq!(format!("{}", "test".red()), format!("\x1b[31mtest\x1b[0m"));
    }

    #[test]
    fn test_chaining() {
        assert_eq!("test".red().bold(), "\x1b[1m\x1b[31mtest\x1b[0m\x1b[0m");
    }

    #[test]
    fn test_hex_colors() {
        assert_eq!("test".hex("#ff8000"), "\x1b[38;2;255;128;0mtest\x1b[0m");
        assert_eq!("test".hex("ff8000"), "\x1b[38;2;255;128;0mtest\x1b[0m");
        assert_eq!("test".on_hex("#0080ff"), "\x1b[48;2;0;128;255mtest\x1b[0m");
    }

    #[test]
    fn test_invalid_hex() {
        // Should return uncolored text for invalid hex codes
        assert_eq!("test".hex("invalid"), "\x1b[0mtest\x1b[0m");
        assert_eq!("test".hex("#12"), "\x1b[0mtest\x1b[0m");
        assert_eq!("test".on_hex("not-a-color"), "\x1b[0mtest\x1b[0m");
    }
}
