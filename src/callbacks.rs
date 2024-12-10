use std::collections::BTreeMap;

use crate::GlobalConfig;

pub(crate) struct Callbacks<'a> {
    config: &'a mut GlobalConfig,

    // (crate_name, version, is_baseline) keys
    generate_starts: BTreeMap<(&'a str, &'a str, bool), std::time::Instant>,
    parse_starts: BTreeMap<(&'a str, &'a str, bool), std::time::Instant>,
}

impl<'a> Callbacks<'a> {
    pub(crate) fn new(config: &'a mut GlobalConfig) -> Self {
        Self {
            config,
            generate_starts: Default::default(),
            parse_starts: Default::default(),
        }
    }
}

impl<'a> crate::data_generation::ProgressCallbacks<'a> for Callbacks<'a> {
    fn generate_rustdoc_start(&mut self, crate_name: &'a str, version: &'a str, is_baseline: bool) {
        let kind = if is_baseline { "baseline" } else { "current" };

        // Ignore terminal printing failures.
        let _ = self.config.shell_status(
            "Building",
            format_args!("{crate_name} v{version} ({kind})",),
        );

        self.generate_starts.insert(
            (crate_name, version, is_baseline),
            std::time::Instant::now(),
        );
    }

    fn generate_rustdoc_success(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
        let kind = if is_baseline { "baseline" } else { "current" };
        let start = self
            .generate_starts
            .remove(&(crate_name, version, is_baseline))
            .expect("success on generation task that never started");

        // Ignore terminal printing failures.
        let _ = self.config.shell_status(
            "Built",
            format_args!("[{:>8.3}s] ({kind})", start.elapsed().as_secs_f32()),
        );
    }

    fn parse_rustdoc_start(
        &mut self,
        cached: bool,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
        let kind = if is_baseline { "baseline" } else { "current" };
        let cached = if cached { ", cached" } else { "" };

        // Ignore terminal printing failures.
        let _ = self.config.shell_status(
            "Parsing",
            format_args!("{crate_name} v{version} ({kind}{cached})",),
        );

        self.parse_starts.insert(
            (crate_name, version, is_baseline),
            std::time::Instant::now(),
        );
    }

    fn parse_rustdoc_success(
        &mut self,
        _cached: bool,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
        let kind = if is_baseline { "baseline" } else { "current" };
        let start = self
            .parse_starts
            .remove(&(crate_name, version, is_baseline))
            .expect("success on parse task that never started");

        // Ignore terminal printing failures.
        let _ = self.config.shell_status(
            "Parsed",
            format_args!("[{:>8.3}s] ({kind})", start.elapsed().as_secs_f32()),
        );
    }

    fn non_fatal_error(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
        error: anyhow::Error,
    ) {
        let kind = if is_baseline { "baseline" } else { "current" };

        // Ignore terminal printing failures.
        let _ = self.config.log_info(|config| {
            config.shell_warn(format!(
                "encountered non-fatal error while working on crate \
                {crate_name} v{version} ({kind}): {error} (root cause: {})",
                error.root_cause()
            ))?;
            Ok(())
        });
    }
}
