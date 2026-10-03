//! Renderer-neutral color resolution, independent of environment detection.

use crate::color::{self, ColorSpec};
use crate::{AnsiColor, ColorLevel};
use std::fmt;

/// Concrete color data selected for a terminal depth, with no ANSI escapes.
///
/// `None` from a resolver means a valid color is disabled by [`ColorLevel::NoColor`].
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum ResolvedColor {
    /// A terminal palette entry whose appearance depends on the terminal theme.
    Named(AnsiColor),
    /// An indexed ANSI 256-color palette entry, including indexes 0–15.
    Ansi256(u8),
    /// An exact 24-bit RGB value.
    Rgb(u8, u8, u8),
}

/// Invalid color input, distinct from valid input resolved under `NoColor`.
#[derive(Debug, Clone, PartialEq)]
pub enum ColorInputError {
    /// Hex input does not match the supported ASCII grammar.
    InvalidHex {
        /// The original input, retained for diagnostics.
        input: String,
    },
    /// An HSL component is non-finite or outside its documented range.
    InvalidHsl {
        /// The invalid component: `"hue"`, `"saturation"`, or `"lightness"`.
        component: &'static str,
        /// The rejected value, including non-finite values.
        value: f32,
    },
}

impl fmt::Display for ColorInputError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidHex { input } => write!(
                f,
                "invalid hex color {input:?}: expected RGB or RRGGBB with an optional # prefix"
            ),
            Self::InvalidHsl { component, value } => write!(
                f,
                "invalid HSL {component}: {value} is non-finite or out of range"
            ),
        }
    }
}

impl std::error::Error for ColorInputError {}

/// Resolve a named terminal palette color for an explicit color level.
///
/// Returns `None` at [`ColorLevel::NoColor`]. At `Ansi16`, `Ansi256`, and
/// `TrueColor`, returns [`ResolvedColor::Named`] with the same palette identity;
/// named colors never become fixed RGB values. Does not inspect the environment.
pub fn resolve_named(color: AnsiColor, level: ColorLevel) -> Option<ResolvedColor> {
    ColorSpec::Named(color).resolve(level)
}

/// Resolve an ANSI 256-color index for an explicit color level.
///
/// Returns `None` at `NoColor`, the nearest named palette color at `Ansi16`,
/// and the original [`ResolvedColor::Ansi256`] index at `Ansi256` and `TrueColor`.
/// An indexed input is never promoted to RGB. Does not inspect the environment.
pub fn resolve_ansi256(index: u8, level: ColorLevel) -> Option<ResolvedColor> {
    ColorSpec::Ansi256(index).resolve(level)
}

/// Resolve RGB for an explicit color level using the same policy as [`crate::StyledText`].
///
/// Returns `None` at `NoColor`, the nearest named palette color at `Ansi16`,
/// the nearest ANSI 256-color index at `Ansi256`, and exact RGB at `TrueColor`.
/// ANSI 256 selection compares the base palette, color cube, and grayscale ramp;
/// ties are deterministic. Does not inspect the environment.
///
/// ```
/// use colored_text::{resolve_rgb, ColorLevel, ResolvedColor};
/// assert_eq!(resolve_rgb(215, 58, 74, ColorLevel::TrueColor),
///     Some(ResolvedColor::Rgb(215, 58, 74)));
/// ```
pub fn resolve_rgb(r: u8, g: u8, b: u8, level: ColorLevel) -> Option<ResolvedColor> {
    ColorSpec::Rgb(r, g, b).resolve(level)
}

/// Validate HSL, convert it to RGB, then apply [`resolve_rgb`]'s depth policy.
///
/// Hue is in `0..=360` degrees; saturation and lightness are in `0..=100`
/// percent. Hue 360 is exactly equivalent to 0. Conversion uses the existing
/// styling algorithm, which truncates channels to `u8`. Does not inspect the environment.
///
/// # Errors
///
/// Returns [`ColorInputError::InvalidHsl`] for any non-finite or out-of-range
/// component, even at `NoColor`. Valid input at `NoColor` returns `Ok(None)`.
/// Legacy `.hsl()` / `.on_hsl()` styling methods retain their permissive behavior.
pub fn resolve_hsl(
    h: f32,
    s: f32,
    l: f32,
    level: ColorLevel,
) -> Result<Option<ResolvedColor>, ColorInputError> {
    let (r, g, b) = hsl_to_rgb(h, s, l)?;
    Ok(resolve_rgb(r, g, b, level))
}

