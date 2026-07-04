use crate::color::{
    ansi256_to_named_color, ansi256_to_rgb, rgb_to_ansi256, rgb_to_named_color, ColorSpec,
    NamedColor,
};
use crate::config::{
    get_stderr_terminal_override_for_tests, get_terminal_override_for_tests,
    set_stderr_terminal_override_for_tests, set_terminal_override_for_tests,
};
use crate::terminal::{detect_color_level, tests::TestEnv};
use crate::*;
use rstest::*;
use std::env;
use std::ffi::OsString;
use std::io::IsTerminal;
use std::sync::{LazyLock, Mutex, MutexGuard};

static TEST_LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));
const COLOR_ENV_KEYS: [&str; 10] = [
    "NO_COLOR",
    "FORCE_COLOR",
    "CLICOLOR",
    "CLICOLOR_FORCE",
    "TERM",
    "COLORTERM",
    "CI",
    "WT_SESSION",
    "ConEmuANSI",
    "ANSICON",
];

struct TestStateGuard {
    _lock: MutexGuard<'static, ()>,
    previous_mode: ColorMode,
    previous_depth_mode: ColorDepthMode,
    previous_env: Vec<(&'static str, Option<OsString>)>,
    previous_terminal_override: Option<bool>,
    previous_stderr_terminal_override: Option<bool>,
}

impl TestStateGuard {
    fn colors_enabled(mode: ColorMode) -> Self {
        Self::with_depth(mode, ColorDepthMode::TrueColor, None, Some(false))
    }

    fn no_color(mode: ColorMode) -> Self {
        Self::with_depth(mode, ColorDepthMode::Auto, Some("1"), Some(false))
    }

    fn auto_terminal(is_terminal: bool) -> Self {
        Self::with_depth(
            ColorMode::Auto,
            ColorDepthMode::Auto,
            None,
            Some(is_terminal),
        )
    }

    fn with_state(
        mode: ColorMode,
        no_color: Option<&str>,
        terminal_override: Option<bool>,
    ) -> Self {
        Self::with_depth(mode, ColorDepthMode::Auto, no_color, terminal_override)
    }

    fn with_depth(
        mode: ColorMode,
        depth_mode: ColorDepthMode,
        no_color: Option<&str>,
        terminal_override: Option<bool>,
    ) -> Self {
        let guard = TEST_LOCK
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());
        let previous_mode = ColorizeConfig::color_mode();
        let previous_depth_mode = ColorizeConfig::color_depth_mode();
        let previous_env = COLOR_ENV_KEYS
            .into_iter()
            .map(|key| (key, env::var_os(key)))
            .collect();
        let previous_terminal_override = get_terminal_override_for_tests();
        let previous_stderr_terminal_override = get_stderr_terminal_override_for_tests();

        for key in COLOR_ENV_KEYS {
            env::remove_var(key);
        }

        match no_color {
            Some(value) => env::set_var("NO_COLOR", value),
            None => env::remove_var("NO_COLOR"),
        }
        env::set_var("TERM", "xterm-256color");
        ColorizeConfig::set_color_mode(mode);
        ColorizeConfig::set_color_depth_mode(depth_mode);
        set_terminal_override_for_tests(terminal_override);
        set_stderr_terminal_override_for_tests(None);

        Self {
            _lock: guard,
            previous_mode,
            previous_depth_mode,
            previous_env,
            previous_terminal_override,
            previous_stderr_terminal_override,
        }
    }
}

impl Drop for TestStateGuard {
    fn drop(&mut self) {
        ColorizeConfig::set_color_mode(self.previous_mode);
        ColorizeConfig::set_color_depth_mode(self.previous_depth_mode);
        set_terminal_override_for_tests(self.previous_terminal_override);
        set_stderr_terminal_override_for_tests(self.previous_stderr_terminal_override);
        for (key, value) in &self.previous_env {
            match value {
                Some(value) => env::set_var(key, value),
                None => env::remove_var(key),
            }
        }
    }
}

