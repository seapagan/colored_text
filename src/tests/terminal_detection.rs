use crate::terminal::{detect_color_level, tests::TestEnv};
use crate::*;
use rstest::rstest;

#[rstest(color_mode, depth_mode, is_terminal, env, expected)]
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
#[case(ColorMode::Auto, ColorDepthMode::Ansi16, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi16, true, TestEnv::default().with("NO_COLOR", ""), ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi256, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::TrueColor, true, TestEnv::default().with("NO_COLOR", "1"), ColorLevel::NoColor)]
#[case(ColorMode::Never, ColorDepthMode::TrueColor, true, TestEnv::default().with("FORCE_COLOR", "3"), ColorLevel::NoColor)]
#[case(ColorMode::Always, ColorDepthMode::TrueColor, true, TestEnv::default().with("FORCE_COLOR", "0"), ColorLevel::NoColor)]
fn test_color_level_precedence(
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    is_terminal: bool,
    env: TestEnv,
    expected: ColorLevel,
) {
    assert_eq!(
        detect_color_level(is_terminal, color_mode, depth_mode, &env),
        expected
    );
}

#[rstest(value, expected)]
#[case("1", ColorLevel::Ansi16)]
#[case("2", ColorLevel::Ansi256)]
#[case("3", ColorLevel::TrueColor)]
#[case("truecolor", ColorLevel::TrueColor)]
#[case("ansi256", ColorLevel::Ansi256)]
#[case("ansi16", ColorLevel::Ansi16)]
#[case("on", ColorLevel::Ansi16)]
fn test_force_color_values(value: &str, expected: ColorLevel) {
    let env = TestEnv::default().with("FORCE_COLOR", value);
    assert_eq!(
        detect_color_level(false, ColorMode::Auto, ColorDepthMode::Auto, &env),
        expected
    );
}

#[rstest(depth_mode, force_color, expected)]
#[case(ColorDepthMode::Ansi16, "3", ColorLevel::TrueColor)]
#[case(ColorDepthMode::Ansi256, "1", ColorLevel::Ansi16)]
#[case(ColorDepthMode::TrueColor, "2", ColorLevel::Ansi256)]
#[case(ColorDepthMode::TrueColor, "0", ColorLevel::NoColor)]
#[case(ColorDepthMode::TrueColor, "false", ColorLevel::NoColor)]
#[case(ColorDepthMode::TrueColor, "off", ColorLevel::NoColor)]
#[case(ColorDepthMode::TrueColor, "none", ColorLevel::NoColor)]
#[case(ColorDepthMode::TrueColor, "never", ColorLevel::NoColor)]
fn test_force_color_overrides_explicit_color_depth(
    depth_mode: ColorDepthMode,
    force_color: &str,
    expected: ColorLevel,
) {
    let env = TestEnv::default().with("FORCE_COLOR", force_color);
    assert_eq!(
        detect_color_level(true, ColorMode::Auto, depth_mode, &env),
        expected
    );
}

#[rstest(color_mode, depth_mode, force_color)]
#[case(ColorMode::Never, ColorDepthMode::TrueColor, "3")]
#[case(ColorMode::Auto, ColorDepthMode::NoColor, "3")]
fn test_hard_disable_modes_override_force_color(
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    force_color: &str,
) {
    let env = TestEnv::default().with("FORCE_COLOR", force_color);
    assert_eq!(
        detect_color_level(true, color_mode, depth_mode, &env),
        ColorLevel::NoColor
    );
}

#[rstest(value, is_terminal, env, expected)]
#[case("", true, TestEnv::default().with("TERM", "xterm-256color"), ColorLevel::Ansi256)]
#[case("", false, TestEnv::default(), ColorLevel::NoColor)]
#[case("sometimes", false, TestEnv::default(), ColorLevel::Ansi16)]
fn test_force_color_fallback_values(
    value: &str,
    is_terminal: bool,
    env: TestEnv,
    expected: ColorLevel,
) {
    let env = env.with("FORCE_COLOR", value);
    assert_eq!(
        detect_color_level(is_terminal, ColorMode::Auto, ColorDepthMode::Auto, &env),
        expected
    );
}

