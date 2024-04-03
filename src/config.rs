use anstream::{eprint, eprintln};
use anstyle::{AnsiColor, Color, Reset, Style};

use crate::templating::make_handlebars_registry;

#[allow(dead_code)]
pub struct GlobalConfig {
    level: Option<log::Level>,
    handlebars: handlebars::Handlebars<'static>,
    /// Minimum rustc version supported.
    ///
    /// This will be used to print an error if the user's rustc version is not high enough.
    minimum_rustc_version: semver::Version,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalConfig {
    pub fn new() -> Self {
        Self {
            level: None,
            handlebars: make_handlebars_registry(),
            minimum_rustc_version: semver::Version::new(1, 74, 0),
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars<'static> {
        &self.handlebars
    }

    pub fn minimum_rustc_version(&self) -> &semver::Version {
        &self.minimum_rustc_version
    }

    pub fn set_level(mut self, level: Option<log::Level>) -> Self {
        self.level = level;
        self
    }

    pub fn is_info(&self) -> bool {
        self.level.is_some() && self.level.unwrap() >= log::Level::Info
    }

    pub fn is_error(&self) -> bool {
        self.level.is_some() && self.level.unwrap() >= log::Level::Error
    }

    pub fn is_verbose(&self) -> bool {
        self.level.is_some() && self.level.unwrap() >= log::Level::Debug
    }

    pub fn is_extra_verbose(&self) -> bool {
        self.level.is_some() && self.level.unwrap() >= log::Level::Trace
    }

    pub fn log_verbose(
        &mut self,
        callback: impl Fn(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_verbose() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_extra_verbose(
        &mut self,
        callback: impl Fn(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_extra_verbose() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_info(
        &mut self,
        callback: impl Fn(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_info() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_error(
        &mut self,
        callback: impl Fn(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_error() {
            callback(self)?;
        }
        Ok(())
    }

    /// Print a message with a colored title in the style of Cargo shell messages.
    pub fn shell_print(
        &mut self,
        status: impl std::fmt::Display,
        message: impl std::fmt::Display,
        color: anstyle::Color,
        justified: bool,
    ) -> anyhow::Result<()> {
        if self.is_info() {
            eprint!("{}", Style::new().fg_color(Some(color)).bold());
            if justified {
                eprint!("{status:>12}");
            } else {
                eprint!("{status}{}{}:", Reset, Style::new().bold());
            }

            eprintln!("{} {message}", Reset);
        }

        Ok(())
    }

    /// Print a styled action message.
    pub fn shell_status(
        &mut self,
        action: impl std::fmt::Display,
        message: impl std::fmt::Display,
    ) -> anyhow::Result<()> {
        self.shell_print(action, message, Color::Ansi(AnsiColor::Green), true)
    }

    pub fn shell_note(&mut self, message: impl std::fmt::Display) -> anyhow::Result<()> {
        self.shell_print("note", message, Color::Ansi(AnsiColor::Cyan), false)
    }

    pub fn shell_warn(&mut self, message: impl std::fmt::Display) -> anyhow::Result<()> {
        self.shell_print("warning", message, Color::Ansi(AnsiColor::Yellow), false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_log_level_info() {
        let mut config = GlobalConfig::new();
        config = config.set_level(Some(log::Level::Info));

        assert!(config.is_info());
        assert!(!config.is_verbose());
        assert!(!config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_debug() {
        let mut config = GlobalConfig::new();
        config = config.set_level(Some(log::Level::Debug));

        assert!(config.is_info());
        assert!(config.is_verbose());
        assert!(!config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_trace() {
        let mut config = GlobalConfig::new();
        config = config.set_level(Some(log::Level::Trace));

        assert!(config.is_info());
        assert!(config.is_verbose());
        assert!(config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_none() {
        let mut config = GlobalConfig::new();
        config = config.set_level(None);

        assert!(!config.is_info());
        assert!(!config.is_verbose());
        assert!(!config.is_extra_verbose());
    }
}
