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
//! - Hex color codes can be provided with or without the `#` prefix
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
//!
//! // Invalid hex codes return plain text
//! println!("{}", "Invalid hex".hex("xyz")); // Returns plain text
//! println!("{}", "Too short".hex("#f8")); // Returns plain text
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
//! # Note
//!
//! Colors and styles are implemented using ANSI escape codes, which are
//! supported by most modern terminals. If your terminal does not support ANSI
//! escape codes, or if color output is disabled by policy, the text is
//! displayed without styling.

use std::cell::RefCell;
use std::fmt::{self, Display};
use std::io::IsTerminal;

/// Runtime color policy for rendered output.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorMode {
    /// Enable styling only when stdout is a terminal.
    #[default]
    Auto,
    /// Always emit styling, even when stdout is not a terminal.
    Always,
    /// Never emit styling.
    Never,
}

/// Configuration for controlling runtime color behavior.
#[derive(Clone, Debug)]
pub struct ColorizeConfig {
    color_mode: ColorMode,
}

thread_local! {
    static CONFIG: RefCell<ColorizeConfig> = RefCell::new(ColorizeConfig::default());
}

impl Default for ColorizeConfig {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::Auto,
        }
    }
}

impl ColorizeConfig {
    /// Set the runtime color policy for the current thread.
    pub fn set_color_mode(mode: ColorMode) {
        CONFIG.with(|config| config.borrow_mut().color_mode = mode);
    }

    /// Get the runtime color policy for the current thread.
    pub fn color_mode() -> ColorMode {
        CONFIG.with(|config| config.borrow().color_mode)
    }

    /// Compatibility shim for the previous API.
    #[deprecated(note = "use ColorizeConfig::set_color_mode(ColorMode) instead")]
    pub fn set_terminal_check(check: bool) {
        let mode = if check {
            ColorMode::Auto
        } else {
            ColorMode::Always
        };
        Self::set_color_mode(mode);
    }
}

fn should_colorize() -> bool {
    match ColorizeConfig::color_mode() {
        ColorMode::Never => false,
        ColorMode::Always => std::env::var_os("NO_COLOR").is_none(),
        ColorMode::Auto => {
            std::env::var_os("NO_COLOR").is_none() && std::io::stdout().is_terminal()
        }
    }
}

