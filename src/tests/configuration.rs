use super::TestStateGuard;
use crate::config::set_stderr_terminal_override_for_tests;
use crate::*;
use rstest::rstest;
use std::{env, io::IsTerminal};

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
fn test_terminal_capabilities_accessor_returns_known_target_capabilities() {
    let _guard =
        TestStateGuard::with_depth(ColorMode::Auto, ColorDepthMode::Auto, None, Some(false));
    let expected = TerminalCapabilities {
        is_terminal: true,
        color_level: ColorLevel::Ansi256,
    };

    assert_eq!(
        ColorizeConfig::terminal_capabilities(RenderTarget::Capabilities(expected)),
        expected
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
fn test_terminal_capabilities_respect_no_color_env() {
    let _guard = TestStateGuard::no_color(ColorMode::Always);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: ColorLevel::TrueColor,
    });

    assert_eq!(ColorizeConfig::color_level(target), ColorLevel::NoColor);
    assert_eq!("test".red().render(target), "test");
}

#[test]
fn test_terminal_capabilities_respect_color_depth_no_color() {
    let _guard =
        TestStateGuard::with_depth(ColorMode::Always, ColorDepthMode::NoColor, None, Some(true));
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: ColorLevel::TrueColor,
    });

    assert_eq!(ColorizeConfig::color_level(target), ColorLevel::NoColor);
    assert_eq!("test".red().render(target), "test");
}

#[rstest(key, value, supplied, expected)]
#[case("FORCE_COLOR", "0", ColorLevel::TrueColor, ColorLevel::TrueColor)]
#[case("CLICOLOR", "0", ColorLevel::TrueColor, ColorLevel::TrueColor)]
#[case("FORCE_COLOR", "3", ColorLevel::Ansi16, ColorLevel::Ansi16)]
fn test_terminal_capabilities_ignore_normal_target_env_overrides(
    key: &str,
    value: &str,
    supplied: ColorLevel,
    expected: ColorLevel,
) {
    let _guard =
        TestStateGuard::with_depth(ColorMode::Auto, ColorDepthMode::Auto, None, Some(false));
    env::set_var(key, value);
    let target = RenderTarget::Capabilities(TerminalCapabilities {
        is_terminal: true,
        color_level: supplied,
    });

    assert_eq!(ColorizeConfig::color_level(target), expected);
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

#[test]
#[allow(deprecated)]
fn test_set_terminal_check_compatibility_mapping() {
    let _guard = TestStateGuard::colors_enabled(ColorMode::Never);
    ColorizeConfig::set_terminal_check(false);
    assert_eq!(ColorizeConfig::color_mode(), ColorMode::Always);

    ColorizeConfig::set_terminal_check(true);
    assert_eq!(ColorizeConfig::color_mode(), ColorMode::Auto);
}
