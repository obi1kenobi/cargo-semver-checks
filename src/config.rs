use anstream::AutoStream;
use anstyle::{AnsiColor, Color, Reset, Style};
use std::io::Write;

use crate::templating::make_handlebars_registry;

// re-export this so users don't have to add the `anstream` crate directly
// just to set color choice
pub use anstream::ColorChoice;

#[allow(dead_code)]
pub struct GlobalConfig {
    level: Option<log::Level>,
    handlebars: handlebars::Handlebars<'static>,
    /// Minimum rustc version supported.
    ///
    /// This will be used to print an error if the user's rustc version is not high enough.
    minimum_rustc_version: semver::Version,
    stdout: AutoStream<Box<dyn Write + 'static>>,
    stderr: AutoStream<Box<dyn Write + 'static>>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalConfig {
    pub fn new() -> Self {
        let stdout_choice = anstream::stdout().current_choice();
        let stderr_choice = anstream::stdout().current_choice();

        Self {
            level: None,
            handlebars: make_handlebars_registry(),
            minimum_rustc_version: semver::Version::new(1, 74, 0),
            stdout: AutoStream::new(Box::new(std::io::stdout()), stdout_choice),
            stderr: AutoStream::new(Box::new(std::io::stderr()), stderr_choice),
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
            write!(self.stderr, "{}", Style::new().fg_color(Some(color)).bold())?;
            if justified {
                write!(self.stderr, "{status:>12}")?;
            } else {
                write!(self.stderr, "{status}{}{}:", Reset, Style::new().bold())?;
            }

            writeln!(self.stderr, "{Reset} {message}")?;
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

    /// Gets the color-supporting `stdout` that the crate will use.
    ///
    /// See [`GlobalConfig::set_stdout`] and [`GlobalConfig::set_out_color_choice`] to
    /// configure this stream
    #[must_use]
    #[inline]
    pub fn stdout(&mut self) -> impl Write + '_ {
        &mut self.stdout
    }

    /// Gets the color-supporting `stderr` that the crate will use.
    ///
    /// See [`GlobalConfig::set_stderr`] and [`GlobalConfig::set_err_color_choice`] to
    /// configure this stream
    #[must_use]
    #[inline]
    pub fn stderr(&mut self) -> impl Write + '_ {
        &mut self.stderr
    }

    /// Sets the stderr output stream
    ///
    /// Defaults to the global color choice setting in [`ColorChoice::global`].
    /// Call [`GlobalConfig::set_err_color_choice`] to customize the color choice
    pub fn set_stderr(&mut self, err: Box<dyn Write + 'static>) {
        self.stderr = AutoStream::new(err, ColorChoice::global());
    }

    /// Sets the stdout output stream
    ///
    /// Defaults to the global color choice setting in [`ColorChoice::global`].
    /// Call [`GlobalConfig::set_out_color_choice`] to customize the color choice
    pub fn set_stdout(&mut self, out: Box<dyn Write + 'static>) {
        self.stdout = AutoStream::new(out, ColorChoice::global());
    }

    /// Individually set the color choice setting for [`GlobalConfig::stderr`]
    ///
    /// Defaults to the global color choice in [`ColorChoice::global`], which can be set
    /// in [`ColorChoice::write_global`] if you are using the `anstream` crate.
    ///
    /// See also [`GlobalConfig::set_out_color_choice`] and [`GlobalConfig::set_color_choice`]
    pub fn set_err_color_choice(&mut self, choice: ColorChoice) {
        // TODO - `anstream` doesn't have a good mechanism to set color choice (on one stream)
        // without making a new object, so we have to make a new autostream, but since we need
        // to move the `RawStream` inner, we temporarily replace it with /dev/null
        let stderr = std::mem::replace(
            &mut self.stderr,
            AutoStream::never(Box::new(std::io::sink())),
        );
        self.stderr = AutoStream::new(stderr.into_inner(), choice);
    }

    /// Individually set the color choice setting for [`GlobalConfig::stdout`]
    ///
    /// Defaults to the global color choice in [`ColorChoice::global`], which can be set
    /// in [`ColorChoice::write_global`] if you are using the `anstream` crate.
    ///
    /// See also [`GlobalConfig::set_err_color_choice`] and [`GlobalConfig::set_color_choice`]
    pub fn set_out_color_choice(&mut self, choice: ColorChoice) {
        // TODO - `anstream` doesn't have a good mechanism to set color choice (on one stream)
        // without making a new object, so we have to make a new autostream, but since we need
        // to move the `RawStream` inner, we temporarily replace it with /dev/null
        let stdout = std::mem::replace(
            &mut self.stdout,
            AutoStream::never(Box::new(std::io::sink())),
        );
        self.stdout = AutoStream::new(stdout.into_inner(), choice);
    }

