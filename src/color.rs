/// Convert HSL color values to RGB.
///
/// - `h`: Hue in degrees
/// - `s`: Saturation percentage
/// - `l`: Lightness percentage
pub(crate) fn hsl_to_rgb(h: f32, s: f32, l: f32) -> (u8, u8, u8) {
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

pub(crate) fn hex_to_rgb(hex: &str) -> Option<(u8, u8, u8)> {
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
pub(crate) enum NamedColor {
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
pub(crate) enum ColorSpec {
    Named(NamedColor),
    Rgb(u8, u8, u8),
}

impl ColorSpec {
    pub(crate) fn foreground_code(&self) -> String {
        match self {
            Self::Named(color) => color.foreground_code().to_string(),
            Self::Rgb(r, g, b) => format!("38;2;{};{};{}", r, g, b),
        }
    }

    pub(crate) fn background_code(&self) -> String {
        match self {
            Self::Named(color) => color.background_code().to_string(),
            Self::Rgb(r, g, b) => format!("48;2;{};{};{}", r, g, b),
        }
    }
}
