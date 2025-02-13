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

use std::cell::RefCell;
use std::io::IsTerminal;

/// Configuration for controlling terminal detection behavior.
#[derive(Clone, Debug)]
pub struct ColorizeConfig {
    check_terminal: bool,
}

thread_local! {
    static CONFIG: RefCell<ColorizeConfig> = RefCell::new(ColorizeConfig::default());
}

impl Default for ColorizeConfig {
    fn default() -> Self {
        Self {
            check_terminal: true, // By default, we check the terminal
        }
    }
}

impl ColorizeConfig {
    /// Set whether to check if output is to a terminal.
    ///
    /// - If true (default), colors will be disabled when not outputting to a terminal
    /// - If false, terminal detection is skipped and colors are enabled (unless NO_COLOR is set)
    pub fn set_terminal_check(check: bool) {
        CONFIG.with(|c| c.borrow_mut().check_terminal = check);
    }

    /// Get the current configuration for this thread
    fn current() -> Self {
        CONFIG.with(|c| c.borrow().clone())
    }
}

/// Check if colors should be applied based on:
/// - NO_COLOR environment variable (returns false if set to any value)
/// - Whether stdout is connected to a terminal (if terminal checking is enabled)
///
/// Terminal checking can be disabled using `ColorizeConfig::set_terminal_check(false)`,
/// in which case colors will be enabled regardless of terminal status (unless NO_COLOR is set).
fn should_colorize() -> bool {
    // Always check NO_COLOR env var
    if std::env::var("NO_COLOR").is_ok() {
        return false;
    }

    // Only check terminal if configured to do so
    !ColorizeConfig::current().check_terminal || std::io::stdout().is_terminal()
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
    use rstest::*;

    /// Disables terminal checks for color support during testing.
    ///
    /// This function is used in tests to ensure that color codes are always
    /// generated, regardless of whether the output is going to a terminal or
    /// not. This allows us to verify the exact ANSI escape sequences that would
    /// be generated under normal circumstances. This is needed since 'nextest'
    /// at least seems to grab the output and so the terminal check would always
    /// return false.
    ///
    /// # Example
    /// ```
    /// #[test]
    /// fn test_colors() {
    ///     no_terminal_check();
    ///     assert_eq!("test".red(), "\x1b[31mtest\x1b[0m");
    /// }
    /// ```
    fn no_terminal_check() {
        ColorizeConfig::set_terminal_check(false);
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
        no_terminal_check();
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
        no_terminal_check();
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
        no_terminal_check();
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
        no_terminal_check();
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
        no_terminal_check();
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
        no_terminal_check();
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
        no_terminal_check();
        let text = "test";
        assert_eq!(text.hex(hex), "\x1b[0mtest\x1b[0m");
        assert_eq!(text.on_hex(hex), "\x1b[0mtest\x1b[0m");
    }

    #[test]
    fn test_string_and_str() {
        let string = String::from("test");
        assert_eq!(string.red(), "test".red());
        assert_eq!(string.blue(), "test".blue());
    }

    #[test]
    fn test_format_macro() {
        no_terminal_check();
        assert_eq!(format!("{}", "test".red()), format!("\x1b[31mtest\x1b[0m"));
    }

    #[test]
    fn test_chaining() {
        no_terminal_check();
        assert_eq!("test".red().bold(), "\x1b[1m\x1b[31mtest\x1b[0m\x1b[0m");
        assert_eq!(
            "test".blue().italic().on_yellow(),
            "\x1b[43m\x1b[3m\x1b[34mtest\x1b[0m\x1b[0m\x1b[0m"
        );
    }

    /// Helper function to check if two RGB values are equal within a tolerance of 1
    /// This accounts for floating-point rounding differences in HSL to RGB conversion
    fn assert_rgb_approx_eq(actual: &str, expected: &str) {
        no_terminal_check();
        let extract_rgb = |s: &str| {
            let parts: Vec<&str> = s.split(';').collect();
            if parts.len() >= 5 {
                let r = parts[2].parse::<i32>().unwrap();
                let g = parts[3].parse::<i32>().unwrap();
                let b = parts[4].split('m').next().unwrap().parse::<i32>().unwrap();
                (r, g, b)
            } else {
                panic!("Invalid ANSI color sequence");
            }
        };

        let (r1, g1, b1) = extract_rgb(actual);
        let (r2, g2, b2) = extract_rgb(expected);

        assert!(
            (r1 - r2).abs() <= 1 && (g1 - g2).abs() <= 1 && (b1 - b2).abs() <= 1,
            "RGB values differ by more than 1: ({}, {}, {}) vs ({}, {}, {})",
            r1,
            g1,
            b1,
            r2,
            g2,
            b2
        );
    }

    #[rstest]
    #[case(0.0, 100.0, 50.0, 255, 0, 0)] // Red (hue segment 0)
    #[case(60.0, 100.0, 50.0, 255, 255, 0)] // Yellow (boundary 0-1)
    #[case(90.0, 100.0, 50.0, 128, 255, 0)] // Chartreuse (hue segment 1)
    #[case(120.0, 100.0, 50.0, 0, 255, 0)] // Green (boundary 1-2)
    #[case(150.0, 100.0, 50.0, 0, 255, 128)] // Spring Green (hue segment 2)
    #[case(180.0, 100.0, 50.0, 0, 255, 255)] // Cyan (boundary 2-3)
    #[case(210.0, 100.0, 50.0, 0, 128, 255)] // Azure (hue segment 3)
    #[case(240.0, 100.0, 50.0, 0, 0, 255)] // Blue (boundary 3-4)
    #[case(300.0, 100.0, 50.0, 255, 0, 255)] // Magenta (boundary 4-5)
    #[case(330.0, 100.0, 50.0, 255, 0, 128)] // Rose (hue segment 5)
    #[case(360.0, 100.0, 50.0, 255, 0, 0)] // Red again (full circle)
    fn test_hsl_colors_comprehensive(
        #[case] h: f32,
        #[case] s: f32,
        #[case] l: f32,
        #[case] r: u8,
        #[case] g: u8,
        #[case] b: u8,
    ) {
        no_terminal_check();
        let actual = "test".hsl(h, s, l);
        let expected = "test".rgb(r, g, b);
        assert_rgb_approx_eq(&actual, &expected);
    }

    #[test]
    fn test_hsl_edge_cases() {
        // Helper closure for approximate RGB comparison
        let assert_hsl_rgb = |h, s, l, r, g, b| {
            let actual = "test".hsl(h, s, l);
            let expected = "test".rgb(r, g, b);
            assert_rgb_approx_eq(&actual, &expected);
        };
        no_terminal_check();

        // Gray scale (0% saturation)
        assert_hsl_rgb(0.0, 0.0, 0.0, 0, 0, 0); // Black
        assert_hsl_rgb(0.0, 0.0, 25.0, 64, 64, 64); // Dark gray
        assert_hsl_rgb(0.0, 0.0, 50.0, 128, 128, 128); // Mid gray
        assert_hsl_rgb(0.0, 0.0, 75.0, 191, 191, 191); // Light gray
        assert_hsl_rgb(0.0, 0.0, 100.0, 255, 255, 255); // White

        // Saturation variations (red hue)
        assert_hsl_rgb(0.0, 25.0, 50.0, 159, 96, 96); // Low saturation
        assert_hsl_rgb(0.0, 50.0, 50.0, 191, 64, 64); // Medium saturation
        assert_hsl_rgb(0.0, 75.0, 50.0, 223, 32, 32); // High saturation

        // Lightness variations with full saturation
        assert_hsl_rgb(120.0, 100.0, 25.0, 0, 128, 0); // Dark green
        assert_hsl_rgb(120.0, 100.0, 75.0, 128, 255, 128); // Light green
    }

    #[test]
    fn test_hsl_background_colors() {
        no_terminal_check();
        // Red background
        let actual = "test".on_hsl(0.0, 100.0, 50.0);
        let expected = "test".on_rgb(255, 0, 0);
        assert_rgb_approx_eq(&actual, &expected);

        // Green background
        let actual = "test".on_hsl(120.0, 100.0, 50.0);
        let expected = "test".on_rgb(0, 255, 0);
        assert_rgb_approx_eq(&actual, &expected);

        // Blue background
        let actual = "test".on_hsl(240.0, 100.0, 50.0);
        let expected = "test".on_rgb(0, 0, 255);
        assert_rgb_approx_eq(&actual, &expected);
    }

    #[test]
    fn test_no_color_and_terminal_detection() {
        // Test NO_COLOR environment variable we also disable the terminal check
        // here so we are sure the NO_COLOR variable is the only thing that
        // disables color output.
        no_terminal_check();
        std::env::set_var("NO_COLOR", "1");

        // Test basic colors
        assert_eq!("test".red(), "test");
        assert_eq!("test".blue(), "test");

        // Test bright colors
        assert_eq!("test".bright_red(), "test");
        assert_eq!("test".bright_blue(), "test");

        // Test background colors
        assert_eq!("test".on_red(), "test");
        assert_eq!("test".on_blue(), "test");

        // Test styles
        assert_eq!("test".bold(), "test");
        assert_eq!("test".italic(), "test");

        // Test RGB colors
        assert_eq!("test".rgb(255, 128, 0), "test");
        assert_eq!("test".on_rgb(255, 128, 0), "test");

        // Test hex colors
        assert_eq!("test".hex("#ff8000"), "test");
        assert_eq!("test".on_hex("#ff8000"), "test");

        // Test HSL colors
        assert_eq!("test".hsl(0.0, 100.0, 50.0), "test");
        assert_eq!("test".on_hsl(0.0, 100.0, 50.0), "test");

        // Test chaining
        assert_eq!("test".red().bold(), "test");
        assert_eq!("test".blue().italic().on_yellow(), "test");

        // Test with String
        let string = String::from("test");
        assert_eq!(string.red(), "test");
        assert_eq!(string.blue(), "test");

        // Clean up
        std::env::remove_var("NO_COLOR");

        // Note: We can't easily test the terminal detection in unit tests
        // since std::io::stdout().is_terminal() depends on the actual
        // terminal state. The behavior has been manually verified:
        // - Returns true when running normally in a terminal
        // - Returns false when output is piped (e.g., `cargo test | cat`)
        // - Returns false when output is redirected (e.g., `cargo test > output.txt`)
    }

    #[test]
    #[should_panic(expected = "Invalid ANSI color sequence")]
    fn test_assert_rgb_approx_eq_invalid_sequence() {
        assert_rgb_approx_eq("invalid", "also invalid");
    }

    #[test]
    #[should_panic(expected = "RGB values differ by more than 1: (255, 0, 0) vs (252, 0, 0)")]
    fn test_assert_rgb_approx_eq_large_diff() {
        no_terminal_check();
        let color1 = "test".rgb(255, 0, 0);
        let color2 = "test".rgb(252, 0, 0);
        assert_rgb_approx_eq(&color1, &color2);
    }
}