#[rstest(env, is_terminal, expected)]
#[case(TestEnv::default().with("NO_COLOR", "1"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("NO_COLOR", ""), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("CLICOLOR", "0"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("CLICOLOR_FORCE", "1"), false, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("CLICOLOR", "0").with("CLICOLOR_FORCE", "1"), false, ColorLevel::Ansi16)]
#[case(TestEnv::default(), false, ColorLevel::NoColor)]
#[case(TestEnv::default(), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("TERM", "dumb"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("TERM", "DUMB"), true, ColorLevel::NoColor)]
#[case(TestEnv::default().with("COLORTERM", "truecolor"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("COLORTERM", "24bit"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("TERM", "xterm-256color"), true, ColorLevel::Ansi256)]
#[case(TestEnv::default().with("TERM", "xterm"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("WT_SESSION", "abc"), true, ColorLevel::TrueColor)]
#[case(TestEnv::default().with("ConEmuANSI", "ON"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("ANSICON", "1"), true, ColorLevel::Ansi16)]
#[case(TestEnv::default().with("CI", "1"), true, ColorLevel::Ansi16)]
fn test_auto_color_level_detection(env: TestEnv, is_terminal: bool, expected: ColorLevel) {
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
fn test_always_mode_uses_ansi16_without_terminal_env() {
    assert_eq!(
        detect_color_level(
            false,
            ColorMode::Always,
            ColorDepthMode::Auto,
            &TestEnv::default()
        ),
        ColorLevel::Ansi16
    );
}

#[rstest(color_mode, depth_mode, is_terminal, env, expected)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    true,
    TestEnv::default().with("CLICOLOR", "0"),
    ColorLevel::NoColor
)]
#[case(
    ColorMode::Always,
    ColorDepthMode::Auto,
    false,
    TestEnv::default().with("CLICOLOR", "0"),
    ColorLevel::NoColor
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    true,
    TestEnv::default().with("CLICOLOR", "0").with("FORCE_COLOR", "2"),
    ColorLevel::Ansi256
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Auto,
    false,
    TestEnv::default().with("CLICOLOR", "0").with("CLICOLOR_FORCE", "1"),
    ColorLevel::Ansi16
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    false,
    TestEnv::default().with("CLICOLOR", "0").with("CLICOLOR_FORCE", "1"),
    ColorLevel::Ansi256
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    false,
    TestEnv::default(),
    ColorLevel::NoColor
)]
#[case(
    ColorMode::Always,
    ColorDepthMode::TrueColor,
    false,
    TestEnv::default(),
    ColorLevel::TrueColor
)]
#[case(
    ColorMode::Always,
    ColorDepthMode::Ansi256,
    false,
    TestEnv::default(),
    ColorLevel::Ansi256
)]
#[case(
    ColorMode::Always,
    ColorDepthMode::Ansi16,
    false,
    TestEnv::default(),
    ColorLevel::Ansi16
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    false,
    TestEnv::default().with("FORCE_COLOR", "2"),
    ColorLevel::Ansi256
)]
#[case(
    ColorMode::Auto,
    ColorDepthMode::Ansi256,
    false,
    TestEnv::default().with("CLICOLOR_FORCE", "1"),
    ColorLevel::Ansi256
)]
fn test_color_depth_mode_selects_depth_after_output_is_enabled(
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    is_terminal: bool,
    env: TestEnv,
    expected: ColorLevel,
) {
    assert_eq!(
        detect_color_level(is_terminal, color_mode, depth_mode, &env),
        expected
    );
}

#[rstest(color_mode, depth_mode, is_terminal, expected)]
#[case(ColorMode::Auto, ColorDepthMode::Auto, true, ColorLevel::NoColor)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi256, true, ColorLevel::Ansi256)]
#[case(ColorMode::Auto, ColorDepthMode::Ansi256, false, ColorLevel::NoColor)]
#[case(
    ColorMode::Always,
    ColorDepthMode::TrueColor,
    false,
    ColorLevel::TrueColor
)]
fn test_term_dumb_is_capability_hint_not_hard_disable(
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    is_terminal: bool,
    expected: ColorLevel,
) {
    let env = TestEnv::default().with("TERM", "dumb");
    assert_eq!(
        detect_color_level(is_terminal, color_mode, depth_mode, &env),
        expected
    );
}