#[rstest]
#[case("red", "\x1b[31mtest\x1b[0m")]
#[case("green", "\x1b[32mtest\x1b[0m")]
#[case("yellow", "\x1b[33mtest\x1b[0m")]
#[case("blue", "\x1b[34mtest\x1b[0m")]
#[case("magenta", "\x1b[35mtest\x1b[0m")]
#[case("cyan", "\x1b[36mtest\x1b[0m")]
#[case("white", "\x1b[37mtest\x1b[0m")]
#[case("black", "\x1b[30mtest\x1b[0m")]
fn test_basic_colors(#[case] color: &str, #[case] expected: &str) {
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

#[rstest]
#[case("bright_red", "\x1b[91mtest\x1b[0m")]
#[case("bright_green", "\x1b[92mtest\x1b[0m")]
#[case("bright_yellow", "\x1b[93mtest\x1b[0m")]
#[case("bright_blue", "\x1b[94mtest\x1b[0m")]
#[case("bright_magenta", "\x1b[95mtest\x1b[0m")]
#[case("bright_cyan", "\x1b[96mtest\x1b[0m")]
#[case("bright_white", "\x1b[97mtest\x1b[0m")]
fn test_bright_colors(#[case] color: &str, #[case] expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let text = "test";
    let actual = match color {
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

#[rstest]
#[case("on_red", "\x1b[41mtest\x1b[0m")]
#[case("on_green", "\x1b[42mtest\x1b[0m")]
#[case("on_yellow", "\x1b[43mtest\x1b[0m")]
#[case("on_blue", "\x1b[44mtest\x1b[0m")]
#[case("on_magenta", "\x1b[45mtest\x1b[0m")]
#[case("on_cyan", "\x1b[46mtest\x1b[0m")]
#[case("on_white", "\x1b[47mtest\x1b[0m")]
#[case("on_black", "\x1b[40mtest\x1b[0m")]
fn test_background_colors(#[case] color: &str, #[case] expected: &str) {
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

#[rstest]
#[case("bold", "\x1b[1mtest\x1b[0m")]
#[case("dim", "\x1b[2mtest\x1b[0m")]
#[case("italic", "\x1b[3mtest\x1b[0m")]
#[case("underline", "\x1b[4mtest\x1b[0m")]
#[case("inverse", "\x1b[7mtest\x1b[0m")]
#[case("strikethrough", "\x1b[9mtest\x1b[0m")]
fn test_styles(#[case] style: &str, #[case] expected: &str) {
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

#[rstest]
#[case(255, 128, 0)]
#[case(0, 255, 0)]
#[case(128, 128, 128)]
#[case(0, 0, 0)]
#[case(255, 255, 255)]
fn test_rgb_colors(#[case] r: u8, #[case] g: u8, #[case] b: u8) {
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

#[rstest]
#[case(0, "\x1b[38;5;0mtest\x1b[0m")]
#[case(255, "\x1b[38;5;255mtest\x1b[0m")]
fn test_ansi256_foreground_colors(#[case] index: u8, #[case] expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".ansi256(index).to_string(), expected);
}

#[rstest]
#[case(0, "\x1b[48;5;0mtest\x1b[0m")]
#[case(255, "\x1b[48;5;255mtest\x1b[0m")]
fn test_ansi256_background_colors(#[case] index: u8, #[case] expected: &str) {
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

#[rstest]
#[case("#ff8000", 255, 128, 0)]
#[case("#f80", 255, 136, 0)]
#[case("#00ff00", 0, 255, 0)]
#[case("0f8", 0, 255, 136)]
#[case("#808080", 128, 128, 128)]
#[case("#000000", 0, 0, 0)]
#[case("#ffffff", 255, 255, 255)]
fn test_hex_colors(#[case] hex: &str, #[case] r: u8, #[case] g: u8, #[case] b: u8) {
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

#[rstest]
#[case("invalid")]
#[case("#12")]
#[case("#1234")]
#[case("#12345678")]
#[case("not-a-color")]
#[case("#12345")]
#[case("#1234567")]
#[case("#xyz")]
fn test_invalid_hex_returns_plain_text(#[case] hex: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let text = "test";
    assert_eq!(text.hex(hex).to_string(), "test");
    assert_eq!(text.on_hex(hex).to_string(), "test");
    assert_eq!(text.red().hex(hex).to_string(), "test");
    assert_eq!(text.on_blue().on_hex(hex).to_string(), "test");
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

fn assert_rgb_approx_eq(actual: &str, expected: &str) {
    let extract_rgb = |s: &str| {
        let start = s.find("38;2;").or_else(|| s.find("48;2;"));
        if let Some(start) = start {
            let sequence = &s[start..];
            let parts: Vec<&str> = sequence.split(';').collect();
            let r = parts.get(2).and_then(|part| part.parse::<i32>().ok());
            let g = parts.get(3).and_then(|part| part.parse::<i32>().ok());
            let b = parts
                .get(4)
                .and_then(|part| part.split('m').next())
                .and_then(|part| part.parse::<i32>().ok());

            if let (Some(r), Some(g), Some(b)) = (r, g, b) {
                return (r, g, b);
            }
        }

        panic!("Invalid ANSI color sequence");
    };

    let (r1, g1, b1) = extract_rgb(actual);
    let (r2, g2, b2) = extract_rgb(expected);

    assert!(
        (r1 - r2).abs() <= 1 && (g1 - g2).abs() <= 1 && (b1 - b2).abs() <= 1,
        "RGB values differ by more than 1: ({}, {}, {}) vs ({}, {}, {})",
        r1,
        g1,
        b1,
        r2,
        g2,
        b2
    );
}

#[rstest]
#[case(0.0, 100.0, 50.0, 255, 0, 0)]
#[case(60.0, 100.0, 50.0, 255, 255, 0)]
#[case(90.0, 100.0, 50.0, 128, 255, 0)]
#[case(120.0, 100.0, 50.0, 0, 255, 0)]
#[case(150.0, 100.0, 50.0, 0, 255, 128)]
#[case(180.0, 100.0, 50.0, 0, 255, 255)]
#[case(210.0, 100.0, 50.0, 0, 128, 255)]
#[case(240.0, 100.0, 50.0, 0, 0, 255)]
#[case(300.0, 100.0, 50.0, 255, 0, 255)]
#[case(330.0, 100.0, 50.0, 255, 0, 128)]
#[case(360.0, 100.0, 50.0, 255, 0, 0)]
fn test_hsl_colors_comprehensive(
    #[case] h: f32,
    #[case] s: f32,
    #[case] l: f32,
    #[case] r: u8,
    #[case] g: u8,
    #[case] b: u8,
) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let actual = "test".hsl(h, s, l).to_string();
    let expected = "test".rgb(r, g, b).to_string();
    assert_rgb_approx_eq(&actual, &expected);
}

#[test]
fn test_hsl_edge_cases() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);

    let assert_hsl_rgb = |h, s, l, r, g, b| {
        let actual = "test".hsl(h, s, l).to_string();
        let expected = "test".rgb(r, g, b).to_string();
        assert_rgb_approx_eq(&actual, &expected);
    };

    assert_hsl_rgb(0.0, 0.0, 0.0, 0, 0, 0);
    assert_hsl_rgb(0.0, 0.0, 25.0, 64, 64, 64);
    assert_hsl_rgb(0.0, 0.0, 50.0, 128, 128, 128);
    assert_hsl_rgb(0.0, 0.0, 75.0, 191, 191, 191);
    assert_hsl_rgb(0.0, 0.0, 100.0, 255, 255, 255);

    assert_hsl_rgb(0.0, 25.0, 50.0, 159, 96, 96);
    assert_hsl_rgb(0.0, 50.0, 50.0, 191, 64, 64);
    assert_hsl_rgb(0.0, 75.0, 50.0, 223, 32, 32);

    assert_hsl_rgb(120.0, 100.0, 25.0, 0, 128, 0);
    assert_hsl_rgb(120.0, 100.0, 75.0, 128, 255, 128);
}

#[test]
fn test_hsl_background_colors() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let actual = "test".on_hsl(0.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(255, 0, 0).to_string();
    assert_rgb_approx_eq(&actual, &expected);

    let actual = "test".on_hsl(120.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(0, 255, 0).to_string();
    assert_rgb_approx_eq(&actual, &expected);

    let actual = "test".on_hsl(240.0, 100.0, 50.0).to_string();
    let expected = "test".on_rgb(0, 0, 255).to_string();
    assert_rgb_approx_eq(&actual, &expected);
}

#[test]
fn test_color_mode_always_forces_color() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!("test".red().to_string(), "\x1b[31mtest\x1b[0m");
}

#[test]
fn test_color_mode_auto_respects_tty_detection() {
    let _guard = TestStateGuard::auto_terminal(false);
    assert_eq!("test".red().to_string(), "test");
}

#[test]
fn test_color_mode_auto_uses_real_stdout_terminal_state_without_override() {
    let _guard = TestStateGuard::with_state(ColorMode::Auto, None, None);
    let expected = if std::io::stdout().is_terminal() {
        ColorLevel::Ansi256
    } else {
        ColorLevel::NoColor
    };
    assert_eq!(ColorizeConfig::color_level(RenderTarget::Stdout), expected);
}

#[test]
fn test_render_auto_uses_real_stderr_terminal_state_without_override() {
    let _guard = TestStateGuard::with_state(ColorMode::Auto, None, None);
    set_stderr_terminal_override_for_tests(None);

    let expected = if std::io::stderr().is_terminal() {
        "\x1b[31mtest\x1b[0m"
    } else {
        "test"
    };

    assert_eq!("test".red().render(RenderTarget::Stderr), expected);
}

#[test]
fn test_color_mode_auto_enables_color_for_terminal_output() {
    let _guard = TestStateGuard::auto_terminal(true);
    assert_eq!("test".red().to_string(), "\x1b[31mtest\x1b[0m");
}

#[test]
fn test_render_auto_uses_stderr_terminal_state() {
    let _guard = TestStateGuard::with_state(ColorMode::Auto, None, Some(false));
    set_stderr_terminal_override_for_tests(Some(true));
    assert_eq!(
        "test".red().render(RenderTarget::Stderr),
        "\x1b[31mtest\x1b[0m"
    );
}

#[test]
fn test_render_auto_uses_custom_terminal_state() {
    let _guard = TestStateGuard::with_state(ColorMode::Auto, None, Some(true));
    assert_eq!("test".red().render(RenderTarget::Terminal(false)), "test");
    assert_eq!(
        "test".red().render(RenderTarget::Terminal(true)),
        "\x1b[31mtest\x1b[0m"
    );
}

#[test]
fn test_render_always_ignores_target_terminal_state() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    assert_eq!(
        "test".red().render(RenderTarget::Terminal(false)),
        "\x1b[31mtest\x1b[0m"
    );
}

#[test]
fn test_render_never_disables_color_for_all_targets() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
    assert_eq!("test".red().render(RenderTarget::Stdout), "test");
    assert_eq!("test".red().render(RenderTarget::Stderr), "test");
    assert_eq!("test".red().render(RenderTarget::Terminal(true)), "test");
}

#[test]
fn test_render_respects_no_color_in_always_mode() {
    let _guard = TestStateGuard::no_color(ColorMode::Always);
    assert_eq!("test".red().render(RenderTarget::Terminal(true)), "test");
}

#[test]
fn test_display_remains_stdout_based_in_auto_mode() {
    let _guard = TestStateGuard::with_state(ColorMode::Auto, None, Some(false));
    set_stderr_terminal_override_for_tests(Some(true));
    assert_eq!("test".red().to_string(), "test");
}

#[test]
fn test_color_mode_never_disables_color() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
    assert_eq!("test".red().to_string(), "test");
    assert_eq!("test".blue().italic().on_yellow().to_string(), "test");
    assert_eq!("test".ansi256(208).to_string(), "test");
}

#[test]
fn test_no_color_disables_output_in_auto_and_always() {
    let _guard = TestStateGuard::no_color(ColorMode::Always);
    assert_eq!("test".red().to_string(), "test");
    assert_eq!("test".blue().italic().on_yellow().to_string(), "test");
    assert_eq!("test".ansi256(208).to_string(), "test");
}

#[rstest]
#[case(
    ColorMode::Never,
    ColorDepthMode::Auto,
    true,
    TestEnv::default(),
    ColorLevel::NoColor
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::NoColor,
    true,
    TestEnv::default(),
    ColorLevel::NoColor
)]
#[case(ColorMode::Auto, ColorDepthMode::Auto, true, TestEnv::default().with("FORCE_COLOR", "0"), ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::Auto, true, TestEnv::default().with("FORCE_COLOR", "false"), ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi16, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::Ansi16)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi256, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::Ansi256)]
#[case(ColorMode::Auto, ColorDepthMode::TrueColor, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::TrueColor)]
#[case(ColorMode::Never, ColorDepthMode::TrueColor, true, TestEnv::default().with("FORCE_COLOR", "3"), ColorLevel::NoColor)]
fn test_color_level_precedence(
    #[case] color_mode: ColorMode,
    #[case] depth_mode: ColorDepthMode,
    #[case] is_terminal: bool,
    #[case] env: TestEnv,
    #[case] expected: ColorLevel,
) {
    assert_eq!(
        detect_color_level(is_terminal, color_mode, depth_mode, &env),
        expected
    );
}

#[rstest]
#[case("1", ColorLevel::Ansi16)]
#[case("2", ColorLevel::Ansi256)]
#[case("3", ColorLevel::TrueColor)]
#[case("truecolor", ColorLevel::TrueColor)]
#[case("ansi256", ColorLevel::Ansi256)]
#[case("ansi16", ColorLevel::Ansi16)]
#[case("on", ColorLevel::Ansi16)]
fn test_force_color_values(#[case] value: &str, #[case] expected: ColorLevel) {
    let env = TestEnv::default().with("FORCE_COLOR", value);
    assert_eq!(
        detect_color_level(false, ColorMode::Auto, ColorDepthMode::Auto, &env),
        expected
    );
}

#[rstest]
#[case(TestEnv::default().with("NO_COLOR", "1"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("CLICOLOR", "0"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("CLICOLOR_FORCE", "1"), false, ColorLevel::Ansi16)]
#[case(TestEnv::default(), false, ColorLevel::NoColor)]
#[case(TestEnv::default().with("TERM", "dumb"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("COLORTERM", "truecolor"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("COLORTERM", "24bit"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("TERM", "xterm-256color"), true, ColorLevel::Ansi256)]
#[case(TestEnv::default().with("TERM", "xterm"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("WT_SESSION", "abc"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("ConEmuANSI", "ON"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("ANSICON", "1"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("CI", "1"), true, ColorLevel::Ansi16)]
fn test_auto_color_level_detection(
    #[case] env: TestEnv,
    #[case] is_terminal: bool,
    #[case] expected: ColorLevel,
) {
    assert_eq!(
        detect_color_level(is_terminal, ColorMode::Auto, ColorDepthMode::Auto, &env),
        expected
    );
}

#[test]
fn test_clicolor_force_can_still_detect_higher_depth() {
    let env = TestEnv::default()
        .with("CLICOLOR_FORCE", "1")
        .with("COLORTERM", "truecolor");

    assert_eq!(
        detect_color_level(false, ColorMode::Auto, ColorDepthMode::Auto, &env),
        ColorLevel::TrueColor
    );
}

#[test]
fn test_color_depth_mode_accessors() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    ColorizeConfig::set_color_depth_mode(ColorDepthMode::Ansi256);
    assert_eq!(ColorizeConfig::color_depth_mode(), ColorDepthMode::Ansi256);
}

