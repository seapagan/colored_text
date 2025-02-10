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
//! // RGB and Hex colors
//! println!("{}", "RGB color".rgb(255, 128, 0));
//! println!("{}", "Hex color".hex("#ff8000"));
//! ```
//!
//! # Features
//!
//! - Basic colors (red, green, blue, yellow, etc.)
//! - Background colors
//! - Bright color variants
//! - Text styles (bold, dim, italic, underline)
//! - RGB and Hex color support
//! - Style chaining
//! - Works with format! macro
//!
//! # Input Handling
//!
//! - RGB values must be in range 0-255 (enforced at compile time via `u8` type)
//! - Attempting to use RGB values > 255 will result in a compile error
//! - Hex color codes can be provided with or without the '#' prefix
//! - Invalid hex codes (wrong length, invalid characters) will result in uncolored text
//! - All color methods are guaranteed to return a valid string, never panicking
//!
//! ```rust
//! use colored_text::Colorize;
//!
//! // Valid hex codes (with or without #)
//! println!("{}", "Valid hex".hex("#ff8000"));
//! println!("{}", "Also valid".hex("ff8000"));
//!
//! // Invalid hex codes return uncolored text
//! println!("{}", "Invalid hex".hex("xyz")); // Returns uncolored text
//! println!("{}", "Too short".hex("#f8")); // Returns uncolored text
//! ```
//!
//! # Note
//!
//! Colors and styles are implemented using ANSI escape codes, which are supported
//! by most modern terminals. If your terminal doesn't support ANSI escape codes,
//! the text will be displayed without styling.

/// Check if colors should be applied based on NO_COLOR environment variable.
/// Returns false if NO_COLOR is set (any value), true otherwise.
fn should_colorize() -> bool {
    std::env::var("NO_COLOR").is_err()
}

/// Convert HSL color values to RGB.
/// - h: Hue (0-360 degrees)
/// - s: Saturation (0-100 percent)
/// - l: Lightness (0-100 percent)
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    // Normalize to 0-1
    let h = h / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    // Calculate intermediate values
    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    // Convert to RGB based on hue segment
    let (r, g, b) = match (h * 6.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    // Convert to 0-255 range
    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

/// Helper function to convert a hex color string to RGB values.
/// Returns None for invalid hex codes:
/// - Must be 6 characters (not counting optional # prefix)
/// - Must contain valid hex digits (0-9, a-f, A-F)
/// - Invalid hex codes will return None, resulting in uncolored text
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
    fn inverse(&self) -> String;
    fn strikethrough(&self) -> String;

    // Background colors
    fn on_red(&self) -> String;
    fn on_green(&self) -> String;
    fn on_yellow(&self) -> String;
    fn on_blue(&self) -> String;
    fn on_magenta(&self) -> String;
    fn on_cyan(&self) -> String;
    fn on_white(&self) -> String;
    fn on_black(&self) -> String;

    // RGB, HSL, and Hex color support
    /// Set text color using RGB values (0-255, compile-time enforced)
    fn rgb(&self, r: u8, g: u8, b: u8) -> String;
    /// Set background color using RGB values (0-255, compile-time enforced)
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> String;
    /// Set text color using HSL values (hue: 0-360, saturation: 0-100, lightness: 0-100)
    fn hsl(&self, h: f32, s: f32, l: f32) -> String;
    /// Set background color using HSL values (hue: 0-360, saturation: 0-100, lightness: 0-100)
    fn on_hsl(&self, h: f32, s: f32, l: f32) -> String;
    fn hex(&self, hex: &str) -> String;
    fn on_hex(&self, hex: &str) -> String;

    // Clear all formatting
    fn clear(&self) -> String;
}

impl<T: std::fmt::Display> Colorize for T {
    fn colorize(&self, color_code: &str) -> String {
        if !should_colorize() {
            return self.to_string();
        }
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

    fn inverse(&self) -> String {
        self.colorize("7")
    }

    fn strikethrough(&self) -> String {
        self.colorize("9")
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
        if !should_colorize() {
            return self.to_string();
        }
        format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn on_rgb(&self, r: u8, g: u8, b: u8) -> String {
        if !should_colorize() {
            return self.to_string();
        }
        format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, self)
    }

    fn hsl(&self, h: f32, s: f32, l: f32) -> String {
        if !should_colorize() {
            return self.to_string();
        }
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.rgb(r, g, b)
    }

    fn on_hsl(&self, h: f32, s: f32, l: f32) -> String {
        if !should_colorize() {
            return self.to_string();
        }
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.on_rgb(r, g, b)
    }

    fn hex(&self, hex: &str) -> String {
        if !should_colorize() {
            return self.to_string();
        }
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.rgb(r, g, b)
        } else {
            self.clear() // Return uncolored text if hex code is invalid
        }
    }

