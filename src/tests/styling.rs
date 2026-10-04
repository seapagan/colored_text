use super::TestStateGuard;
use crate::*;
use rstest::rstest;

#[rstest(color, expected)]
#[case("red", "\x1b[31mtest\x1b[0m")]
#[case("green", "\x1b[32mtest\x1b[0m")]
#[case("yellow", "\x1b[33mtest\x1b[0m")]
#[case("blue", "\x1b[34mtest\x1b[0m")]
#[case("magenta", "\x1b[35mtest\x1b[0m")]
#[case("cyan", "\x1b[36mtest\x1b[0m")]
#[case("white", "\x1b[37mtest\x1b[0m")]
#[case("black", "\x1b[30mtest\x1b[0m")]
fn test_basic_colors(color: &str, expected: &str) {
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

#[rstest(color, expected)]
#[case("bright_black", "\x1b[90mtest\x1b[0m")]
#[case("bright_red", "\x1b[91mtest\x1b[0m")]
#[case("bright_green", "\x1b[92mtest\x1b[0m")]
#[case("bright_yellow", "\x1b[93mtest\x1b[0m")]
#[case("bright_blue", "\x1b[94mtest\x1b[0m")]
#[case("bright_magenta", "\x1b[95mtest\x1b[0m")]
#[case("bright_cyan", "\x1b[96mtest\x1b[0m")]
#[case("bright_white", "\x1b[97mtest\x1b[0m")]
fn test_bright_colors(color: &str, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let text = "test";
    let actual = match color {
        "bright_black" => text.bright_black().to_string(),
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

#[rstest(color, expected)]
#[case("on_red", "\x1b[41mtest\x1b[0m")]
#[case("on_green", "\x1b[42mtest\x1b[0m")]
#[case("on_yellow", "\x1b[43mtest\x1b[0m")]
#[case("on_blue", "\x1b[44mtest\x1b[0m")]
#[case("on_magenta", "\x1b[45mtest\x1b[0m")]
#[case("on_cyan", "\x1b[46mtest\x1b[0m")]
#[case("on_white", "\x1b[47mtest\x1b[0m")]
#[case("on_black", "\x1b[40mtest\x1b[0m")]
fn test_background_colors(color: &str, expected: &str) {
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

#[rstest(color, expected)]
#[case("on_bright_black", "\x1b[100mtest\x1b[0m")]
#[case("on_bright_red", "\x1b[101mtest\x1b[0m")]
#[case("on_bright_green", "\x1b[102mtest\x1b[0m")]
#[case("on_bright_yellow", "\x1b[103mtest\x1b[0m")]
#[case("on_bright_blue", "\x1b[104mtest\x1b[0m")]
#[case("on_bright_magenta", "\x1b[105mtest\x1b[0m")]
#[case("on_bright_cyan", "\x1b[106mtest\x1b[0m")]
#[case("on_bright_white", "\x1b[107mtest\x1b[0m")]
fn test_bright_background_colors(color: &str, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let text = "test";
    let actual = match color {
        "on_bright_black" => text.on_bright_black().to_string(),
        "on_bright_red" => text.on_bright_red().to_string(),
        "on_bright_green" => text.on_bright_green().to_string(),
        "on_bright_yellow" => text.on_bright_yellow().to_string(),
        "on_bright_blue" => text.on_bright_blue().to_string(),
        "on_bright_magenta" => text.on_bright_magenta().to_string(),
        "on_bright_cyan" => text.on_bright_cyan().to_string(),
        "on_bright_white" => text.on_bright_white().to_string(),
        _ => unreachable!(),
    };
    assert_eq!(actual, expected);
}

#[rstest(style, expected)]
#[case("bold", "\x1b[1mtest\x1b[0m")]
#[case("dim", "\x1b[2mtest\x1b[0m")]
#[case("italic", "\x1b[3mtest\x1b[0m")]
#[case("underline", "\x1b[4mtest\x1b[0m")]
#[case("inverse", "\x1b[7mtest\x1b[0m")]
#[case("strikethrough", "\x1b[9mtest\x1b[0m")]
fn test_styles(style: &str, expected: &str) {
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

#[rstest(r, g, b)]
#[case(255, 128, 0)]
#[case(0, 255, 0)]
#[case(128, 128, 128)]
#[case(0, 0, 0)]
#[case(255, 255, 255)]
fn test_rgb_colors(r: u8, g: u8, b: u8) {
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

#[rstest(index, expected)]
#[case(0, "\x1b[38;5;0mtest\x1b[0m")]
#[case(255, "\x1b[38;5;255mtest\x1b[0m")]
fn test_ansi256_foreground_colors(index: u8, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".ansi256(index).to_string(), expected);
}

#[rstest(index, expected)]
#[case(0, "\x1b[48;5;0mtest\x1b[0m")]
#[case(255, "\x1b[48;5;255mtest\x1b[0m")]
fn test_ansi256_background_colors(index: u8, expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".on_ansi256(index).to_string(), expected);
}

#[test]
fn test_color256_aliases_match_ansi256_methods() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!(
        "test".color256(208).to_string(),
        "test".ansi256(208).to_string()
    );
    assert_eq!(
        "test".on_color256(236).to_string(),
        "test".on_ansi256(236).to_string()
    );
}

#[rstest(hex, r, g, b)]
#[case("#ff8000", 255, 128, 0)]
#[case("#f80", 255, 136, 0)]
#[case("#00ff00", 0, 255, 0)]
#[case("0f8", 0, 255, 136)]
#[case("#808080", 128, 128, 128)]
#[case("#000000", 0, 0, 0)]
#[case("#ffffff", 255, 255, 255)]
fn test_hex_colors(hex: &str, r: u8, g: u8, b: u8) {
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

#[rstest(hex)]
#[case("invalid")]
#[case("#12")]
#[case("#1234")]
#[case("#12345678")]
#[case("not-a-color")]
#[case("+ab")]
#[case("+abcde")]
#[case("#+ab")]
#[case("#12345")]
#[case("#1234567")]
#[case("#xyz")]
#[case("##f80")]
#[case("##123456")]
#[case(" #f80")]
#[case("#f80 ")]
#[case("f80\n")]
#[case("aé")]
#[case("aéabc")]
#[case("aaaéa")]
#[case("aa€a")]
fn test_invalid_hex_returns_plain_text(hex: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let text = "test";
    assert_eq!(text.hex(hex).to_string(), "test");
    assert_eq!(text.on_hex(hex).to_string(), "test");
    let styled = text.colorize("4").bold().red().on_blue();
    assert_eq!(styled.clone().hex(hex).to_string(), "test");
    assert_eq!(styled.on_hex(hex).to_string(), "test");
}

#[test]
fn test_clear_returns_plain_text() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".clear().to_string(), "test");
    assert_eq!("test".red().clear().to_string(), "test");
    assert_eq!("test".ansi256(208).clear().to_string(), "test");
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
    assert_eq!(
        "test".ansi256(208).bold().on_ansi256(236).to_string(),
        "\x1b[1;38;5;208;48;5;236mtest\x1b[0m"
    );
}

#[test]
fn test_conflicting_chains_use_last_color() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".red().green().to_string(), "\x1b[32mtest\x1b[0m");
    assert_eq!("test".on_red().on_blue().to_string(), "\x1b[44mtest\x1b[0m");
    assert_eq!(
        "test".red().ansi256(208).to_string(),
        "\x1b[38;5;208mtest\x1b[0m"
    );
    assert_eq!("test".ansi256(208).red().to_string(), "\x1b[31mtest\x1b[0m");
    assert_eq!(
        "test".on_blue().on_ansi256(236).to_string(),
        "\x1b[48;5;236mtest\x1b[0m"
    );
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

#[test]
fn test_empty_text_keeps_styling_when_color_is_enabled() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("".red().to_string(), "\x1b[31m\x1b[0m");
    assert_eq!("".bold().to_string(), "\x1b[1m\x1b[0m");
    assert_eq!(
        "".rgb(255, 128, 0).to_string(),
        "\x1b[38;2;255;128;0m\x1b[0m"
    );
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