#[test]
fn test_terminal_capabilities_for_known_target() {
    let _guard =
        TestStateGuard::with_depth(ColorMode::Auto, ColorDepthMode::Auto, None, Some(false));
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: false,
        color_level: ColorLevel::Ansi256,
    });

    assert_eq!(ColorizeConfig::color_level(target), ColorLevel::Ansi256);
    assert_eq!(
        "test".rgb(255, 128, 0).render(target),
        "\x1b[38;5;208mtest\x1b[0m"
    );
}

#[test]
fn test_terminal_capabilities_respect_color_mode_never() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: ColorLevel::TrueColor,
    });

    assert_eq!(ColorizeConfig::color_level(target), ColorLevel::NoColor);
    assert_eq!("test".red().render(target), "test");
}

#[test]
fn test_terminal_true_uses_env_depth_detection() {
    let _guard =
        TestStateGuard::with_depth(ColorMode::Auto, ColorDepthMode::Auto, None, Some(false));
    env::set_var("COLORTERM", "truecolor");

    assert_eq!(
        ColorizeConfig::color_level(RenderTarget::Terminal(true)),
        ColorLevel::TrueColor
    );
}

#[rstest]
#[case(ColorLevel::TrueColor, "\x1b[38;2;255;128;0mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[33mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_rgb_degrades_by_color_level(#[case] level: ColorLevel, #[case] expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".rgb(255, 128, 0).render(target), expected);
}

