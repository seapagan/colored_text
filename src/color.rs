use crate::terminal::ColorLevel;

const ANSI256_STEPS: [u8; 6] = [0, 95, 135, 175, 215, 255];

const ANSI16_RGB: [(u8, u8, u8); 16] = [
    (0, 0, 0),
    (128, 0, 0),
    (0, 128, 0),
    (128, 128, 0),
    (0, 0, 128),
    (128, 0, 128),
    (0, 128, 128),
    (192, 192, 192),
    (128, 128, 128),
    (255, 0, 0),
    (0, 255, 0),
    (255, 255, 0),
    (0, 0, 255),
    (255, 0, 255),
    (0, 255, 255),
    (255, 255, 255),
];

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
    let expanded = match hex.len() {
        3 => {
            let mut expanded = String::with_capacity(6);
            for ch in hex.chars() {
                expanded.push(ch);
                expanded.push(ch);
            }
            expanded
        }
        6 => hex.to_string(),
        _ => return None,
    };

    let r = u8::from_str_radix(&expanded[0..2], 16).ok()?;
    let g = u8::from_str_radix(&expanded[2..4], 16).ok()?;
    let b = u8::from_str_radix(&expanded[4..6], 16).ok()?;

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
    pub(crate) fn foreground_code(self) -> &'static str {
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

    pub(crate) fn background_code(self) -> &'static str {
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
    Ansi256(u8),
    Rgb(u8, u8, u8),
}

impl ColorSpec {
    pub(crate) fn foreground_code(&self, level: ColorLevel) -> Option<String> {
        match self {
            Self::Named(color) if level >= ColorLevel::Ansi16 => {
                Some(color.foreground_code().to_string())
            }
            Self::Ansi256(index) if level >= ColorLevel::Ansi256 => Some(format!("38;5;{index}")),
            Self::Ansi256(index) if level == ColorLevel::Ansi16 => {
                Some(ansi256_to_named_color(*index).foreground_code().to_string())
            }
            Self::Rgb(r, g, b) if level == ColorLevel::TrueColor => {
                Some(format!("38;2;{};{};{}", r, g, b))
            }
            Self::Rgb(r, g, b) if level == ColorLevel::Ansi256 => {
                Some(format!("38;5;{}", rgb_to_ansi256(*r, *g, *b)))
            }
            Self::Rgb(r, g, b) if level == ColorLevel::Ansi16 => {
                Some(rgb_to_named_color(*r, *g, *b).foreground_code().to_string())
            }
            _ => None,
        }
    }

    pub(crate) fn background_code(&self, level: ColorLevel) -> Option<String> {
        match self {
            Self::Named(color) if level >= ColorLevel::Ansi16 => {
                Some(color.background_code().to_string())
            }
            Self::Ansi256(index) if level >= ColorLevel::Ansi256 => Some(format!("48;5;{index}")),
            Self::Ansi256(index) if level == ColorLevel::Ansi16 => {
                Some(ansi256_to_named_color(*index).background_code().to_string())
            }
            Self::Rgb(r, g, b) if level == ColorLevel::TrueColor => {
                Some(format!("48;2;{};{};{}", r, g, b))
            }
            Self::Rgb(r, g, b) if level == ColorLevel::Ansi256 => {
                Some(format!("48;5;{}", rgb_to_ansi256(*r, *g, *b)))
            }
            Self::Rgb(r, g, b) if level == ColorLevel::Ansi16 => {
                Some(rgb_to_named_color(*r, *g, *b).background_code().to_string())
            }
            _ => None,
        }
    }
}

pub(crate) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let target = (r, g, b);
    let mut best_index = 0;
    let mut best_distance = u32::MAX;

    for index in 0..=255 {
        let candidate = ansi256_to_rgb(index);
        let distance = distance_squared(target, candidate);
        if distance < best_distance {
            best_index = index;
            best_distance = distance;
        }
    }

    best_index
}

pub(crate) fn ansi256_to_rgb(index: u8) -> (u8, u8, u8) {
    match index {
        0..=15 => ANSI16_RGB[usize::from(index)],
        16..=231 => {
            let offset = index - 16;
            let red = offset / 36;
            let green = (offset % 36) / 6;
            let blue = offset % 6;
            (
                ANSI256_STEPS[usize::from(red)],
                ANSI256_STEPS[usize::from(green)],
                ANSI256_STEPS[usize::from(blue)],
            )
        }
        232..=255 => {
            let value = 8 + (index - 232) * 10;
            (value, value, value)
        }
    }
}

pub(crate) fn rgb_to_named_color(r: u8, g: u8, b: u8) -> NamedColor {
    let target = (r, g, b);
    let mut best = NamedColor::Black;
    let mut best_distance = u32::MAX;

    for (color, rgb) in named_color_candidates() {
        let distance = distance_squared(target, rgb);
        if distance < best_distance {
            best = color;
            best_distance = distance;
        }
    }

    best
}

pub(crate) fn ansi256_to_named_color(index: u8) -> NamedColor {
    let (r, g, b) = ansi256_to_rgb(index);
    rgb_to_named_color(r, g, b)
}

fn named_color_candidates() -> [(NamedColor, (u8, u8, u8)); 15] {
    [
        (NamedColor::Black, ANSI16_RGB[0]),
        (NamedColor::Red, ANSI16_RGB[1]),
        (NamedColor::Green, ANSI16_RGB[2]),
        (NamedColor::Yellow, ANSI16_RGB[3]),
        (NamedColor::Blue, ANSI16_RGB[4]),
        (NamedColor::Magenta, ANSI16_RGB[5]),
        (NamedColor::Cyan, ANSI16_RGB[6]),
        (NamedColor::White, ANSI16_RGB[7]),
        (NamedColor::BrightRed, ANSI16_RGB[9]),
        (NamedColor::BrightGreen, ANSI16_RGB[10]),
        (NamedColor::BrightYellow, ANSI16_RGB[11]),
        (NamedColor::BrightBlue, ANSI16_RGB[12]),
        (NamedColor::BrightMagenta, ANSI16_RGB[13]),
        (NamedColor::BrightCyan, ANSI16_RGB[14]),
        (NamedColor::BrightWhite, ANSI16_RGB[15]),
    ]
}

fn distance_squared(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    let dr = i32::from(a.0) - i32::from(b.0);
    let dg = i32::from(a.1) - i32::from(b.1);
    let db = i32::from(a.2) - i32::from(b.2);
    (dr * dr + dg * dg + db * db) as u32
}
