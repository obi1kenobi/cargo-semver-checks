use termcolor::{ColorChoice, StandardStream};

use crate::templating::make_handlebars_registry;

#[allow(dead_code)]
pub(crate) struct GlobalConfig {
    printing_to_terminal: bool,
    output_writer: StandardStream,
    handlebars: handlebars::Handlebars<'static>,
}

impl GlobalConfig {
    pub fn new() -> Self {
        let printing_to_terminal = atty::is(atty::Stream::Stdout);

        let color_choice = match std::env::var("CARGO_TERM_COLOR").as_deref() {
            Ok("always") => ColorChoice::Always,
            Ok("alwaysansi") => ColorChoice::AlwaysAnsi,
            Ok("auto") => ColorChoice::Auto,
            Ok("never") => ColorChoice::Never,
            Ok(_) | Err(..) => {
                if printing_to_terminal {
                    ColorChoice::Auto
                } else {
                    ColorChoice::Never
                }
            }
        };

        Self {
            printing_to_terminal,
            output_writer: StandardStream::stdout(color_choice),
            handlebars: make_handlebars_registry(),
        }
    }

    pub fn printing_to_terminal(&self) -> bool {
        self.printing_to_terminal
    }

    pub fn output_writer(&mut self) -> &mut StandardStream {
        &mut self.output_writer
    }

    pub fn handlebars(&self) -> &handlebars::Handlebars<'static> {
        &self.handlebars
    }
}
