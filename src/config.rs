use std::cell::RefCell;
use std::io::IsTerminal;

/// Runtime color policy for rendered output.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorMode {
    /// Enable styling only when stdout is a terminal.
    #[default]
    Auto,
    /// Always emit styling, even when stdout is not a terminal.
    Always,
    /// Never emit styling.
    Never,
}

/// Configuration for controlling runtime color behavior.
///
/// The active configuration is stored per thread. This makes it straightforward
/// to force a specific color mode in tests or narrow execution paths without
/// changing global process state.
#[derive(Clone, Debug)]
pub struct ColorizeConfig {
    color_mode: ColorMode,
}

thread_local! {
    static CONFIG: RefCell<ColorizeConfig> = RefCell::new(ColorizeConfig::default());
    #[cfg(test)]
    static TERMINAL_OVERRIDE: RefCell<Option<bool>> = const { RefCell::new(None) };
}

impl Default for ColorizeConfig {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::Auto,
        }
    }
}

impl ColorizeConfig {
    /// Set the runtime color policy for the current thread.
    ///
    /// In [`ColorMode::Auto`], styling is emitted only when stdout is a
    /// terminal. In [`ColorMode::Always`], styling is emitted regardless of
    /// terminal detection. In [`ColorMode::Never`], styling is disabled.
    pub fn set_color_mode(mode: ColorMode) {
        CONFIG.with(|config| config.borrow_mut().color_mode = mode);
    }

    /// Get the runtime color policy for the current thread.
    pub fn color_mode() -> ColorMode {
        CONFIG.with(|config| config.borrow().color_mode)
    }

    /// Compatibility shim for the previous API.
    ///
    /// `true` maps to [`ColorMode::Auto`], and `false` maps to
    /// [`ColorMode::Always`].
    #[deprecated(note = "use ColorizeConfig::set_color_mode(ColorMode) instead")]
    pub fn set_terminal_check(check: bool) {
        let mode = if check {
            ColorMode::Auto
        } else {
            ColorMode::Always
        };
        Self::set_color_mode(mode);
    }
}

pub(crate) fn should_colorize() -> bool {
    match ColorizeConfig::color_mode() {
        ColorMode::Never => false,
        ColorMode::Always => std::env::var_os("NO_COLOR").is_none(),
        ColorMode::Auto => std::env::var_os("NO_COLOR").is_none() && stdout_is_terminal(),
    }
}

fn stdout_is_terminal() -> bool {
    #[cfg(test)]
    if let Some(value) = TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow()) {
        return value;
    }

    std::io::stdout().is_terminal()
}

#[cfg(test)]
pub(crate) fn set_terminal_override_for_tests(value: Option<bool>) {
    TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow_mut() = value);
}
