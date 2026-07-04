use std::fmt::{self, Display};

use crate::color::{hex_to_rgb, hsl_to_rgb, ColorSpec, NamedColor};
use crate::config::{color_level, color_level_for, RenderTarget};
use crate::terminal::ColorLevel;

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

    fn active_codes(&self, level: ColorLevel) -> Vec<String> {
        if level == ColorLevel::NoColor {
            return Vec::new();
        }

        let mut codes = self.raw_codes.clone();
        codes.extend(self.styles.sgr_codes());

        if let Some(foreground) = &self.foreground {
            if let Some(code) = foreground.foreground_code(level) {
                codes.push(code);
            }
        }

        if let Some(background) = &self.background {
            if let Some(code) = background.background_code(level) {
                codes.push(code);
            }
        }

        codes
    }

    /// Apply a raw ANSI SGR code sequence to the value.
    ///
    /// This is an escape hatch for manual SGR composition. Prefer the typed
    /// color and style methods when possible.
    pub fn colorize(mut self, color_code: &str) -> Self {
        if !color_code.trim().is_empty() {
            self.raw_codes.push(color_code.to_string());
        }
        self
    }

    /// Apply the standard red foreground color.
    pub fn red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Red))
    }

    /// Apply the standard green foreground color.
    pub fn green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Green))
    }

    /// Apply the standard yellow foreground color.
    pub fn yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Yellow))
    }

    /// Apply the standard blue foreground color.
    pub fn blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Blue))
    }

    /// Apply the standard magenta foreground color.
    pub fn magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Magenta))
    }

    /// Apply the standard cyan foreground color.
    pub fn cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Cyan))
    }

    /// Apply the standard white foreground color.
    pub fn white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::White))
    }

    /// Apply the standard black foreground color.
    pub fn black(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::Black))
    }

    /// Apply the bright red foreground color.
    pub fn bright_red(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightRed))
    }

    /// Apply the bright green foreground color.
    pub fn bright_green(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightGreen))
    }

    /// Apply the bright yellow foreground color.
    pub fn bright_yellow(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightYellow))
    }

    /// Apply the bright blue foreground color.
    pub fn bright_blue(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightBlue))
    }

    /// Apply the bright magenta foreground color.
    pub fn bright_magenta(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightMagenta))
    }

    /// Apply the bright cyan foreground color.
    pub fn bright_cyan(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightCyan))
    }

    /// Apply the bright white foreground color.
    pub fn bright_white(self) -> Self {
        self.with_foreground(ColorSpec::Named(NamedColor::BrightWhite))
    }

    /// Add bold text styling.
    pub fn bold(self) -> Self {
        self.set_style(|styles| styles.bold = true)
    }

    /// Add dim text styling.
    pub fn dim(self) -> Self {
        self.set_style(|styles| styles.dim = true)
    }

    /// Add italic text styling.
    pub fn italic(self) -> Self {
        self.set_style(|styles| styles.italic = true)
    }

    /// Add underline text styling.
    pub fn underline(self) -> Self {
        self.set_style(|styles| styles.underline = true)
    }

    /// Swap the foreground and background when rendered.
    pub fn inverse(self) -> Self {
        self.set_style(|styles| styles.inverse = true)
    }

    /// Add strikethrough text styling.
    pub fn strikethrough(self) -> Self {
        self.set_style(|styles| styles.strikethrough = true)
    }

    /// Apply the standard red background color.
    pub fn on_red(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Red))
    }

    /// Apply the standard green background color.
    pub fn on_green(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Green))
    }

    /// Apply the standard yellow background color.
    pub fn on_yellow(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Yellow))
    }

    /// Apply the standard blue background color.
    pub fn on_blue(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Blue))
    }

    /// Apply the standard magenta background color.
    pub fn on_magenta(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Magenta))
    }

    /// Apply the standard cyan background color.
    pub fn on_cyan(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Cyan))
    }

    /// Apply the standard white background color.
    pub fn on_white(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::White))
    }

    /// Apply the standard black background color.
    pub fn on_black(self) -> Self {
        self.with_background(ColorSpec::Named(NamedColor::Black))
    }

    /// Apply an ANSI 256-color foreground.
    pub fn ansi256(self, index: u8) -> Self {
        self.with_foreground(ColorSpec::Ansi256(index))
    }

    /// Apply an ANSI 256-color background.
    pub fn on_ansi256(self, index: u8) -> Self {
        self.with_background(ColorSpec::Ansi256(index))
    }

    /// Alias for [`Self::ansi256`].
    pub fn color256(self, index: u8) -> Self {
        self.ansi256(index)
    }

    /// Alias for [`Self::on_ansi256`].
    pub fn on_color256(self, index: u8) -> Self {
        self.on_ansi256(index)
    }

    /// Apply a true-color RGB foreground.
    pub fn rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_foreground(ColorSpec::Rgb(r, g, b))
    }

    /// Apply a true-color RGB background.
    pub fn on_rgb(self, r: u8, g: u8, b: u8) -> Self {
        self.with_background(ColorSpec::Rgb(r, g, b))
    }

    /// Convert HSL to RGB and apply it to the foreground color.
    pub fn hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.rgb(r, g, b)
    }

    /// Convert HSL to RGB and apply it to the background color.
    pub fn on_hsl(self, h: f32, s: f32, l: f32) -> Self {
        let (r, g, b) = hsl_to_rgb(h, s, l);
        self.on_rgb(r, g, b)
    }

    /// Apply a hex foreground color.
    ///
    /// Invalid input clears all styling and returns plain text.
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

    /// Render the styled value for a specific output target.
    ///
    /// This is useful when the caller knows the real destination is stderr or a
    /// custom writer and wants [`crate::ColorMode::Auto`] to evaluate that
    /// destination instead of the default stdout-based behavior used by
    /// [`Display`].
    pub fn render(&self, target: RenderTarget) -> String {
        self.render_with_color_level(color_level_for(target))
    }

    fn render_with_color_level(&self, level: ColorLevel) -> String {
        let codes = self.active_codes(level);
        if codes.is_empty() {
            return self.text.clone();
        }

        format!("\x1b[{}m{}\x1b[0m", codes.join(";"), self.text)
    }
}

impl Display for StyledText {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.render_with_color_level(color_level()))
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

    /// Apply an ANSI 256-color foreground.
    fn ansi256(&self, index: u8) -> StyledText;
    /// Apply an ANSI 256-color background.
    fn on_ansi256(&self, index: u8) -> StyledText;
    /// Alias for [`Colorize::ansi256`].
    fn color256(&self, index: u8) -> StyledText;
    /// Alias for [`Colorize::on_ansi256`].
    fn on_color256(&self, index: u8) -> StyledText;

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

    fn ansi256(&self, index: u8) -> StyledText {
        StyledText::plain(self.to_string()).ansi256(index)
    }

    fn on_ansi256(&self, index: u8) -> StyledText {
        StyledText::plain(self.to_string()).on_ansi256(index)
    }

    fn color256(&self, index: u8) -> StyledText {
        self.ansi256(index)
    }

    fn on_color256(&self, index: u8) -> StyledText {
        self.on_ansi256(index)
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