#[rstest]
#[case(ColorLevel::TrueColor, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi256, "\x1b[38;5;208mtest\x1b[0m")]
#[case(ColorLevel::Ansi16, "\x1b[93mtest\x1b[0m")]
#[case(ColorLevel::NoColor, "test")]
fn test_ansi256_degrades_by_color_level(#[case] level: ColorLevel, #[case] expected: &str) {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: level,
    });

    assert_eq!("test".ansi256(208).render(target), expected);
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

#[rstest]
#[case(0, (0, 0, 0))]
#[case(15, (255, 255, 255))]
#[case(16, (0, 0, 0))]
#[case(21, (0, 0, 255))]
#[case(208, (255, 135, 0))]
#[case(232, (8, 8, 8))]
#[case(255, (238, 238, 238))]
fn test_ansi256_palette_rgb_values(#[case] index: u8, #[case] expected: (u8, u8, u8)) {
    assert_eq!(ansi256_to_rgb(index), expected);
}

#[rstest]
#[case((255, 128, 0), 208)]
#[case((0, 0, 255), 12)]
#[case((128, 128, 128), 8)]
#[case((238, 238, 238), 255)]
fn test_rgb_to_ansi256_known_values(#[case] rgb: (u8, u8, u8), #[case] expected: u8) {
    assert_eq!(rgb_to_ansi256(rgb.0, rgb.1, rgb.2), expected);
}

