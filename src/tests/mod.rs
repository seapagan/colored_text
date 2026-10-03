mod colors;
mod configuration;
mod hsl;
mod rendering;
mod styling;
mod terminal_detection;

use crate::config::{
    get_stderr_terminal_override_for_tests, get_terminal_override_for_tests,
    set_stderr_terminal_override_for_tests, set_terminal_override_for_tests,
};
use crate::*;
use std::env;
use std::ffi::OsString;
use std::sync::{Mutex, MutexGuard};

static TEST_LOCK: Mutex<()> = Mutex::new(());
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
