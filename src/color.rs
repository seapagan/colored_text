use crate::resolution::ResolvedColor;
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

const NAMED_COLORS: [AnsiColor; 16] = [
    AnsiColor::Black,
    AnsiColor::Red,
    AnsiColor::Green,
    AnsiColor::Yellow,
    AnsiColor::Blue,
    AnsiColor::Magenta,
    AnsiColor::Cyan,
    AnsiColor::White,
    AnsiColor::BrightBlack,
    AnsiColor::BrightRed,
    AnsiColor::BrightGreen,
    AnsiColor::BrightYellow,
    AnsiColor::BrightBlue,
    AnsiColor::BrightMagenta,
    AnsiColor::BrightCyan,
    AnsiColor::BrightWhite,
];

/// The standard 16 terminal palette colors.
///
/// Their appearance depends on the terminal theme. Bright variants select
/// distinct palette entries, but some themes display them similarly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AnsiColor {
    /// Standard black terminal palette entry.
    Black,
    /// Standard red terminal palette entry.
    Red,
    /// Standard green terminal palette entry.
    Green,
    /// Standard yellow terminal palette entry.
    Yellow,
    /// Standard blue terminal palette entry.
    Blue,
    /// Standard magenta terminal palette entry.
    Magenta,
    /// Standard cyan terminal palette entry.
    Cyan,
    /// Standard white terminal palette entry.
    White,
    /// Bright black terminal palette entry.
    BrightBlack,
    /// Bright red terminal palette entry.
    BrightRed,
    /// Bright green terminal palette entry.
    BrightGreen,
    /// Bright yellow terminal palette entry.
    BrightYellow,
    /// Bright blue terminal palette entry.
    BrightBlue,
    /// Bright magenta terminal palette entry.
    BrightMagenta,
    /// Bright cyan terminal palette entry.
    BrightCyan,
    /// Bright white terminal palette entry.
    BrightWhite,
}

impl AnsiColor {
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
    Named(AnsiColor),
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

    pub(crate) fn resolve(&self, level: ColorLevel) -> Option<ResolvedColor> {
        match (level, self) {
            (ColorLevel::NoColor, _) => None,
            (_, Self::Named(color)) => Some(ResolvedColor::Named(*color)),
            (ColorLevel::Ansi16, Self::Ansi256(index)) => {
                Some(ResolvedColor::Named(ansi256_to_named_color(*index)))
            }
            (ColorLevel::Ansi16, Self::Rgb(r, g, b)) => {
                Some(ResolvedColor::Named(rgb_to_named_color(*r, *g, *b)))
            }
            (ColorLevel::Ansi256 | ColorLevel::TrueColor, Self::Ansi256(index)) => {
                Some(ResolvedColor::Ansi256(*index))
            }
            (ColorLevel::Ansi256, Self::Rgb(r, g, b)) => {
                Some(ResolvedColor::Ansi256(rgb_to_ansi256(*r, *g, *b)))
            }
            (ColorLevel::TrueColor, Self::Rgb(r, g, b)) => Some(ResolvedColor::Rgb(*r, *g, *b)),
        }
    }

    fn code(&self, level: ColorLevel, position: ColorPosition) -> Option<String> {
        self.resolve(level).map(|color| match color {
            ResolvedColor::Named(color) => position.named_code(color),
            ResolvedColor::Ansi256(index) => format!("{};5;{index}", position.extended_prefix()),
            ResolvedColor::Rgb(r, g, b) => {
                format!("{};2;{r};{g};{b}", position.extended_prefix())
            }
        })
    }
}

#[derive(Clone, Copy)]
enum ColorPosition {
    Foreground,
    Background,
}

impl ColorPosition {
    fn named_code(self, color: AnsiColor) -> String {
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

pub(crate) fn rgb_to_named_color(r: u8, g: u8, b: u8) -> AnsiColor {
    let target = (r, g, b);
    let (best, _) = nearest_by_distance(named_color_candidates(), |candidate| {
        distance_squared(target, candidate.1)
    });
    best
}

pub(crate) fn ansi256_to_named_color(index: u8) -> AnsiColor {
    let (r, g, b) = ansi256_to_rgb(index);
    rgb_to_named_color(r, g, b)
}

fn named_color_candidates() -> impl Iterator<Item = (AnsiColor, (u8, u8, u8))> {
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