#[rstest]
#[case((255, 0, 0), NamedColor::BrightRed)]
#[case((0, 0, 128), NamedColor::Blue)]
#[case((255, 255, 255), NamedColor::BrightWhite)]
#[case((255, 128, 0), NamedColor::Yellow)]
fn test_rgb_to_named_color_known_values(#[case] rgb: (u8, u8, u8), #[case] expected: NamedColor) {
    assert_eq!(rgb_to_named_color(rgb.0, rgb.1, rgb.2), expected);
}

#[rstest]
#[case(12, NamedColor::BrightBlue)]
#[case(208, NamedColor::BrightYellow)]
#[case(236, NamedColor::Black)]
fn test_ansi256_to_named_color_known_values(#[case] index: u8, #[case] expected: NamedColor) {
    assert_eq!(ansi256_to_named_color(index), expected);
}

#[test]
#[allow(deprecated)]
fn test_set_terminal_check_compatibility_mapping() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
    ColorizeConfig::set_terminal_check(false);
    assert_eq!(ColorizeConfig::color_mode(), ColorMode::Always);

    ColorizeConfig::set_terminal_check(true);
    assert_eq!(ColorizeConfig::color_mode(), ColorMode::Auto);
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

#[rstest]
#[case(NamedColor::BrightRed, "101")]
#[case(NamedColor::BrightGreen, "102")]
#[case(NamedColor::BrightYellow, "103")]
#[case(NamedColor::BrightBlue, "104")]
#[case(NamedColor::BrightMagenta, "105")]
#[case(NamedColor::BrightCyan, "106")]
#[case(NamedColor::BrightWhite, "107")]
fn test_bright_background_color_codes(#[case] color: NamedColor, #[case] expected: &str) {
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

#[test]
fn test_from_styled_text_to_string() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let rendered: String = "test".red().bold().into();
    assert_eq!(rendered, "\x1b[1;31mtest\x1b[0m");
}

#[test]
#[should_panic(expected = "Invalid ANSI color sequence")]
fn test_assert_rgb_approx_eq_invalid_sequence() {
    assert_rgb_approx_eq("invalid", "also invalid");
}

#[test]
#[should_panic(expected = "RGB values differ by more than 1: (255, 0, 0) vs (252, 0, 0)")]
fn test_assert_rgb_approx_eq_large_diff() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Always);
    let color1 = "test".rgb(255, 0, 0).to_string();
    let color2 = "test".rgb(252, 0, 0).to_string();
    assert_rgb_approx_eq(&color1, &color2);
}
