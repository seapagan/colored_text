use super::TestStateGuard;
use crate::color::{
    ansi256_to_named_color, ansi256_to_rgb, rgb_to_ansi256, rgb_to_named_color, AnsiColor,
    ColorSpec,
};
use crate::*;
use rstest::rstest;

#[rstest(level, expected)]
#[case(ColorLevel::TrueColor, "\x1b[38;2;255;128;0mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[33mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_rgb_degrades_by_color_level(level: ColorLevel, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".rgb(255, 128, 0).render(target), expected);
}

#[rstest(level, expected)]
#[case(ColorLevel::TrueColor, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[93mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_ansi256_degrades_by_color_level(level: ColorLevel, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".ansi256(208).render(target), expected);
}

#[rstest(level, expected)]
#[case(ColorLevel::TrueColor, "\x1b[48;2;255;128;0mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[48;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[43mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_rgb_background_degrades_by_color_level(level: ColorLevel, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".on_rgb(255, 128, 0).render(target), expected);
}

#[rstest(level, expected)]
#[case(ColorLevel::TrueColor, "\x1b[48;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[48;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[103mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_ansi256_background_degrades_by_color_level(level: ColorLevel, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".on_ansi256(208).render(target), expected);
}

#[test]
fn test_color_specs_return_none_without_color_support() {
    assert_eq!(
        ColorSpec::Named(AnsiColor::Red).foreground_code(ColorLevel::NoColor),
        None
    );
    assert_eq!(
        ColorSpec::Named(AnsiColor::Red).background_code(ColorLevel::NoColor),
        None
    );
}

#[test]
fn test_styled_text_inherent_ansi256_aliases() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!(
        StyledText::plain("test").color256(208).to_string(),
        "\x1b[38;5;208mtest\x1b[0m"
    );
    assert_eq!(
        StyledText::plain("test").on_color256(236).to_string(),
        "\x1b[48;5;236mtest\x1b[0m"
    );
}

#[test]
fn test_styled_text_inherent_bright_black_methods() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!(
        StyledText::plain("test").bright_black().to_string(),
        "\x1b[90mtest\x1b[0m"
    );
    assert_eq!(
        StyledText::plain("test").on_bright_black().to_string(),
        "\x1b[100mtest\x1b[0m"
    );
}

#[test]
fn test_named_colors_remain_named_for_color_levels() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);

    for level in [
        ColorLevel::Ansi16,
        ColorLevel::Ansi256,
        ColorLevel::TrueColor,
    ] {
        let target = RenderTarget::Capabilities(TerminalCapabilities {
            is_terminal: true,
            color_level: level,
        });
        assert_eq!("test".red().render(target), "\x1b[31mtest\x1b[0m");
    }
}

#[test]
fn test_no_color_suppresses_styles_and_raw_codes() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: false,
        color_level: ColorLevel::NoColor,
    });

    assert_eq!("test".bold().red().colorize("4").render(target), "test");
}

#[rstest(index, expected)]
#[case(0, (0, 0, 0))]
#[case(15, (255, 255, 255))]
#[case(16, (0, 0, 0))]
#[case(21, (0, 0, 255))]
#[case(208, (255, 135, 0))]
#[case(232, (8, 8, 8))]
#[case(255, (238, 238, 238))]
fn test_ansi256_palette_rgb_values(index: u8, expected: (u8, u8, u8)) {
    assert_eq!(ansi256_to_rgb(index), expected);
}

#[rstest(rgb, expected)]
#[case((0, 0, 0), 0)]
#[case((255, 128, 0), 208)]
#[case((0, 0, 255), 12)]
#[case((128, 128, 128), 8)]
#[case((238, 238, 238), 255)]
fn test_rgb_to_ansi256_known_values(rgb: (u8, u8, u8), expected: u8) {
    assert_eq!(rgb_to_ansi256(rgb.0, rgb.1, rgb.2), expected);
}

#[rstest(rgb, expected)]
#[case((255, 0, 0), AnsiColor::BrightRed)]
#[case((0, 0, 128), AnsiColor::Blue)]
#[case((128, 128, 128), AnsiColor::BrightBlack)]
#[case((255, 255, 255), AnsiColor::BrightWhite)]
#[case((255, 128, 0), AnsiColor::Yellow)]
fn test_rgb_to_named_color_known_values(rgb: (u8, u8, u8), expected: AnsiColor) {
    assert_eq!(rgb_to_named_color(rgb.0, rgb.1, rgb.2), expected);
}

#[test]
fn test_rgb_to_named_color_ties_use_palette_order() {
    assert_eq!(rgb_to_named_color(64, 64, 64), AnsiColor::Black);
}

#[rstest(index, expected)]
#[case(8, AnsiColor::BrightBlack)]
#[case(12, AnsiColor::BrightBlue)]
#[case(208, AnsiColor::BrightYellow)]
#[case(236, AnsiColor::Black)]
fn test_ansi256_to_named_color_known_values(index: u8, expected: AnsiColor) {
    assert_eq!(ansi256_to_named_color(index), expected);
}

#[rstest(color, expected)]
#[case(AnsiColor::BrightBlack, "100")]
#[case(AnsiColor::BrightRed, "101")]
#[case(AnsiColor::BrightGreen, "102")]
#[case(AnsiColor::BrightYellow, "103")]
#[case(AnsiColor::BrightBlue, "104")]
#[case(AnsiColor::BrightMagenta, "105")]
#[case(AnsiColor::BrightCyan, "106")]
#[case(AnsiColor::BrightWhite, "107")]
fn test_bright_background_color_codes(color: AnsiColor, expected: &str) {
    assert_eq!(
        ColorSpec::Named(color).background_code(ColorLevel::Ansi16),
        Some(expected.to_string())
    );
}

#[test]
fn test_ansi256_color_codes() {
    assert_eq!(
        ColorSpec::Ansi256(208).foreground_code(ColorLevel::Ansi256),
        Some("38;5;208".to_string())
    );
    assert_eq!(
        ColorSpec::Ansi256(236).background_code(ColorLevel::Ansi256),
        Some("48;5;236".to_string())
    );
}
