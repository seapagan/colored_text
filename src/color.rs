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

const NAMED_COLORS: [NamedColor; 16] = [
    NamedColor::Black,
    NamedColor::Red,
    NamedColor::Green,
    NamedColor::Yellow,
    NamedColor::Blue,
    NamedColor::Magenta,
    NamedColor::Cyan,
    NamedColor::White,
    NamedColor::BrightBlack,
    NamedColor::BrightRed,
    NamedColor::BrightGreen,
    NamedColor::BrightYellow,
    NamedColor::BrightBlue,
    NamedColor::BrightMagenta,
    NamedColor::BrightCyan,
    NamedColor::BrightWhite,
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
    BrightBlack,
    BrightRed,
    BrightGreen,
    BrightYellow,
    BrightBlue,
    BrightMagenta,
    BrightCyan,
    BrightWhite,
}

impl NamedColor {
    pub(crate) fn foreground_code(self) -> String {
        self.foreground_code_value().to_string()
    }

    pub(crate) fn background_code(self) -> String {
        (self.foreground_code_value() + 10).to_string()
    }

    fn foreground_code_value(self) -> u8 {
        match self {
            Self::Black => 30,
            Self::Red => 31,
            Self::Green => 32,
            Self::Yellow => 33,
            Self::Blue => 34,
            Self::Magenta => 35,
            Self::Cyan => 36,
            Self::White => 37,
            Self::BrightBlack => 90,
            Self::BrightRed => 91,
            Self::BrightGreen => 92,
            Self::BrightYellow => 93,
            Self::BrightBlue => 94,
            Self::BrightMagenta => 95,
            Self::BrightCyan => 96,
            Self::BrightWhite => 97,
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
        self.code(level, ColorPosition::Foreground)
    }

    pub(crate) fn background_code(&self, level: ColorLevel) -> Option<String> {
        self.code(level, ColorPosition::Background)
    }

    fn code(&self, level: ColorLevel, position: ColorPosition) -> Option<String> {
        match (level, self) {
            (ColorLevel::NoColor, _) => None,
            (_, Self::Named(color)) => Some(position.named_code(*color)),
            (ColorLevel::Ansi16, Self::Ansi256(index)) => {
                Some(position.named_code(ansi256_to_named_color(*index)))
            }
            (ColorLevel::Ansi16, Self::Rgb(r, g, b)) => {
                Some(position.named_code(rgb_to_named_color(*r, *g, *b)))
            }
            (ColorLevel::Ansi256 | ColorLevel::TrueColor, Self::Ansi256(index)) => {
                Some(format!("{};5;{index}", position.extended_prefix()))
            }
            (ColorLevel::Ansi256, Self::Rgb(r, g, b)) => Some(format!(
                "{};5;{}",
                position.extended_prefix(),
                rgb_to_ansi256(*r, *g, *b)
            )),
            (ColorLevel::TrueColor, Self::Rgb(r, g, b)) => Some(format!(
                "{};2;{};{};{}",
                position.extended_prefix(),
                r,
                g,
                b
            )),
        }
    }
}

#[derive(Clone, Copy)]
enum ColorPosition {
    Foreground,
    Background,
}

impl ColorPosition {
    fn named_code(self, color: NamedColor) -> String {
        match self {
            Self::Foreground => color.foreground_code(),
            Self::Background => color.background_code(),
        }
    }

    fn extended_prefix(self) -> &'static str {
        match self {
            Self::Foreground => "38",
            Self::Background => "48",
        }
    }
}

pub(crate) fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
    let target = (r, g, b);
    let mut best_index = nearest_by_distance(0..=15, |index| {
        distance_squared(target, ansi256_to_rgb(index))
    });
    let mut best_distance = distance_squared(target, ansi256_to_rgb(best_index));

    let cube_index = rgb_to_ansi256_cube(r, g, b);
    let cube_distance = distance_squared(target, ansi256_to_rgb(cube_index));
    if cube_distance < best_distance {
        best_index = cube_index;
        best_distance = cube_distance;
    }

    let gray_index = rgb_to_ansi256_gray(r, g, b);
    let gray_distance = distance_squared(target, ansi256_to_rgb(gray_index));
    if gray_distance < best_distance {
        best_index = gray_index;
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

fn rgb_to_ansi256_cube(r: u8, g: u8, b: u8) -> u8 {
    let red = nearest_ansi256_cube_component(r);
    let green = nearest_ansi256_cube_component(g);
    let blue = nearest_ansi256_cube_component(b);
    16 + 36 * red + 6 * green + blue
}

fn nearest_ansi256_cube_component(value: u8) -> u8 {
    let (index, _) = nearest_by_distance(ANSI256_STEPS.iter().copied().enumerate(), |candidate| {
        component_distance_squared(value, candidate.1)
    });
    index as u8
}

fn rgb_to_ansi256_gray(r: u8, g: u8, b: u8) -> u8 {
    let average = (u16::from(r) + u16::from(g) + u16::from(b)) / 3;
    let ramp_index = if average <= 8 {
        0
    } else if average >= 238 {
        23
    } else {
        ((average - 8) + 5) / 10
    };

    232 + ramp_index as u8
}

pub(crate) fn rgb_to_named_color(r: u8, g: u8, b: u8) -> NamedColor {
    let target = (r, g, b);
    let (best, _) = nearest_by_distance(named_color_candidates(), |candidate| {
        distance_squared(target, candidate.1)
    });
    best
}

pub(crate) fn ansi256_to_named_color(index: u8) -> NamedColor {
    let (r, g, b) = ansi256_to_rgb(index);
    rgb_to_named_color(r, g, b)
}

fn named_color_candidates() -> impl Iterator<Item = (NamedColor, (u8, u8, u8))> {
    NAMED_COLORS.into_iter().zip(ANSI16_RGB)
}

fn nearest_by_distance<T>(candidates: impl IntoIterator<Item = T>, distance: impl Fn(T) -> u32) -> T
where
    T: Copy,
{
    let mut candidates = candidates.into_iter();
    let mut best = candidates
        .next()
        .expect("nearest_by_distance requires at least one candidate");
    let mut best_distance = distance(best);

    for candidate in candidates {
        let candidate_distance = distance(candidate);
        // Strict comparison keeps candidate order as the deterministic
        // tie-breaker.
        if candidate_distance < best_distance {
            best = candidate;
            best_distance = candidate_distance;
        }
    }

    best
}

fn distance_squared(a: (u8, u8, u8), b: (u8, u8, u8)) -> u32 {
    component_distance_squared(a.0, b.0)
        + component_distance_squared(a.1, b.1)
        + component_distance_squared(a.2, b.2)
}

fn component_distance_squared(a: u8, b: u8) -> u32 {
    let distance = i32::from(a) - i32::from(b);
    distance.unsigned_abs().pow(2)
}
