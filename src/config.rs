use anstream::{AutoStream, ColorChoice};
use anstyle::{AnsiColor, Color, Reset, Style};
use clap::ValueEnum;
use std::{collections::HashSet, io::Write};

use crate::templating::make_handlebars_registry;

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
    feature_flags: HashSet<FeatureFlag>,
}

impl Default for GlobalConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl GlobalConfig {
    /// Creates a new `GlobalConfig` instance.
    ///
    /// Reads color choice from the value set by [`ColorChoice::write_global`] at the time
    /// of creation; see [`GlobalConfig::set_color_choice`] for finer-grained control over
    /// `cargo-semver-checks`'s color output
    pub fn new() -> Self {
        let stdout_choice = anstream::stdout().current_choice();
        let stderr_choice = anstream::stdout().current_choice();

        Self {
            level: None,
            handlebars: make_handlebars_registry(),
            minimum_rustc_version: semver::Version::new(1, 77, 0),
            stdout: AutoStream::new(Box::new(std::io::stdout()), stdout_choice),
            stderr: AutoStream::new(Box::new(std::io::stderr()), stderr_choice),
            feature_flags: HashSet::new(),
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars<'static> {
        &self.handlebars
    }

    pub fn minimum_rustc_version(&self) -> &semver::Version {
        &self.minimum_rustc_version
    }

    pub fn set_log_level(&mut self, level: Option<log::Level>) -> &mut Self {
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
        callback: impl FnOnce(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_verbose() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_extra_verbose(
        &mut self,
        callback: impl FnOnce(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_extra_verbose() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_info(
        &mut self,
        callback: impl FnOnce(&mut Self) -> anyhow::Result<()>,
    ) -> anyhow::Result<()> {
        if self.is_info() {
            callback(self)?;
        }
        Ok(())
    }

    pub fn log_error(
        &mut self,
        callback: impl FnOnce(&mut Self) -> anyhow::Result<()>,
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

    pub fn shell_error(&mut self, message: impl std::fmt::Display) -> anyhow::Result<()> {
        self.shell_print("error", message, Color::Ansi(AnsiColor::Red), false)
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
    /// Defaults to the global color choice setting set by [`ColorChoice::write_global`]
    /// *at the time of calling `set_stderr`*.
    /// Call [`GlobalConfig::set_err_color_choice`] to customize the color choice after if needed.
    pub fn set_stderr(&mut self, err: Box<dyn Write + 'static>) -> &mut Self {
        self.stderr = AutoStream::auto(err);
        self
    }

    /// Sets the stdout output stream
    ///
    /// Defaults to the global color choice setting set by [`ColorChoice::write_global`].
    /// *at the time of calling `set_stdout`*.
    /// Call [`GlobalConfig::set_out_color_choice`] to customize the color choice after if needed.
    pub fn set_stdout(&mut self, out: Box<dyn Write + 'static>) -> &mut Self {
        self.stdout = AutoStream::auto(out);
        self
    }

    /// Individually set the color choice setting for [`GlobalConfig::stderr`]
    ///
    /// Defaults to the global color choice in [`ColorChoice::global`], which can be set
    /// in [`ColorChoice::write_global`] if you are using the `anstream` crate.
    ///
    /// See also [`GlobalConfig::set_out_color_choice`] and [`GlobalConfig::set_color_choice`]
    pub fn set_err_color_choice(&mut self, use_color: bool) -> &mut Self {
        // `anstream` doesn't have a good mechanism to set color choice (on one stream)
        // without making a new object, so we have to make a new autostream, but since we need
        // to move the `RawStream` inner, we temporarily replace it with /dev/null
        let stderr = std::mem::replace(
            &mut self.stderr,
            AutoStream::never(Box::new(std::io::sink())),
        );
        self.stderr = AutoStream::new(
            stderr.into_inner(),
            if use_color {
                ColorChoice::Always
            } else {
                ColorChoice::Never
            },
        );
        self
    }

    /// Individually set the color choice setting for [`GlobalConfig::stdout`]
    ///
    /// Defaults to the global color choice in [`ColorChoice::global`], which can be set
    /// in [`ColorChoice::write_global`] if you are using the `anstream` crate.
    ///
    /// See also [`GlobalConfig::set_err_color_choice`] and [`GlobalConfig::set_color_choice`]
    pub fn set_out_color_choice(&mut self, use_color: bool) -> &mut Self {
        // `anstream` doesn't have a good mechanism to set color choice (on one stream)
        // without making a new object, so we have to make a new autostream, but since we need
        // to move the `RawStream` inner, we temporarily replace it with /dev/null
        let stdout = std::mem::replace(
            &mut self.stdout,
            AutoStream::never(Box::new(std::io::sink())),
        );
        self.stdout = AutoStream::new(
            stdout.into_inner(),
            if use_color {
                ColorChoice::Always
            } else {
                ColorChoice::Never
            },
        );
        self
    }

    /// Sets the color choice for both [`GlobalConfig::stderr`] and [`GlobalConfig::stdout`]
    ///
    /// If not set, defaults to the value in [`ColorChoice::global`] at the time the streams
    /// are set using [`GlobalConfig::set_stdout`] and `err`, which can be set beforehand
    ///
    /// See also [`GlobalConfig::set_err_color_choice`] and [`GlobalConfig::set_out_color_choice`]
    pub fn set_color_choice(&mut self, use_color: bool) -> &mut Self {
        self.set_err_color_choice(use_color);
        self.set_out_color_choice(use_color);
        self
    }

    /// Gets the color choice (i.e., whether to output colors) for the configured stderr
    ///
    /// See also [`GlobalConfig::set_err_color_choice`]
    #[must_use]
    #[inline]
    pub fn err_color_choice(&self) -> bool {
        match &self.stderr.current_choice() {
            ColorChoice::Always | ColorChoice::AlwaysAnsi => true,
            // note: the `auto` branch is unreachable, as [`AutoStream::current_choice`]
            // returns the *currently active* choice, not the initially-configured choice
            // so an initial choice of `Auto` would be converted into either `Always` or `Never`.
            ColorChoice::Never | ColorChoice::Auto => false,
        }
    }

    /// Gets the color choice (i.e., whether to output colors) for the configured stdout
    ///
    /// See also [`GlobalConfig::set_out_color_choice`]
    #[must_use]
    #[inline]
    pub fn out_color_choice(&self) -> bool {
        match &self.stdout.current_choice() {
            ColorChoice::Always | ColorChoice::AlwaysAnsi => true,
            // note: the `auto` branch is unreachable, as [`AutoStream::current_choice`]
            // returns the *currently active* choice, not the initially-configured choice
            // so an initial choice of `Auto` would be converted into either `Always` or `Never`.
            ColorChoice::Never | ColorChoice::Auto => false,
        }
    }

    /// Set (overwrite) the [`FeatureFlag`] set.
    #[inline]
    pub fn set_feature_flags(&mut self, flags: HashSet<FeatureFlag>) -> &mut Self {
        self.feature_flags = flags;
        self
    }

    /// Enable a single [feature flag](FeatureFlag).
    #[inline]
    pub fn enable_feature_flag(&mut self, flag: FeatureFlag) -> &mut Self {
        self.feature_flags.insert(flag);
        self
    }

    /// Test for whether a specific feature flag is enabled.  If the flag has been
    /// stabilized, this will always return true.
    #[must_use]
    #[inline]
    pub fn feature_flag_enabled(&self, flag: FeatureFlag) -> bool {
        flag.stable || self.feature_flags.contains(&flag)
    }

    /// Returns a set of all enabled feature flags.
    #[must_use]
    #[inline]
    pub fn feature_flags(&self) -> &HashSet<FeatureFlag> {
        &self.feature_flags
    }
}

/// A feature flag for gating unstable `cargo-semver-checks` features.
///
/// ## Feature-gating code
///
/// To only execute a block of code when a given feature flag `flag` has been enabled,
/// wrap the block in `if config.feature_flag_enabled(flag)`, where `config` is the
/// program's [`GlobalConfig`].
///
/// ## Adding a new unstable feature flag
///
/// Create a new associated constant in the `impl FeatureFlag` block with an identifier,
/// help message, and `stable: false`, and **add this constant** to the
/// [`ALL_FLAGS`](Self::ALL_FLAGS) slice.
///
/// ## Stabilizing a feature flag
///
/// Set `stable: true` on the associated constant for that flag.  To keep the transition
/// from unstable flag to stable feature, mark the associated constant `#[deprecated]`, but
/// don't remove it.  This will warn downstream code and `cargo-semver-checks` binary users
/// that the flag has been stabilized and may be removed in a future release, and hide it
/// from `-Z help`, without breaking downstream code and binary users right when it is stabilized.
///
/// When you stabilize a flag and mark it as `#[deprecated]`, remove any `if` blocks
/// that feature-gate code. Testing if a flag is enabled with [`GlobalConfig::feature_flag_enabled`]
/// will always return `true` when a flag has been stabilized.
///
/// ## See also
///
/// - [`GlobalConfig::feature_flag_enabled]
/// - [`GlobalConfig::feature_flags]
/// - [`GlobalConfig::set_feature_flags]
/// - [`GlobalConfig::enable_feature_flag]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FeatureFlag {
    /// `kebab-case` identifier for this feature flag.
    pub id: &'static str,
    /// Optional help message for this flag.
    ///
    /// If this contains multiple lines. the first should be able to serve as
    /// a one-line 'short help' when needed, and the full help string will be
    /// used as a `long help`.
    pub help: Option<&'static str>,
    /// Whether this flag is stable and enabled by default.  Stable flags may
    /// be removed in future releases of `cargo-semver-checks` as the feature
    /// is stabilized and feature-gated code is unconditionally executed.
    pub stable: bool,
}

impl FeatureFlag {
    /// Print a list of the current unstable feature flags.
    pub const HELP: Self = Self {
        id: "help",
        help: Some("Print a list of the current unstable feature flags"),
        stable: false,
    };

    /// Enables the use of unstable CLI flags.
    pub const UNSTABLE_OPTIONS: Self = Self {
        id: "unstable-options",
        help: Some(
            "Enables the use of unstable CLI flags.\n\
            Run `cargo semver-checks -Z help` to list them",
        ),
        stable: false,
    };

    /// All feature flags that currently exist in `cargo-semver-checks`.
    pub const ALL_FLAGS: &'static [Self] = &[Self::HELP, Self::UNSTABLE_OPTIONS];
}

impl ValueEnum for FeatureFlag {
    #[inline]
    fn value_variants<'a>() -> &'a [Self] {
        Self::ALL_FLAGS
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        Some(
            clap::builder::PossibleValue::new(self.id)
                .hide(self.stable)
                .help(self.help),
        )
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

    /// helper struct to implement `Write + 'static` while keeping
    /// view access to an underlying buffer
    ///
    /// Uses [`Mutex::try_lock`] so no calls should block even though it is a mutex
    #[derive(Debug, Clone, Default)]
    struct SharedBuffer(Rc<Mutex<Cursor<Vec<u8>>>>);

    impl SharedBuffer {
        fn new() -> Self {
            Self::default()
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

    /// asserts that there must be color/no color, based on the truth of `color`,
    /// in the given stream, given a copy of the buffer it links to
    fn expect_color(mut stream: impl Write, buf: SharedBuffer, color: bool) {
        let expected: &[u8] = if color {
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
            color, !color
        );
    }

    fn assert_color_choice(
        make_choice: impl Fn(&mut GlobalConfig),
        stdout_color: Option<bool>,
        stderr_color: Option<bool>,
    ) {
        let mut config = GlobalConfig::new();

        let out = SharedBuffer::new();
        let err = SharedBuffer::new();
        config.set_stdout(Box::new(out.clone()));
        config.set_stderr(Box::new(err.clone()));

        make_choice(&mut config);

        if let Some(stdout_color) = stdout_color {
            expect_color(config.stdout(), out, stdout_color);
        }

        if let Some(stderr_color) = stderr_color {
            expect_color(config.stderr(), err, stderr_color);
        }
    }

    #[test]
    fn test_log_level_info() {
        let mut config = GlobalConfig::new();
        config.set_log_level(Some(log::Level::Info));

        assert!(config.is_info());
        assert!(!config.is_verbose());
        assert!(!config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_debug() {
        let mut config = GlobalConfig::new();
        config.set_log_level(Some(log::Level::Debug));

        assert!(config.is_info());
        assert!(config.is_verbose());
        assert!(!config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_trace() {
        let mut config = GlobalConfig::new();
        config.set_log_level(Some(log::Level::Trace));

        assert!(config.is_info());
        assert!(config.is_verbose());
        assert!(config.is_extra_verbose());
    }

    #[test]
    fn test_log_level_none() {
        let mut config = GlobalConfig::new();
        config.set_log_level(None);

        assert!(!config.is_info());
        assert!(!config.is_verbose());
        assert!(!config.is_extra_verbose());
    }

    #[test]
    fn test_set_color_choice() {
        assert_color_choice(
            |config| {
                config.set_color_choice(false);
            },
            Some(false),
            Some(false),
        );
        assert_color_choice(
            |config| {
                config.set_color_choice(true);
            },
            Some(true),
            Some(true),
        );
    }

    #[test]
    fn test_set_out_color_choice() {
        assert_color_choice(
            |config| {
                config.set_out_color_choice(false);
            },
            Some(false),
            None,
        );
        assert_color_choice(
            |config| {
                config.set_out_color_choice(true);
            },
            Some(true),
            None,
        );
    }

    #[test]
    fn test_set_err_color_choice() {
        assert_color_choice(
            |config| {
                config.set_err_color_choice(false);
            },
            None,
            Some(false),
        );
        assert_color_choice(
            |config| {
                config.set_err_color_choice(true);
            },
            None,
            Some(true),
        );
    }

    #[test]
    fn test_set_global_color_choice() {
        ColorChoice::Always.write_global();
        assert_color_choice(|_| (), Some(true), Some(true));

        ColorChoice::AlwaysAnsi.write_global();
        assert_color_choice(|_| (), Some(true), Some(true));

        ColorChoice::Never.write_global();
        assert_color_choice(|_| (), Some(false), Some(false));

        // We don't test `ColorChoice::Auto` because it depends on the tty status of the output,
        // which could lead to a flaky test depending on where and how the test is executed.
    }

    #[test]
    fn test_get_color_choice() {
        let mut config = GlobalConfig::new();
        config.set_color_choice(true);
        assert!(config.err_color_choice());
        assert!(config.out_color_choice());

        config.set_out_color_choice(false);
        assert!(!config.out_color_choice());

        config.set_color_choice(false);
        config.set_err_color_choice(true);
        assert!(config.err_color_choice());

        ColorChoice::AlwaysAnsi.write_global();
        // we have to instantiate a new GlobalConfig here for it to read
        // the color choice
        config = GlobalConfig::new();
        assert!(config.err_color_choice());
        assert!(config.out_color_choice());
    }

    #[test]
    fn stable_flags_always_enabled() {
        let config = GlobalConfig::new();
        assert!(config.feature_flag_enabled(FeatureFlag {
            id: "ad-hoc",
            help: None,
            stable: true
        }));
        assert!(!config.feature_flag_enabled(FeatureFlag {
            id: "ad-hoc",
            help: None,
            stable: false,
        }));
    }
}