/// Convert HSL color values to RGB.
///
/// - `h`: Hue in degrees
/// - `s`: Saturation percentage
/// - `l`: Lightness percentage
fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
    let h = h / 360.0;
    let s = s / 100.0;
    let l = l / 100.0;

    let c = (1.0 - (2.0 * l - 1.0).abs()) * s;
    let x = c * (1.0 - ((h * 6.0) % 2.0 - 1.0).abs());
    let m = l - c / 2.0;

    let (r, g, b) = match (h * 6.0) as i32 {
        0 => (c, x, 0.0),
        1 => (x, c, 0.0),
        2 => (0.0, c, x),
        3 => (0.0, x, c),
        4 => (x, 0.0, c),
        _ => (c, 0.0, x),
    };

    (
        ((r + m) * 255.0) as u8,
        ((g + m) * 255.0) as u8,
        ((b + m) * 255.0) as u8,
    )
}

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum NamedColor {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl NamedColor {
    fn foreground_code(self) -> &'static str {
        match self {
            Self::Black => "30",
            Self::Red => "31",
            Self::Green => "32",
            Self::Yellow => "33",
            Self::Blue => "34",
            Self::Magenta => "35",
            Self::Cyan => "36",
            Self::White => "37",
            Self::BrightRed => "91",
            Self::BrightGreen => "92",
            Self::BrightYellow => "93",
            Self::BrightBlue => "94",
            Self::BrightMagenta => "95",
            Self::BrightCyan => "96",
            Self::BrightWhite => "97",
        }
    }

    fn background_code(self) -> &'static str {
        match self {
            Self::Black => "40",
            Self::Red => "41",
            Self::Green => "42",
            Self::Yellow => "43",
            Self::Blue => "44",
            Self::Magenta => "45",
            Self::Cyan => "46",
            Self::White => "47",
            Self::BrightRed => "101",
            Self::BrightGreen => "102",
            Self::BrightYellow => "103",
            Self::BrightBlue => "104",
            Self::BrightMagenta => "105",
            Self::BrightCyan => "106",
            Self::BrightWhite => "107",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum ColorSpec {
    Named(NamedColor),
    Rgb(u8, u8, u8),
}

impl ColorSpec {
    fn foreground_code(&self) -> String {
        match self {
            Self::Named(color) => color.foreground_code().to_string(),
            Self::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    fn background_code(&self) -> String {
        match self {
            Self::Named(color) => color.background_code().to_string(),
            Self::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct StyleFlags {
    bold: bool,
    dim: bool,
    italic: bool,
    underline: bool,
    inverse: bool,
    strikethrough: bool,
}

impl StyleFlags {
    fn sgr_codes(&self) -> Vec<String> {
        let mut codes = Vec::new();
        if self.bold {
            codes.push("1".to_string());
        }
        if self.dim {
            codes.push("2".to_string());
        }
        if self.italic {
            codes.push("3".to_string());
        }
        if self.underline {
            codes.push("4".to_string());
        }
        if self.inverse {
            codes.push("7".to_string());
        }
        if self.strikethrough {
            codes.push("9".to_string());
        }
        codes
    }
}

/// A styled text value that composes colors and text attributes before render.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StyledText {
    text: String,
    foreground: Option<ColorSpec>,
    background: Option<ColorSpec>,
    styles: StyleFlags,
    raw_codes: Vec<String>,
}

impl StyledText {
    /// Create a plain styled value from text.
    pub fn plain(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            foreground: None,
            background: None,
            styles: StyleFlags::default(),
            raw_codes: Vec::new(),
        }
    }

    /// Return the plain, unstyled text payload.
    pub fn plain_text(&self) -> &str {
        &self.text
    }

    fn with_foreground(mut self, color: ColorSpec) -> Self {
        self.foreground = Some(color);
        self
    }

    fn with_background(mut self, color: ColorSpec) -> Self {
        self.background = Some(color);
        self
    }

    fn set_style(mut self, update: impl FnOnce(&mut StyleFlags)) -> Self {
        update(&mut self.styles);
        self
    }

    fn active_codes(&self) -> Vec<String> {
        let mut codes = self.raw_codes.clone();
        codes.extend(self.styles.sgr_codes());

        if let Some(foreground) = &self.foreground {
            codes.push(foreground.foreground_code());
        }

        if let Some(background) = &self.background {
            codes.push(background.background_code());
        }

        codes
    }

    /// Apply a raw ANSI SGR code sequence to the value.
    pub fn colorize(mut self, color_code: &str) -> Self {
        if !color_code.trim().is_empty() {
            self.raw_codes.push(color_code.to_string());
        }
        self
    }

    pub fn red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Red))
    }

    pub fn green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Green))
    }

    pub fn yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Yellow))
    }

    pub fn blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Blue))
    }

    pub fn magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Magenta))
    }

    pub fn cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Cyan))
    }

    pub fn white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::White))
    }

    pub fn black(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Black))
    }

    pub fn bright_red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightRed))
    }

    pub fn bright_green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightGreen))
    }

    pub fn bright_yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightYellow))
    }

    pub fn bright_blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightBlue))
    }

    pub fn bright_magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightMagenta))
    }

    pub fn bright_cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightCyan))
    }

    pub fn bright_white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightWhite))
    }

    pub fn bold(self) -> Self {
        self.set_style(|styles| styles.bold = true)
    }

    pub fn dim(self) -> Self {
        self.set_style(|styles| styles.dim = true)
    }

    pub fn italic(self) -> Self {
        self.set_style(|styles| styles.italic = true)
    }

    pub fn underline(self) -> Self {
        self.set_style(|styles| styles.underline = true)
    }

    pub fn inverse(self) -> Self {
        self.set_style(|styles| styles.inverse = true)
    }

    pub fn strikethrough(self) -> Self {
        self.set_style(|styles| styles.strikethrough = true)
    }

    pub fn on_red(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Red))
    }

    pub fn on_green(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Green))
    }

    pub fn on_yellow(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Yellow))
    }

    pub fn on_blue(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Blue))
    }

    pub fn on_magenta(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Magenta))
    }

    pub fn on_cyan(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Cyan))
    }

    pub fn on_white(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::White))
    }

    pub fn on_black(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Black))
    }

    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_foreground(ColorSpec::Rgb(r, g, b))
    }

    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_background(ColorSpec::Rgb(r, g, b))
    }

    pub fn hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.rgb(r, g, b)
    }

    pub fn on_hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.on_rgb(r, g, b)
    }

    pub fn hex(self, hex: &str) -> Self {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.rgb(r, g, b)
        } else {
            self.clear()
        }
    }

    pub fn on_hex(self, hex: &str) -> Self {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.on_rgb(r, g, b)
        } else {
            self.clear()
        }
    }

    /// Remove all applied styling and return plain text.
    pub fn clear(mut self) -> Self {
        self.foreground = None;
        self.background = None;
        self.styles = StyleFlags::default();
        self.raw_codes.clear();
        self
    }
}

