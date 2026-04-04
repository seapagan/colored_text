use std::fmt::{self, Display};

use crate::color::{hex_to_rgb, hsl_to_rgb, ColorSpec, NamedColor};
use crate::config::should_colorize;

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
///
/// `StyledText` is an immutable builder-style value. Each styling method returns
/// a new value with the additional color or style applied.
#[must_use = "StyledText must be rendered, converted, or otherwise used"]
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
    #[must_use]
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
    #[must_use]
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
    ///
    /// This is an escape hatch for manual SGR composition. Prefer the typed
    /// color and style methods when possible.
    #[must_use]
    pub fn colorize(mut self, color_code: &str) -> Self {
        if !color_code.trim().is_empty() {
            self.raw_codes.push(color_code.to_string());
        }
        self
    }

    /// Apply the standard red foreground color.
    #[must_use]
    pub fn red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Red))
    }

    /// Apply the standard green foreground color.
    #[must_use]
    pub fn green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Green))
    }

    /// Apply the standard yellow foreground color.
    #[must_use]
    pub fn yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Yellow))
    }

    /// Apply the standard blue foreground color.
    #[must_use]
    pub fn blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Blue))
    }

    /// Apply the standard magenta foreground color.
    #[must_use]
    pub fn magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Magenta))
    }

    /// Apply the standard cyan foreground color.
    #[must_use]
    pub fn cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Cyan))
    }

    /// Apply the standard white foreground color.
    #[must_use]
    pub fn white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::White))
    }

    /// Apply the standard black foreground color.
    #[must_use]
    pub fn black(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Black))
    }

    /// Apply the bright red foreground color.
    #[must_use]
    pub fn bright_red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightRed))
    }

    /// Apply the bright green foreground color.
    #[must_use]
    pub fn bright_green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightGreen))
    }

    /// Apply the bright yellow foreground color.
    #[must_use]
    pub fn bright_yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightYellow))
    }

    /// Apply the bright blue foreground color.
    #[must_use]
    pub fn bright_blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightBlue))
    }

    /// Apply the bright magenta foreground color.
    #[must_use]
    pub fn bright_magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightMagenta))
    }

    /// Apply the bright cyan foreground color.
    #[must_use]
    pub fn bright_cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightCyan))
    }

    /// Apply the bright white foreground color.
    #[must_use]
    pub fn bright_white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightWhite))
    }

    /// Add bold text styling.
    #[must_use]
    pub fn bold(self) -> Self {
        self.set_style(|styles| styles.bold = true)
    }

    /// Add dim text styling.
    #[must_use]
    pub fn dim(self) -> Self {
        self.set_style(|styles| styles.dim = true)
    }

    /// Add italic text styling.
    #[must_use]
    pub fn italic(self) -> Self {
        self.set_style(|styles| styles.italic = true)
    }

    /// Add underline text styling.
    #[must_use]
    pub fn underline(self) -> Self {
        self.set_style(|styles| styles.underline = true)
    }

    /// Swap the foreground and background when rendered.
    #[must_use]
    pub fn inverse(self) -> Self {
        self.set_style(|styles| styles.inverse = true)
    }

    /// Add strikethrough text styling.
    #[must_use]
    pub fn strikethrough(self) -> Self {
        self.set_style(|styles| styles.strikethrough = true)
    }

    /// Apply the standard red background color.
    #[must_use]
    pub fn on_red(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Red))
    }

    /// Apply the standard green background color.
    #[must_use]
    pub fn on_green(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Green))
    }

    /// Apply the standard yellow background color.
    #[must_use]
    pub fn on_yellow(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Yellow))
    }

    /// Apply the standard blue background color.
    #[must_use]
    pub fn on_blue(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Blue))
    }

    /// Apply the standard magenta background color.
    #[must_use]
    pub fn on_magenta(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Magenta))
    }

    /// Apply the standard cyan background color.
    #[must_use]
    pub fn on_cyan(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Cyan))
    }

    /// Apply the standard white background color.
    #[must_use]
    pub fn on_white(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::White))
    }

    /// Apply the standard black background color.
    #[must_use]
    pub fn on_black(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Black))
    }

    /// Apply a true-color RGB foreground.
    #[must_use]
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_foreground(ColorSpec::Rgb(r, g, b))
    }

    /// Apply a true-color RGB background.
    #[must_use]
    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_background(ColorSpec::Rgb(r, g, b))
    }

    /// Convert HSL to RGB and apply it to the foreground color.
    #[must_use]
    pub fn hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.rgb(r, g, b)
    }

    /// Convert HSL to RGB and apply it to the background color.
    #[must_use]
    pub fn on_hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.on_rgb(r, g, b)
    }

    /// Apply a hex foreground color.
    ///
    /// Invalid input clears all styling and returns plain text.
    #[must_use]
    pub fn hex(self, hex: &str) -> Self {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.rgb(r, g, b)
        } else {
            self.clear()
        }
    }

    /// Apply a hex background color.
    ///
    /// Invalid input clears all styling and returns plain text.
    #[must_use]
    pub fn on_hex(self, hex: &str) -> Self {
        if let Some((r, g, b)) = hex_to_rgb(hex) {
            self.on_rgb(r, g, b)
        } else {
            self.clear()
        }
    }

    /// Remove all applied styling and return plain text.
    #[must_use]
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
    /// Apply a raw ANSI SGR code sequence to a displayable value.
    fn colorize(&self, color_code: &str) -> StyledText;

    /// Apply the standard red foreground color.
    fn red(&self) -> StyledText;
    /// Apply the standard green foreground color.
    fn green(&self) -> StyledText;
    /// Apply the standard yellow foreground color.
    fn yellow(&self) -> StyledText;
    /// Apply the standard blue foreground color.
    fn blue(&self) -> StyledText;
    /// Apply the standard magenta foreground color.
    fn magenta(&self) -> StyledText;
    /// Apply the standard cyan foreground color.
    fn cyan(&self) -> StyledText;
    /// Apply the standard white foreground color.
    fn white(&self) -> StyledText;
    /// Apply the standard black foreground color.
    fn black(&self) -> StyledText;

    /// Apply the bright red foreground color.
    fn bright_red(&self) -> StyledText;
    /// Apply the bright green foreground color.
    fn bright_green(&self) -> StyledText;
    /// Apply the bright yellow foreground color.
    fn bright_yellow(&self) -> StyledText;
    /// Apply the bright blue foreground color.
    fn bright_blue(&self) -> StyledText;
    /// Apply the bright magenta foreground color.
    fn bright_magenta(&self) -> StyledText;
    /// Apply the bright cyan foreground color.
    fn bright_cyan(&self) -> StyledText;
    /// Apply the bright white foreground color.
    fn bright_white(&self) -> StyledText;

    /// Add bold text styling.
    fn bold(&self) -> StyledText;
    /// Add dim text styling.
    fn dim(&self) -> StyledText;
    /// Add italic text styling.
    fn italic(&self) -> StyledText;
    /// Add underline text styling.
    fn underline(&self) -> StyledText;
    /// Swap foreground and background when rendered.
    fn inverse(&self) -> StyledText;
    /// Add strikethrough text styling.
    fn strikethrough(&self) -> StyledText;

    /// Apply the standard red background color.
    fn on_red(&self) -> StyledText;
    /// Apply the standard green background color.
    fn on_green(&self) -> StyledText;
    /// Apply the standard yellow background color.
    fn on_yellow(&self) -> StyledText;
    /// Apply the standard blue background color.
    fn on_blue(&self) -> StyledText;
    /// Apply the standard magenta background color.
    fn on_magenta(&self) -> StyledText;
    /// Apply the standard cyan background color.
    fn on_cyan(&self) -> StyledText;
    /// Apply the standard white background color.
    fn on_white(&self) -> StyledText;
    /// Apply the standard black background color.
    fn on_black(&self) -> StyledText;

    /// Apply a true-color RGB foreground.
    fn rgb(&self, r: u8, g: u8, b: u8) -> StyledText;
    /// Apply a true-color RGB background.
    fn on_rgb(&self, r: u8, g: u8, b: u8) -> StyledText;
    /// Convert HSL to RGB and apply it to the foreground.
    fn hsl(&self, h: f32, s: f32, l: f32) -> StyledText;
    /// Convert HSL to RGB and apply it to the background.
    fn on_hsl(&self, h: f32, s: f32, l: f32) -> StyledText;
    /// Apply a hex foreground color, or plain text on invalid input.
    fn hex(&self, hex: &str) -> StyledText;
    /// Apply a hex background color, or plain text on invalid input.
    fn on_hex(&self, hex: &str) -> StyledText;
    /// Remove all styling and return plain text.
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
