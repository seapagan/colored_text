use std::cell::RefCell;
use std::io::IsTerminal;

use crate::terminal::{
    color_level_for_capabilities, terminal_capabilities, ColorLevel, TerminalCapabilities,
};

/// Runtime color policy for rendered output.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorMode {
    /// Enable styling only when stdout is a terminal.
    #[default]
    Auto,
    /// Always emit styling, even when stdout is not a terminal.
    ///
    /// `NO_COLOR` still takes precedence and disables styled output.
    Always,
    /// Never emit styling.
    Never,
}

/// Output target used when rendering styled text explicitly.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RenderTarget {
    /// Resolve terminal capability from stdout.
    Stdout,
    /// Resolve terminal capability from stderr.
    Stderr,
    /// Use an explicit terminal capability for a custom destination.
    Terminal(bool),
    /// Use exact, caller-provided capabilities for a custom destination.
    Capabilities(TerminalCapabilities),
}

/// Runtime color depth override for rendered output.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum ColorDepthMode {
    /// Detect color depth from the output target and environment.
    #[default]
    Auto,
    /// Disable all ANSI SGR output.
    NoColor,
    /// Force named ANSI color output.
    Ansi16,
    /// Force ANSI 256-color output.
    Ansi256,
    /// Force 24-bit RGB truecolor output.
    TrueColor,
}

/// Configuration for controlling runtime color behavior.
///
/// The active configuration is stored per thread. This makes it straightforward
/// to force a specific color mode in tests or narrow execution paths without
/// changing global process state.
#[derive(Clone, Debug)]
pub struct ColorizeConfig {
    color_mode: ColorMode,
    color_depth_mode: ColorDepthMode,
}

thread_local! {
    static CONFIG: RefCell<ColorizeConfig> = RefCell::new(ColorizeConfig::default());
    #[cfg(test)]
    #[allow(clippy::missing_const_for_thread_local)]
    static STDOUT_TERMINAL_OVERRIDE: RefCell<Option<bool>> = RefCell::new(None);
    #[cfg(test)]
    #[allow(clippy::missing_const_for_thread_local)]
    static STDERR_TERMINAL_OVERRIDE: RefCell<Option<bool>> = RefCell::new(None);
}

impl Default for ColorizeConfig {
    fn default() -> Self {
        Self {
            color_mode: ColorMode::Auto,
            color_depth_mode: ColorDepthMode::Auto,
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

    /// Set the runtime color depth override for the current thread.
    ///
    /// In [`ColorDepthMode::Auto`], color depth is detected from the target and
    /// environment. Explicit modes force a level, except [`ColorMode::Never`]
    /// still disables all ANSI output.
    pub fn set_color_depth_mode(mode: ColorDepthMode) {
        CONFIG.with(|config| config.borrow_mut().color_depth_mode = mode);
    }

    /// Get the runtime color depth override for the current thread.
    pub fn color_depth_mode() -> ColorDepthMode {
        CONFIG.with(|config| config.borrow().color_depth_mode)
    }

    /// Resolve terminal capabilities for a render target using current config.
    pub fn terminal_capabilities(target: RenderTarget) -> TerminalCapabilities {
        capabilities_for(target)
    }

    /// Resolve the color level for a render target using current config.
    pub fn color_level(target: RenderTarget) -> ColorLevel {
        capabilities_for(target).color_level
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

pub(crate) fn color_level() -> ColorLevel {
    color_level_for(RenderTarget::Stdout)
}

pub(crate) fn color_level_for(target: RenderTarget) -> ColorLevel {
    capabilities_for(target).color_level
}

fn capabilities_for(target: RenderTarget) -> TerminalCapabilities {
    let color_mode = ColorizeConfig::color_mode();
    let depth_mode = ColorizeConfig::color_depth_mode();

    match target {
        RenderTarget::Capabilities(capabilities) => TerminalCapabilities {
            color_level: color_level_for_capabilities(capabilities, color_mode, depth_mode),
            ..capabilities
        },
        _ => terminal_capabilities(target_is_terminal(target), color_mode, depth_mode),
    }
}

fn target_is_terminal(target: RenderTarget) -> bool {
    match target {
        RenderTarget::Stdout => stdout_is_terminal(),
        RenderTarget::Stderr => stderr_is_terminal(),
        RenderTarget::Terminal(value) => value,
        RenderTarget::Capabilities(capabilities) => capabilities.is_terminal,
    }
}

fn stdout_is_terminal() -> bool {
    #[cfg(test)]
    if let Some(value) = STDOUT_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow()) {
        return value;
    }

    std::io::stdout().is_terminal()
}

fn stderr_is_terminal() -> bool {
    #[cfg(test)]
    if let Some(value) = STDERR_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow()) {
        return value;
    }

    std::io::stderr().is_terminal()
}

#[cfg(test)]
pub(crate) fn set_terminal_override_for_tests(value: Option<bool>) {
    STDOUT_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow_mut() = value);
}

#[cfg(test)]
pub(crate) fn get_terminal_override_for_tests() -> Option<bool> {
    STDOUT_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow())
}

#[cfg(test)]
pub(crate) fn set_stderr_terminal_override_for_tests(value: Option<bool>) {
    STDERR_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow_mut() = value);
}

#[cfg(test)]
pub(crate) fn get_stderr_terminal_override_for_tests() -> Option<bool> {
    STDERR_TERMINAL_OVERRIDE.with(|override_value| *override_value.borrow())
}