impl Display for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let codes = self.active_codes();
        if !should_colorize() || codes.is_empty() {
            return f.write_str(&self.text);
        }

        write!(f, "\x1b[{}m{}\x1b[0m", codes.join(";"), self.text)
    }
}

impl From<StyledText> for String {
    fn from(value: StyledText) -> Self {
        value.to_string()
    }
}

/// Trait for turning values into styled terminal text.
pub trait Colorize {
    /// Apply a raw ANSI SGR code sequence.
    fn colorize(&self, color_code: &str) -> StyledText;

    fn red(&self) -> StyledText;
    fn green(&self) -> StyledText;
    fn yellow(&self) -> StyledText;
    fn blue(&self) -> StyledText;
    fn magenta(&self) -> StyledText;
    fn cyan(&self) -> StyledText;
    fn white(&self) -> StyledText;
    fn black(&self) -> StyledText;

    fn bright_red(&self) -> StyledText;
    fn bright_green(&self) -> StyledText;
    fn bright_yellow(&self) -> StyledText;
    fn bright_blue(&self) -> StyledText;
    fn bright_magenta(&self) -> StyledText;
    fn bright_cyan(&self) -> StyledText;
    fn bright_white(&self) -> StyledText;

    fn bold(&self) -> StyledText;
    fn dim(&self) -> StyledText;
    fn italic(&self) -> StyledText;
    fn underline(&self) -> StyledText;
    fn inverse(&self) -> StyledText;
    fn strikethrough(&self) -> StyledText;

    fn on_red(&self) -> StyledText;
    fn on_green(&self) -> StyledText;
    fn on_yellow(&self) -> StyledText;
    fn on_blue(&self) -> StyledText;
    fn on_magenta(&self) -> StyledText;
    fn on_cyan(&self) -> StyledText;
    fn on_white(&self) -> StyledText;
    fn on_black(&self) -> StyledText;

    fn rgb(&self, r: u8, g: u8, b: u8) -> StyledText;
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> StyledText;
    fn hsl(&self, h: f32, s: f32, l: f32) -> StyledText;
    fn on_hsl(&self, h: f32, s: f32, l: f32) -> StyledText;
    fn hex(&self, hex: &str) -> StyledText;
    fn on_hex(&self, hex: &str) -> StyledText;
    fn clear(&self) -> StyledText;
}

impl<T: Display> Colorize for T {
    fn colorize(&self, color_code: &str) -> StyledText {
        StyledText::plain(self.to_string()).colorize(color_code)
    }

    fn red(&self) -> StyledText {
        StyledText::plain(self.to_string()).red()
    }

    fn green(&self) -> StyledText {
        StyledText::plain(self.to_string()).green()
    }

    fn yellow(&self) -> StyledText {
        StyledText::plain(self.to_string()).yellow()
    }

    fn blue(&self) -> StyledText {
        StyledText::plain(self.to_string()).blue()
    }

    fn magenta(&self) -> StyledText {
        StyledText::plain(self.to_string()).magenta()
    }

    fn cyan(&self) -> StyledText {
        StyledText::plain(self.to_string()).cyan()
    }

    fn white(&self) -> StyledText {
        StyledText::plain(self.to_string()).white()
    }

    fn black(&self) -> StyledText {
        StyledText::plain(self.to_string()).black()
    }

    fn bright_red(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_red()
    }

    fn bright_green(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_green()
    }

    fn bright_yellow(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_yellow()
    }

    fn bright_blue(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_blue()
    }

    fn bright_magenta(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_magenta()
    }

    fn bright_cyan(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_cyan()
    }