    /// Sets the color choice for both [`GlobalConfig::stderr`] and [`GlobalConfig::stdout`]
    ///
    /// Defaults to the global color choice in [`ColorChoice::global`], which can be set
    /// in [`ColorChoice::write_global`] if you are using the `anstream` crate.
    ///
    /// Prefer to use [`ColorChoice::write_global`] to avoid creating new stream objects if you
    /// don't need to configure `cargo-semver-checks` colors differently than other crates
    /// that use `anstream` for outputting colors.
    ///
    /// See also [`GlobalConfig::set_err_color_choice`] and [`GlobalConfig::set_out_color_choice`]
    pub fn set_color_choice(&mut self, choice: ColorChoice) {
        self.set_err_color_choice(choice);
        self.set_out_color_choice(choice);
    }
}

#[cfg(test)]
mod tests {
    use std::{
        io::{Cursor, Read, Seek},
        rc::Rc,
        sync::Mutex,
    };

    use super::*;
    use ColorChoice::*;

    /// helper struct to implement `Write + 'static` while keeping
    /// view access to an underlying buffer
    ///
    /// Uses [`Mutex::try_lock`] so no calls should block even though it is a mutex
    #[derive(Debug, Clone, Default)]
    struct SharedBuffer(Rc<Mutex<Cursor<Vec<u8>>>>);

    impl SharedBuffer {
        fn new() -> Self {
            Default::default()
        }
    }

    impl Write for SharedBuffer {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.try_lock().expect("mutex locked").write(buf)
        }

        fn flush(&mut self) -> std::io::Result<()> {
            self.0.try_lock().expect("mutex locked").flush()
        }
    }

    /// asserts that there must be color/no color, based on the truth of `COLOR`,
    /// in the given stream, given a copy of the buffer it links to
    fn expect_color<const COLOR: bool>(mut stream: impl Write, buf: SharedBuffer) {
        let expected: &[u8] = if COLOR {
            b"\x1b[1mcolor!\x1b[0m"
        } else {
            b"color!"
        };

        write!(stream, "{}color!{}", Style::new().bold(), Reset).expect("error writing");
        let mut grd = buf.0.try_lock().expect("mutex locked");

        grd.rewind().expect("error rewinding");
        let mut data = Vec::new();
        grd.read_to_end(&mut data).expect("error reading");

        assert_eq!(
            data, expected,
            "expected color: {}; found color: {}",
            COLOR, !COLOR
        );
    }

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

    #[test]
    fn test_set_color_choice() {
        fn assert_set_color<const COLOR: bool>(choice: ColorChoice) {
            let mut config = GlobalConfig::new();

            let out = SharedBuffer::new();
            let err = SharedBuffer::new();
            config.set_stdout(Box::new(out.clone()));
            config.set_stderr(Box::new(err.clone()));

            config.set_color_choice(choice);

            expect_color::<COLOR>(config.stdout(), out);
            expect_color::<COLOR>(config.stderr(), err);
        }

        assert_set_color::<true>(Always);
        assert_set_color::<true>(AlwaysAnsi);
        // a SharedBuffer is not a tty, so it should default to no colors.
        // TODO: is this test sound?
        assert_set_color::<false>(Auto);
        assert_set_color::<false>(Never);
    }

    #[test]
    fn test_set_out_color_choice() {
        fn assert_set_out_color<const COLOR: bool>(choice: ColorChoice) {
            let mut config = GlobalConfig::new();

            let out = SharedBuffer::new();
            config.set_stdout(Box::new(out.clone()));

            config.set_color_choice(choice);

            expect_color::<COLOR>(config.stdout(), out);
        }

        assert_set_out_color::<true>(Always);
        assert_set_out_color::<true>(AlwaysAnsi);
        assert_set_out_color::<false>(Auto);
        assert_set_out_color::<false>(Never);
    }

    #[test]
    fn test_set_err_color_choice() {
        fn assert_set_err_color<const COLOR: bool>(choice: ColorChoice) {
            let mut config = GlobalConfig::new();

            let err = SharedBuffer::new();
            config.set_stderr(Box::new(err.clone()));

            config.set_color_choice(choice);

            expect_color::<COLOR>(config.stderr(), err);
        }

        assert_set_err_color::<true>(Always);
        assert_set_err_color::<true>(AlwaysAnsi);
        assert_set_err_color::<false>(Auto);
        assert_set_err_color::<false>(Never);
    }
}
