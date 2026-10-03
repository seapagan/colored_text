use super::TestStateGuard;
use crate::*;
use rstest::rstest;

const PALETTE_CODES: [(AnsiColor, u8); 16] = [
    (AnsiColor::Black, 30),
    (AnsiColor::Red, 31),
    (AnsiColor::Green, 32),
    (AnsiColor::Yellow, 33),
    (AnsiColor::Blue, 34),
    (AnsiColor::Magenta, 35),
    (AnsiColor::Cyan, 36),
    (AnsiColor::White, 37),
    (AnsiColor::BrightBlack, 90),
    (AnsiColor::BrightRed, 91),
    (AnsiColor::BrightGreen, 92),
    (AnsiColor::BrightYellow, 93),
    (AnsiColor::BrightBlue, 94),
    (AnsiColor::BrightMagenta, 95),
    (AnsiColor::BrightCyan, 96),
    (AnsiColor::BrightWhite, 97),
];

fn renderer_codes(color: ResolvedColor) -> String {
    match color {
        ResolvedColor::Named(color) => {
            let code = PALETTE_CODES
                .iter()
                .find(|entry| entry.0 == color)
                .unwrap()
                .1;
            format!("{code};{}", code + 10)
        }
        ResolvedColor::Ansi256(index) => format!("38;5;{index};48;5;{index}"),
        ResolvedColor::Rgb(r, g, b) => format!("38;2;{r};{g};{b};48;2;{r};{g};{b}"),
    }
}

fn assert_renderer_parity(styled: StyledText, resolved: Option<ResolvedColor>, level: ColorLevel) {
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });
    let expected = match resolved {
        Some(color) => format!("\x1b[{}mtest\x1b[0m", renderer_codes(color)),
        None => "test".to_owned(),
    };
    assert_eq!(styled.render(target), expected);
}

#[rstest]
fn named_renderer_matches_structured_resolution(
    #[values(
        ColorLevel::NoColor,
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor
    )]
    level: ColorLevel,
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let styles = [
        "test".black().on_black(),
        "test".red().on_red(),
        "test".green().on_green(),
        "test".yellow().on_yellow(),
        "test".blue().on_blue(),
        "test".magenta().on_magenta(),
        "test".cyan().on_cyan(),
        "test".white().on_white(),
        "test".bright_black().on_bright_black(),
        "test".bright_red().on_bright_red(),
        "test".bright_green().on_bright_green(),
        "test".bright_yellow().on_bright_yellow(),
        "test".bright_blue().on_bright_blue(),
        "test".bright_magenta().on_bright_magenta(),
        "test".bright_cyan().on_bright_cyan(),
        "test".bright_white().on_bright_white(),
    ];
    for ((color, _), styled) in PALETTE_CODES.into_iter().zip(styles) {
        assert_renderer_parity(styled, resolve_named(color, level), level);
    }
}

#[rstest]
fn rgb_renderer_matches_structured_resolution(
    #[values(
        ColorLevel::NoColor,
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor
    )]
    level: ColorLevel,
    #[values((0, 0, 0), (255, 255, 255), (255, 0, 0), (0, 255, 0), (0, 0, 255), (95, 135, 175), (118, 118, 118), (128, 0, 0), (215, 58, 74), (64, 64, 64))]
    rgb: (u8, u8, u8),
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let (r, g, b) = rgb;
    assert_renderer_parity(
        "test".rgb(r, g, b).on_rgb(r, g, b),
        resolve_rgb(r, g, b, level),
        level,
    );
}

#[rstest]
fn indexed_renderer_matches_structured_resolution(
    #[values(
        ColorLevel::NoColor,
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor
    )]
    level: ColorLevel,
    #[values(0, 8, 15, 16, 67, 208, 231, 232, 236, 255)] index: u8,
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_renderer_parity(
        "test".ansi256(index).on_ansi256(index),
        resolve_ansi256(index, level),
        level,
    );
}

#[rstest]
fn hsl_renderer_matches_structured_resolution(
    #[values(
        ColorLevel::NoColor,
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor
    )]
    level: ColorLevel,
    #[values((0., 100., 50.), (360., 100., 50.), (120., 100., 50.), (240., 100., 50.), (0., 0., 50.))]
    hsl: (f32, f32, f32),
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let (h, s, l) = hsl;
    assert_renderer_parity(
        "test".hsl(h, s, l).on_hsl(h, s, l),
        resolve_hsl(h, s, l, level).unwrap(),
        level,
    );
}

#[rstest]
fn hex_renderer_matches_structured_resolution(
    #[values(
        ColorLevel::NoColor,
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor
    )]
    level: ColorLevel,
    #[values("f80", "#F80", "d73A4a", "#d73a4a", "000", "#FFFFFF")] hex: &str,
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_renderer_parity(
        "test".hex(hex).on_hex(hex),
        resolve_hex(hex, level).unwrap(),
        level,
    );
}