    fn bright_white(&self) -> StyledText {
        StyledText::plain(self.to_string()).bright_white()
    }

    fn bold(&self) -> StyledText {
        StyledText::plain(self.to_string()).bold()
    }

    fn dim(&self) -> StyledText {
        StyledText::plain(self.to_string()).dim()
    }

    fn italic(&self) -> StyledText {
        StyledText::plain(self.to_string()).italic()
    }

    fn underline(&self) -> StyledText {
        StyledText::plain(self.to_string()).underline()
    }

    fn inverse(&self) -> StyledText {
        StyledText::plain(self.to_string()).inverse()
    }

    fn strikethrough(&self) -> StyledText {
        StyledText::plain(self.to_string()).strikethrough()
    }

    fn on_red(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_red()
    }

    fn on_green(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_green()
    }

    fn on_yellow(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_yellow()
    }

    fn on_blue(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_blue()
    }

    fn on_magenta(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_magenta()
    }

    fn on_cyan(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_cyan()
    }

    fn on_white(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_white()
    }

    fn on_black(&self) -> StyledText {
        StyledText::plain(self.to_string()).on_black()
    }

    fn rgb(&self, r: u8, g: u8, b: u8) -> StyledText {
        StyledText::plain(self.to_string()).rgb(r, g, b)
    }

    fn on_rgb(&self, r: u8, g: u8, b: u8) -> StyledText {
        StyledText::plain(self.to_string()).on_rgb(r, g, b)
    }

    fn hsl(&self, h: f32, s: f32, l: f32) -> StyledText {
        StyledText::plain(self.to_string()).hsl(h, s, l)
    }

    fn on_hsl(&self, h: f32, s: f32, l: f32) -> StyledText {
        StyledText::plain(self.to_string()).on_hsl(h, s, l)
    }

    fn hex(&self, hex: &str) -> StyledText {
        StyledText::plain(self.to_string()).hex(hex)
    }

    fn on_hex(&self, hex: &str) -> StyledText {
        StyledText::plain(self.to_string()).on_hex(hex)
    }