    fn on_hex(&self, hex: &str) -> String {
        if !should_colorize() {
            return self.to_string();
        }
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
    use rstest::rstest;
    fn setup() {
        std::env::remove_var("NO_COLOR");
    }

    fn teardown() {
        std::env::remove_var("NO_COLOR");
    }

    // Test data for basic colors
    #[rstest]
    #[case("red", "31")]
    #[case("green", "32")]
    #[case("yellow", "33")]
    #[case("blue", "34")]
    #[case("magenta", "35")]
    #[case("cyan", "36")]
    #[case("white", "37")]
    #[case("black", "30")]
    fn test_basic_colors(#[case] color: &str, #[case] code: &str) {
        setup();
        let text = "test";
        let expected = format!("\x1b[{}m{}\x1b[0m", code, text);
        match color {
            "red" => assert_eq!(text.red(), expected),
            "green" => assert_eq!(text.green(), expected),
            "yellow" => assert_eq!(text.yellow(), expected),
            "blue" => assert_eq!(text.blue(), expected),
            "magenta" => assert_eq!(text.magenta(), expected),
            "cyan" => assert_eq!(text.cyan(), expected),
            "white" => assert_eq!(text.white(), expected),
            "black" => assert_eq!(text.black(), expected),
            _ => unreachable!(),
        }
    }

    // Test data for bright colors
    #[rstest]
    #[case("bright_red", "91")]
    #[case("bright_green", "92")]
    #[case("bright_yellow", "93")]
    #[case("bright_blue", "94")]
    #[case("bright_magenta", "95")]
    #[case("bright_cyan", "96")]
    #[case("bright_white", "97")]
    fn test_bright_colors(#[case] color: &str, #[case] code: &str) {
        setup();
        let text = "test";
        let expected = format!("\x1b[{}m{}\x1b[0m", code, text);
        match color {
            "bright_red" => assert_eq!(text.bright_red(), expected),
            "bright_green" => assert_eq!(text.bright_green(), expected),
            "bright_yellow" => assert_eq!(text.bright_yellow(), expected),
            "bright_blue" => assert_eq!(text.bright_blue(), expected),
            "bright_magenta" => assert_eq!(text.bright_magenta(), expected),
            "bright_cyan" => assert_eq!(text.bright_cyan(), expected),
            "bright_white" => assert_eq!(text.bright_white(), expected),
            _ => unreachable!(),
        }
    }

    // Test data for background colors
    #[rstest]
    #[case("on_red", "41")]
    #[case("on_green", "42")]
    #[case("on_yellow", "43")]
    #[case("on_blue", "44")]
    #[case("on_magenta", "45")]
    #[case("on_cyan", "46")]
    #[case("on_white", "47")]
    #[case("on_black", "40")]
    fn test_background_colors(#[case] color: &str, #[case] code: &str) {
        setup();
        let text = "test";
        let expected = format!("\x1b[{}m{}\x1b[0m", code, text);
        match color {
            "on_red" => assert_eq!(text.on_red(), expected),
            "on_green" => assert_eq!(text.on_green(), expected),
            "on_yellow" => assert_eq!(text.on_yellow(), expected),
            "on_blue" => assert_eq!(text.on_blue(), expected),
            "on_magenta" => assert_eq!(text.on_magenta(), expected),
            "on_cyan" => assert_eq!(text.on_cyan(), expected),
            "on_white" => assert_eq!(text.on_white(), expected),
            "on_black" => assert_eq!(text.on_black(), expected),
            _ => unreachable!(),
        }
    }

    // Test data for styles
    #[rstest]
    #[case("bold", "1")]
    #[case("dim", "2")]
    #[case("italic", "3")]
    #[case("underline", "4")]
    #[case("inverse", "7")]
    #[case("strikethrough", "9")]
    fn test_styles(#[case] style: &str, #[case] code: &str) {
        setup();
        let text = "test";
        let expected = format!("\x1b[{}m{}\x1b[0m", code, text);
        match style {
            "bold" => assert_eq!(text.bold(), expected),
            "dim" => assert_eq!(text.dim(), expected),
            "italic" => assert_eq!(text.italic(), expected),
            "underline" => assert_eq!(text.underline(), expected),
            "inverse" => assert_eq!(text.inverse(), expected),
            "strikethrough" => assert_eq!(text.strikethrough(), expected),
            _ => unreachable!(),
        }
    }

    // Test RGB colors with various values
    #[rstest]
    #[case(255, 128, 0)]
    #[case(0, 255, 0)]
    #[case(128, 128, 128)]
    #[case(0, 0, 0)]
    #[case(255, 255, 255)]
    fn test_rgb_colors(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
        setup();
        let text = "test";
        assert_eq!(
            text.rgb(r, g, b),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_rgb(r, g, b),
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
    }

    // Test hex colors with various values
    #[rstest]
    #[case("#ff8000", 255, 128, 0)]
    #[case("#00ff00", 0, 255, 0)]
    #[case("#808080", 128, 128, 128)]
    #[case("#000000", 0, 0, 0)]
    #[case("#ffffff", 255, 255, 255)]
    fn test_hex_colors(#[case] hex: &str, #[case] r: u8, #[case] g: u8, #[case] b: u8) {
        setup();
        let text = "test";
        assert_eq!(
            text.hex(hex),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_hex(hex),
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );

        // Test without # prefix
        let hex_no_hash = hex.trim_start_matches('#');
        assert_eq!(
            text.hex(hex_no_hash),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_hex(hex_no_hash),
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
    }

    #[rstest]
    #[case("invalid")]
    #[case("#12")]
    #[case("not-a-color")]
    #[case("#12345")]
    #[case("#1234567")]
    #[case("#xyz")]
    fn test_invalid_hex(#[case] hex: &str) {
        setup();
        let text = "test";
        assert_eq!(text.hex(hex), "\x1b[0mtest\x1b[0m");
        assert_eq!(text.on_hex(hex), "\x1b[0mtest\x1b[0m");
    }

    #[test]
    fn test_string_and_str() {
        setup();
        let string = String::from("test");
        assert_eq!(string.red(), "test".red());
        assert_eq!(string.blue(), "test".blue());
    }

    #[test]
    fn test_format_macro() {
        setup();
        assert_eq!(format!("{}", "test".red()), format!("\x1b[31mtest\x1b[0m"));
    }

    #[test]
    fn test_chaining() {
        setup();
        assert_eq!("test".red().bold(), "\x1b[1m\x1b[31mtest\x1b[0m\x1b[0m");
        assert_eq!(
            "test".blue().italic().on_yellow(),
            "\x1b[43m\x1b[3m\x1b[34mtest\x1b[0m\x1b[0m\x1b[0m"
        );
    }

    #[test]
    fn test_hsl_colors() {
        setup();
        // Red (0° hue)
        assert_eq!("test".hsl(0.0, 100.0, 50.0), "test".rgb(255, 0, 0));

        // Green (120° hue)
        assert_eq!("test".hsl(120.0, 100.0, 50.0), "test".rgb(0, 255, 0));

        // Blue (240° hue)
        assert_eq!("test".hsl(240.0, 100.0, 50.0), "test".rgb(0, 0, 255));

        // Gray (any hue, 0% saturation)
        let gray = "test".hsl(0.0, 0.0, 50.0);
        // Due to floating point conversion, the value might be 127 or 128
        assert!(gray == "test".rgb(127, 127, 127) || gray == "test".rgb(128, 128, 128));

        // White (100% lightness)
        assert_eq!("test".hsl(0.0, 0.0, 100.0), "test".rgb(255, 255, 255));

        // Black (0% lightness)
        assert_eq!("test".hsl(0.0, 0.0, 0.0), "test".rgb(0, 0, 0));
    }

    #[test]
    fn test_hsl_background_colors() {
        setup();
        // Red background
        assert_eq!("test".on_hsl(0.0, 100.0, 50.0), "test".on_rgb(255, 0, 0));

        // Green background
        assert_eq!("test".on_hsl(120.0, 100.0, 50.0), "test".on_rgb(0, 255, 0));

        // Blue background
        assert_eq!("test".on_hsl(240.0, 100.0, 50.0), "test".on_rgb(0, 0, 255));
    }

    #[test]
    #[test]
    fn test_no_color_basic() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".red();
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_bright() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".bright_red();
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_background() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".on_red();
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_style() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".bold();
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_rgb() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".rgb(255, 128, 0);
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_hex() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".hex("#ff8000");
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_chaining() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let result = "test".red().bold();
        teardown();
        assert_eq!(result, "test");
    }

    #[test]
    fn test_no_color_string() {
        setup();
        std::env::set_var("NO_COLOR", "1");
        let string = String::from("test");
        let result = string.red();
        teardown();
        assert_eq!(result, "test");
    }
}
