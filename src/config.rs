use termcolor::{ColorChoice, StandardStream};

use crate::templating::make_handlebars_registry;

#[allow(dead_code)]
pub(crate) struct GlobalConfig {
    is_stdout_tty: bool,
    stdout: StandardStream,
    stderr: StandardStream,
    handlebars: handlebars::Handlebars<'static>,
}

impl GlobalConfig {
    pub fn new() -> Self {
        let is_stdout_tty = atty::is(atty::Stream::Stdout);

        let color_choice = match std::env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => ColorChoice::Always,
            Ok("alwaysansi") => ColorChoice::AlwaysAnsi,
            Ok("auto") => ColorChoice::Auto,
            Ok("never") => ColorChoice::Never,
            Ok(_) | Err(..) => {
                if is_stdout_tty {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                }
            }
        };

        Self {
            is_stdout_tty,
            stdout: StandardStream::stdout(color_choice),
            stderr: StandardStream::stderr(color_choice),
            handlebars: make_handlebars_registry(),
        }
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars<'static> {
        &self.handlebars
    }

    pub fn is_stdout_tty(&self) -> bool {
        self.is_stdout_tty
    }

    pub fn stdout(&mut self) -> &mut StandardStream {
        &mut self.stdout
    }

    pub fn stderr(&mut self) -> &mut StandardStream {
        &mut self.stderr
    }

    /// Print a message with a colored title in the style of Cargo shell messages.
    pub fn shell_print(
        &mut self,
        status: impl std::fmt::Display,
        message: impl std::fmt::Display,
        color: termcolor::Color,
        justified: bool,
    ) -> anyhow::Result<()> {
        use std::io::Write;
        use termcolor::WriteColor;

        self.stderr().set_color(
            termcolor::ColorSpec::new()
                .set_fg(Some(color))
                .set_bold(true),
        )?;
        if justified {
            write!(self.stderr(), "{status:>12}")?;
        } else {
            write!(self.stderr(), "{}", status)?;
            self.stderr()
                .set_color(termcolor::ColorSpec::new().set_bold(true))?;
            write!(self.stderr(), ":")?;
        }
        self.stderr().reset()?;

        writeln!(self.stderr(), " {message}")?;

        Ok(())
    }

    /// Print a styled action message.
    pub fn shell_status(
        &mut self,
        action: impl std::fmt::Display,
        message: impl std::fmt::Display,
    ) -> anyhow::Result<()> {
        self.shell_print(action, message, termcolor::Color::Green, true)
    }
}