    fn clear(&self) -> StyledText {
        StyledText::plain(self.to_string()).clear()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;
    use std::env;
    use std::ffi::OsString;
    use std::sync::{LazyLock, Mutex, MutexGuard};

    static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    struct TestStateGuard {
        _lock: MutexGuard<'static, ()>,
        previous_mode: ColorMode,
        previous_no_color: Option<OsString>,
    }

    impl TestStateGuard {
        fn colors_enabled(mode: ColorMode) -> Self {
            let guard = TEST_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let previous_mode = ColorizeConfig::color_mode();
            let previous_no_color = env::var_os("NO_COLOR");

            env::remove_var("NO_COLOR");
            ColorizeConfig::set_color_mode(mode);

            Self {
                _lock: guard,
                previous_mode,
                previous_no_color,
            }
        }

        fn no_color(mode: ColorMode) -> Self {
            let guard = TEST_LOCK
                .lock()
                .unwrap_or_else(|poisoned| poisoned.into_inner());
            let previous_mode = ColorizeConfig::color_mode();
            let previous_no_color = env::var_os("NO_COLOR");

            env::set_var("NO_COLOR", "1");
            ColorizeConfig::set_color_mode(mode);

            Self {
                _lock: guard,
                previous_mode,
                previous_no_color,
            }
        }
    }

    impl Drop for TestStateGuard {
        fn drop(&mut self) {
            ColorizeConfig::set_color_mode(self.previous_mode);
            match self.previous_no_color.as_ref() {
                Some(value) => env::set_var("NO_COLOR", value),
                None => env::remove_var("NO_COLOR"),
            }
        }
    }

    #[rstest]
    #[case("red", "\x1b[31mtest\x1b[0m")]
    #[case("green", "\x1b[32mtest\x1b[0m")]
    #[case("yellow", "\x1b[33mtest\x1b[0m")]
    #[case("blue", "\x1b[34mtest\x1b[0m")]
    #[case("magenta", "\x1b[35mtest\x1b[0m")]
    #[case("cyan", "\x1b[36mtest\x1b[0m")]
    #[case("white", "\x1b[37mtest\x1b[0m")]
    #[case("black", "\x1b[30mtest\x1b[0m")]
    fn test_basic_colors(#[case] color: &str, #[case] expected: &str) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        let actual = match color {
            "red" => text.red().to_string(),
            "green" => text.green().to_string(),
            "yellow" => text.yellow().to_string(),
            "blue" => text.blue().to_string(),
            "magenta" => text.magenta().to_string(),
            "cyan" => text.cyan().to_string(),
            "white" => text.white().to_string(),
            "black" => text.black().to_string(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("bright_red", "\x1b[91mtest\x1b[0m")]
    #[case("bright_green", "\x1b[92mtest\x1b[0m")]
    #[case("bright_yellow", "\x1b[93mtest\x1b[0m")]
    #[case("bright_blue", "\x1b[94mtest\x1b[0m")]
    #[case("bright_magenta", "\x1b[95mtest\x1b[0m")]
    #[case("bright_cyan", "\x1b[96mtest\x1b[0m")]
    #[case("bright_white", "\x1b[97mtest\x1b[0m")]
    fn test_bright_colors(#[case] color: &str, #[case] expected: &str) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        let actual = match color {
            "bright_red" => text.bright_red().to_string(),
            "bright_green" => text.bright_green().to_string(),
            "bright_yellow" => text.bright_yellow().to_string(),
            "bright_blue" => text.bright_blue().to_string(),
            "bright_magenta" => text.bright_magenta().to_string(),
            "bright_cyan" => text.bright_cyan().to_string(),
            "bright_white" => text.bright_white().to_string(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("on_red", "\x1b[41mtest\x1b[0m")]
    #[case("on_green", "\x1b[42mtest\x1b[0m")]
    #[case("on_yellow", "\x1b[43mtest\x1b[0m")]
    #[case("on_blue", "\x1b[44mtest\x1b[0m")]
    #[case("on_magenta", "\x1b[45mtest\x1b[0m")]
    #[case("on_cyan", "\x1b[46mtest\x1b[0m")]
    #[case("on_white", "\x1b[47mtest\x1b[0m")]
    #[case("on_black", "\x1b[40mtest\x1b[0m")]
    fn test_background_colors(#[case] color: &str, #[case] expected: &str) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        let actual = match color {
            "on_red" => text.on_red().to_string(),
            "on_green" => text.on_green().to_string(),
            "on_yellow" => text.on_yellow().to_string(),
            "on_blue" => text.on_blue().to_string(),
            "on_magenta" => text.on_magenta().to_string(),
            "on_cyan" => text.on_cyan().to_string(),
            "on_white" => text.on_white().to_string(),
            "on_black" => text.on_black().to_string(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case("bold", "\x1b[1mtest\x1b[0m")]
    #[case("dim", "\x1b[2mtest\x1b[0m")]
    #[case("italic", "\x1b[3mtest\x1b[0m")]
    #[case("underline", "\x1b[4mtest\x1b[0m")]
    #[case("inverse", "\x1b[7mtest\x1b[0m")]
    #[case("strikethrough", "\x1b[9mtest\x1b[0m")]
    fn test_styles(#[case] style: &str, #[case] expected: &str) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        let actual = match style {
            "bold" => text.bold().to_string(),
            "dim" => text.dim().to_string(),
            "italic" => text.italic().to_string(),
            "underline" => text.underline().to_string(),
            "inverse" => text.inverse().to_string(),
            "strikethrough" => text.strikethrough().to_string(),
            _ => unreachable!(),
        };
        assert_eq!(actual, expected);
    }

    #[rstest]
    #[case(255, 128, 0)]
    #[case(0, 255, 0)]
    #[case(128, 128, 128)]
    #[case(0, 0, 0)]
    #[case(255, 255, 255)]
    fn test_rgb_colors(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        assert_eq!(
            text.rgb(r, g, b).to_string(),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_rgb(r, g, b).to_string(),
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
    }

    #[rstest]
    #[case("#ff8000", 255, 128, 0)]
    #[case("#00ff00", 0, 255, 0)]
    #[case("#808080", 128, 128, 128)]
    #[case("#000000", 0, 0, 0)]
    #[case("#ffffff", 255, 255, 255)]
    fn test_hex_colors(#[case] hex: &str, #[case] r: u8, #[case] g: u8, #[case] b: u8) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        assert_eq!(
            text.hex(hex).to_string(),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_hex(hex).to_string(),
            format!("\x1b[48;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );

        let hex_without_prefix = hex.trim_start_matches('#');
        assert_eq!(
            text.hex(hex_without_prefix).to_string(),
            format!("\x1b[38;2;{};{};{}m{}\x1b[0m", r, g, b, text)
        );
        assert_eq!(
            text.on_hex(hex_without_prefix).to_string(),
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
    fn test_invalid_hex_returns_plain_text(#[case] hex: &str) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let text = "test";
        assert_eq!(text.hex(hex).to_string(), "test");
        assert_eq!(text.on_hex(hex).to_string(), "test");
        assert_eq!(text.red().hex(hex).to_string(), "test");
        assert_eq!(text.on_blue().on_hex(hex).to_string(), "test");
    }

    #[test]
    fn test_clear_returns_plain_text() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".clear().to_string(), "test");
        assert_eq!("test".red().clear().to_string(), "test");
        assert_eq!(
            "test".blue().italic().on_yellow().clear().to_string(),
            "test"
        );
    }

    #[test]
    fn test_chaining_composes_once() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".red().bold().to_string(), "\x1b[1;31mtest\x1b[0m");
        assert_eq!(
            "test".blue().italic().on_yellow().to_string(),
            "\x1b[3;34;43mtest\x1b[0m"
        );
        assert_eq!(
            "test".rgb(255, 128, 0).on_blue().to_string(),
            "\x1b[38;2;255;128;0;44mtest\x1b[0m"
        );
    }

    #[test]
    fn test_conflicting_chains_use_last_color() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".red().green().to_string(), "\x1b[32mtest\x1b[0m");
        assert_eq!("test".on_red().on_blue().to_string(), "\x1b[44mtest\x1b[0m");
    }

    #[test]
    fn test_style_flags_accumulate() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".bold().dim().to_string(), "\x1b[1;2mtest\x1b[0m");
        assert_eq!(
            "test".underline().italic().strikethrough().to_string(),
            "\x1b[3;4;9mtest\x1b[0m"
        );
    }

    #[test]
    fn test_string_and_plain_text_access() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let string = String::from("test");
        let styled = string.red().bold();
        assert_eq!(styled.to_string(), "\x1b[1;31mtest\x1b[0m");
        assert_eq!(styled.plain_text(), "test");
    }

    #[test]
    fn test_format_macro_uses_display() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!(format!("{}", "test".red()), "\x1b[31mtest\x1b[0m");
    }

    fn assert_rgb_approx_eq(actual: &str, expected: &str) {
        let extract_rgb = |s: &str| {
            let start = s.find("38;2;").or_else(|| s.find("48;2;"));
            if let Some(start) = start {
                let sequence = &s[start..];
                let parts: Vec<&str> = sequence.split(';').collect();
                let r = parts.get(2).and_then(|part| part.parse::<i32>().ok());
                let g = parts.get(3).and_then(|part| part.parse::<i32>().ok());
                let b = parts
                    .get(4)
                    .and_then(|part| part.split('m').next())
                    .and_then(|part| part.parse::<i32>().ok());

                if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                    return (r, g, b);
                }
            }

            panic!("Invalid ANSI color sequence");
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
    #[case(0.0, 100.0, 50.0, 255, 0, 0)]
    #[case(60.0, 100.0, 50.0, 255, 255, 0)]
    #[case(90.0, 100.0, 50.0, 128, 255, 0)]
    #[case(120.0, 100.0, 50.0, 0, 255, 0)]
    #[case(150.0, 100.0, 50.0, 0, 255, 128)]
    #[case(180.0, 100.0, 50.0, 0, 255, 255)]
    #[case(210.0, 100.0, 50.0, 0, 128, 255)]
    #[case(240.0, 100.0, 50.0, 0, 0, 255)]
    #[case(300.0, 100.0, 50.0, 255, 0, 255)]
    #[case(330.0, 100.0, 50.0, 255, 0, 128)]
    #[case(360.0, 100.0, 50.0, 255, 0, 0)]
    fn test_hsl_colors_comprehensive(
        #[case] h: f32,
        #[case] s: f32,
        #[case] l: f32,
        #[case] r: u8,
        #[case] g: u8,
        #[case] b: u8,
    ) {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let actual = "test".hsl(h, s, l).to_string();
        let expected = "test".rgb(r, g, b).to_string();
        assert_rgb_approx_eq(&actual, &expected);
    }

    #[test]
    fn test_hsl_edge_cases() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);

        let assert_hsl_rgb = |h, s, l, r, g, b| {
            let actual = "test".hsl(h, s, l).to_string();
            let expected = "test".rgb(r, g, b).to_string();
            assert_rgb_approx_eq(&actual, &expected);
        };

        assert_hsl_rgb(0.0, 0.0, 0.0, 0, 0, 0);
        assert_hsl_rgb(0.0, 0.0, 25.0, 64, 64, 64);
        assert_hsl_rgb(0.0, 0.0, 50.0, 128, 128, 128);
        assert_hsl_rgb(0.0, 0.0, 75.0, 191, 191, 191);
        assert_hsl_rgb(0.0, 0.0, 100.0, 255, 255, 255);

        assert_hsl_rgb(0.0, 25.0, 50.0, 159, 96, 96);
        assert_hsl_rgb(0.0, 50.0, 50.0, 191, 64, 64);
        assert_hsl_rgb(0.0, 75.0, 50.0, 223, 32, 32);

        assert_hsl_rgb(120.0, 100.0, 25.0, 0, 128, 0);
        assert_hsl_rgb(120.0, 100.0, 75.0, 128, 255, 128);
    }

    #[test]
    fn test_hsl_background_colors() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let actual = "test".on_hsl(0.0, 100.0, 50.0).to_string();
        let expected = "test".on_rgb(255, 0, 0).to_string();
        assert_rgb_approx_eq(&actual, &expected);

        let actual = "test".on_hsl(120.0, 100.0, 50.0).to_string();
        let expected = "test".on_rgb(0, 255, 0).to_string();
        assert_rgb_approx_eq(&actual, &expected);

        let actual = "test".on_hsl(240.0, 100.0, 50.0).to_string();
        let expected = "test".on_rgb(0, 0, 255).to_string();
        assert_rgb_approx_eq(&actual, &expected);
    }

    #[test]
    fn test_color_mode_always_forces_color() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".red().to_string(), "\x1b[31mtest\x1b[0m");
    }

    #[test]
    fn test_color_mode_auto_respects_tty_detection() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Auto);
        assert_eq!("test".red().to_string(), "test");
    }

    #[test]
    fn test_color_mode_never_disables_color() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
        assert_eq!("test".red().to_string(), "test");
        assert_eq!("test".blue().italic().on_yellow().to_string(), "test");
    }

    #[test]
    fn test_no_color_disables_output_in_auto_and_always() {
        let _guard = TestStateGuard::no_color(ColorMode::Always);
        assert_eq!("test".red().to_string(), "test");
        assert_eq!("test".blue().italic().on_yellow().to_string(), "test");
    }

    #[test]
    #[allow(deprecated)]
    fn test_set_terminal_check_compatibility_mapping() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
        ColorizeConfig::set_terminal_check(false);
        assert_eq!(ColorizeConfig::color_mode(), ColorMode::Always);

        ColorizeConfig::set_terminal_check(true);
        assert_eq!(ColorizeConfig::color_mode(), ColorMode::Auto);
    }

    #[test]
    fn test_raw_colorize_codes_still_render() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        assert_eq!("test".colorize("31;1").to_string(), "\x1b[31;1mtest\x1b[0m");
        assert_eq!(
            "test".colorize("31").green().to_string(),
            "\x1b[31;32mtest\x1b[0m"
        );
    }

    #[test]
    fn test_from_styled_text_to_string() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let rendered: String = "test".red().bold().into();
        assert_eq!(rendered, "\x1b[1;31mtest\x1b[0m");
    }

    #[test]
    #[should_panic(expected = "Invalid ANSI color sequence")]
    fn test_assert_rgb_approx_eq_invalid_sequence() {
        assert_rgb_approx_eq("invalid", "also invalid");
    }

    #[test]
    #[should_panic(expected = "RGB values differ by more than 1: (255, 0, 0) vs (252, 0, 0)")]
    fn test_assert_rgb_approx_eq_large_diff() {
        let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
        let color1 = "test".rgb(255, 0, 0).to_string();
        let color2 = "test".rgb(252, 0, 0).to_string();
        assert_rgb_approx_eq(&color1, &color2);
    }
}