/// Parse hex, then apply [`resolve_rgb`]'s depth policy.
///
/// Accepts ASCII `RGB`, `#RGB`, `RRGGBB`, and `#RRGGBB`, case-insensitively.
/// Three-digit shorthand duplicates each digit. Whitespace, repeated prefixes,
/// and non-hex characters are rejected. Does not inspect the environment.
///
/// # Errors
///
/// Returns [`ColorInputError::InvalidHex`] for malformed input, even at `NoColor`.
/// Valid input at `NoColor` returns `Ok(None)`. Legacy `.hex()` / `.on_hex()`
/// methods retain their existing grammar and plain-text fallback behavior.
pub fn resolve_hex(
    input: &str,
    level: ColorLevel,
) -> Result<Option<ResolvedColor>, ColorInputError> {
    let (r, g, b) = hex_to_rgb(input)?;
    Ok(resolve_rgb(r, g, b, level))
}

/// Validate and convert HSL to RGB without terminal resolution or environment detection.
///
/// Hue is in `0..=360` degrees; saturation and lightness are in `0..=100`
/// percent. Hue 360 is exactly equivalent to 0. Conversion uses the existing
/// styling algorithm, which truncates channels to `u8`.
/// [`resolve_hsl`] uses this conversion before applying [`resolve_rgb`].
/// Legacy `.hsl()` / `.on_hsl()` styling methods remain permissive.
///
/// # Errors
///
/// Returns [`ColorInputError::InvalidHsl`] for the first non-finite or out-of-range
/// component, checking hue, saturation, then lightness.
///
/// ```
/// use colored_text::hsl_to_rgb;
/// assert_eq!(hsl_to_rgb(360.0, 100.0, 50.0)?, (255, 0, 0));
/// assert_eq!(hsl_to_rgb(0.0, 0.0, 50.0)?, (127, 127, 127));
/// # Ok::<(), colored_text::ColorInputError>(())
/// ```
pub fn hsl_to_rgb(h: f32, s: f32, l: f32) -> Result<(u8, u8, u8), ColorInputError> {
    for (component, value, maximum) in [
        ("hue", h, 360.),
        ("saturation", s, 100.),
        ("lightness", l, 100.),
    ] {
        if !value.is_finite() || !(0.0..=maximum).contains(&value) {
            return Err(ColorInputError::InvalidHsl { component, value });
        }
    }
    Ok(color::legacy_permissive_hsl_to_rgb(
        if h == 360. { 0. } else { h },
        s,
        l,
    ))
}

/// Parse hex into RGB without terminal resolution or environment detection.
///
/// Accepts ASCII `RGB`, `#RGB`, `RRGGBB`, and `#RRGGBB`, case-insensitively.
/// Three-digit shorthand duplicates each digit. [`resolve_hex`] uses this
/// parser before applying [`resolve_rgb`]. Legacy `.hex()` / `.on_hex()`
/// methods retain their existing grammar and plain-text fallback behavior.
///
/// # Errors
///
/// Returns [`ColorInputError::InvalidHex`] for malformed input, including wrong
/// lengths, whitespace, repeated prefixes, and non-ASCII or non-hex characters.
/// The error retains the original input.
///
/// ```
/// use colored_text::hex_to_rgb;
/// assert_eq!(hex_to_rgb("#aBc")?, (170, 187, 204));
/// assert_eq!(hex_to_rgb("d73a4a")?, (215, 58, 74));
/// # Ok::<(), colored_text::ColorInputError>(())
/// ```
pub fn hex_to_rgb(input: &str) -> Result<(u8, u8, u8), ColorInputError> {
    let hex = input.strip_prefix('#').unwrap_or(input);
    let rgb = if hex.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        color::legacy_permissive_hex_to_rgb(hex)
    } else {
        None
    };
    rgb.ok_or_else(|| ColorInputError::InvalidHex {
        input: input.to_owned(),
    })
}
