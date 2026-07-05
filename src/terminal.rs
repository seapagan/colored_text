use crate::config::{ColorDepthMode, ColorMode};

/// Resolved color support level for an output target.
///
/// Named colors and text styles render at [`ColorLevel::Ansi16`] and above.
/// ANSI 256-color values render directly at [`ColorLevel::Ansi256`] and
/// [`ColorLevel::TrueColor`], while RGB, HSL, and hex colors degrade to the
/// nearest supported color level.
#[derive(Debug, Clone, Copy, Eq, Ord, PartialEq, PartialOrd)]
pub enum ColorLevel {
    /// Do not emit ANSI SGR sequences.
    NoColor,
    /// Emit named ANSI colors and text styles.
    Ansi16,
    /// Emit ANSI 256-color sequences.
    Ansi256,
    /// Emit 24-bit RGB truecolor sequences.
    TrueColor,
}

/// Resolved terminal capability information for an output target.
///
/// Values returned by [`crate::ColorizeConfig::terminal_capabilities`] are
/// detected heuristically from the target and environment. Values supplied via
/// [`crate::RenderTarget::Capabilities`] are treated as exact caller knowledge,
/// subject only to hard color-disabling policy.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct TerminalCapabilities {
    /// Whether the target is known to be a terminal.
    pub is_terminal: bool,
    /// The color depth this target is expected to support.
    pub color_level: ColorLevel,
}

pub(crate) trait EnvProvider {
    fn get(&self, key: &str) -> Option<String>;

    fn is_set(&self, key: &str) -> bool {
        self.get(key).is_some()
    }
}

pub(crate) struct ProcessEnv;

impl EnvProvider for ProcessEnv {
    fn get(&self, key: &str) -> Option<String> {
        std::env::var(key).ok()
    }

    fn is_set(&self, key: &str) -> bool {
        std::env::var_os(key).is_some()
    }
}

pub(crate) fn terminal_capabilities(
    is_terminal: bool,
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
) -> TerminalCapabilities {
    TerminalCapabilities {
        is_terminal,
        color_level: detect_color_level(is_terminal, color_mode, depth_mode, &ProcessEnv),
    }
}

pub(crate) fn color_level_for_capabilities(
    capabilities: TerminalCapabilities,
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
) -> ColorLevel {
    if let Some(level) = hard_disable_level(color_mode, depth_mode, &ProcessEnv) {
        return level;
    }

    capabilities.color_level
}

pub(crate) fn detect_color_level(
    is_terminal: bool,
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    env: &impl EnvProvider,
) -> ColorLevel {
    if let Some(level) = hard_disable_level(color_mode, depth_mode, env) {
        return level;
    }

    if let Some(forced) = force_color_level(env) {
        return forced;
    }

    let clicolor_forced = clicolor_force_enabled(env);

    if clicolor_disabled(env) && !clicolor_forced {
        return ColorLevel::NoColor;
    }

    if color_mode == ColorMode::Auto && !is_terminal && !clicolor_forced {
        return ColorLevel::NoColor;
    }

    let mut level = match detect_env_color_level(env) {
        Some(detected) if clicolor_forced => detected.max(ColorLevel::Ansi16),
        Some(detected) => detected,
        None if clicolor_forced => ColorLevel::Ansi16,
        None => ColorLevel::TrueColor,
    };

    if let Some(explicit) = explicit_depth_level(depth_mode) {
        level = explicit;
    }

    level
}

fn hard_disable_level(
    color_mode: ColorMode,
    depth_mode: ColorDepthMode,
    env: &impl EnvProvider,
) -> Option<ColorLevel> {
    if env.is_set("NO_COLOR")
        || color_mode == ColorMode::Never
        || depth_mode == ColorDepthMode::NoColor
    {
        Some(ColorLevel::NoColor)
    } else {
        None
    }
}

fn force_color_level(env: &impl EnvProvider) -> Option<ColorLevel> {
    let value = env.get("FORCE_COLOR")?;
    let normalized = normalize_env_value(&value);

    match normalized.as_str() {
        "" => None,
        "0" | "no_color" | "none" | "never" | "false" | "off" => Some(ColorLevel::NoColor),
        "1" | "ansi" | "ansi16" | "basic" | "true" | "yes" | "on" => Some(ColorLevel::Ansi16),
        "2" | "ansi256" | "256" | "8bit" | "8-bit" => Some(ColorLevel::Ansi256),
        "3" | "truecolor" | "true_color" | "24bit" | "24-bit" | "16m" => {
            Some(ColorLevel::TrueColor)
        }
        _ => Some(ColorLevel::Ansi16),
    }
}

fn explicit_depth_level(depth_mode: ColorDepthMode) -> Option<ColorLevel> {
    match depth_mode {
        ColorDepthMode::Auto | ColorDepthMode::NoColor => None,
        ColorDepthMode::Ansi16 => Some(ColorLevel::Ansi16),
        ColorDepthMode::Ansi256 => Some(ColorLevel::Ansi256),
        ColorDepthMode::TrueColor => Some(ColorLevel::TrueColor),
    }
}

fn clicolor_force_enabled(env: &impl EnvProvider) -> bool {
    env.get("CLICOLOR_FORCE").is_some_and(|value| {
        let normalized = normalize_env_value(&value);
        !normalized.is_empty() && normalized != "0"
    })
}

fn clicolor_disabled(env: &impl EnvProvider) -> bool {
    env.get("CLICOLOR")
        .is_some_and(|value| normalize_env_value(&value) == "0")
}

fn detect_env_color_level(env: &impl EnvProvider) -> Option<ColorLevel> {
    if env
        .get("TERM")
        .is_some_and(|term| term.eq_ignore_ascii_case("dumb"))
    {
        return Some(ColorLevel::NoColor);
    }

    if env
        .get("COLORTERM")
        .is_some_and(|value| matches!(normalize_env_value(&value).as_str(), "truecolor" | "24bit"))
    {
        return Some(ColorLevel::TrueColor);
    }

    if env.is_set("WT_SESSION") {
        return Some(ColorLevel::TrueColor);
    }

    if env
        .get("TERM")
        .is_some_and(|term| term.to_ascii_lowercase().contains("256color"))
    {
        return Some(ColorLevel::Ansi256);
    }

    if env
        .get("ConEmuANSI")
        .is_some_and(|value| value.eq_ignore_ascii_case("ON"))
        || env.is_set("ANSICON")
        || env.is_set("CI")
    {
        return Some(ColorLevel::Ansi16);
    }

    if env.get("TERM").is_some_and(|term| !term.is_empty()) {
        return Some(ColorLevel::Ansi16);
    }

    None
}

fn normalize_env_value(value: &str) -> String {
    value.trim().to_ascii_lowercase()
}

#[cfg(test)]
pub(crate) mod tests {
    use std::collections::HashMap;

    use super::*;

    #[derive(Default)]
    pub(crate) struct TestEnv {
        values: HashMap<String, String>,
    }

    impl TestEnv {
        pub(crate) fn with(mut self, key: &str, value: &str) -> Self {
            self.values.insert(key.to_string(), value.to_string());
            self
        }
    }

    impl EnvProvider for TestEnv {
        fn get(&self, key: &str) -> Option<String> {
            self.values.get(key).cloned()
        }
    }
}
