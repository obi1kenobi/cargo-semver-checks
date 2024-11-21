#[allow(unused_variables)] // All default implementations are no-ops.
pub(crate) trait ProgressCallbacks<'a> {
    fn generate_placeholder_project_start(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn generate_placeholder_project_success(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn rustdoc_cache_hit(&mut self, crate_name: &'a str, version: &'a str, is_baseline: bool) {}

    fn generate_rustdoc_start(&mut self, crate_name: &'a str, version: &'a str, is_baseline: bool) {
    }

    fn generate_rustdoc_success(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn rustdoc_cache_populated(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn parse_rustdoc_start(
        &mut self,
        cached: bool,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn parse_rustdoc_success(
        &mut self,
        cached: bool,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
    ) {
    }

    fn non_fatal_error(
        &mut self,
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
        error: anyhow::Error,
    ) {
    }
}

pub(crate) struct CallbackHandler<'a> {
    crate_name: &'a str,
    version: &'a str,
    is_baseline: bool,
    callbacks: &'a mut dyn ProgressCallbacks<'a>,
}

impl std::fmt::Debug for CallbackHandler<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CallbackHandler")
            .field("crate_name", &self.crate_name)
            .field("version", &self.version)
            .field("callbacks", &"<dyn Trait>")
            .finish()
    }
}

impl<'a> CallbackHandler<'a> {
    pub(crate) fn new(
        crate_name: &'a str,
        version: &'a str,
        is_baseline: bool,
        callbacks: &'a mut dyn ProgressCallbacks<'a>,
    ) -> Self {
        Self {
            crate_name,
            version,
            is_baseline,
            callbacks,
        }
    }

    pub(super) fn generate_placeholder_project_start(&mut self) {
        self.callbacks.generate_placeholder_project_start(
            self.crate_name,
            self.version,
            self.is_baseline,
        )
    }

    pub(super) fn generate_placeholder_project_success(&mut self) {
        self.callbacks.generate_placeholder_project_success(
            self.crate_name,
            self.version,
            self.is_baseline,
        )
    }

    pub(super) fn rustdoc_cache_hit(&mut self) {
        self.callbacks
            .rustdoc_cache_hit(self.crate_name, self.version, self.is_baseline)
    }

    pub(super) fn generate_rustdoc_start(&mut self) {
        self.callbacks
            .generate_rustdoc_start(self.crate_name, self.version, self.is_baseline)
    }

    pub(super) fn generate_rustdoc_success(&mut self) {
        self.callbacks
            .generate_rustdoc_success(self.crate_name, self.version, self.is_baseline)
    }

    pub(super) fn rustdoc_cache_populated(&mut self) {
        self.callbacks
            .rustdoc_cache_populated(self.crate_name, self.version, self.is_baseline)
    }

    pub(super) fn parse_rustdoc_start(&mut self, cached: bool) {
        self.callbacks
            .parse_rustdoc_start(cached, self.crate_name, self.version, self.is_baseline)
    }

    pub(super) fn parse_rustdoc_success(&mut self, cached: bool) {
        self.callbacks.parse_rustdoc_success(
            cached,
            self.crate_name,
            self.version,
            self.is_baseline,
        )
    }

    pub(super) fn non_fatal_error(&mut self, error: anyhow::Error) {
        self.callbacks
            .non_fatal_error(self.crate_name, self.version, self.is_baseline, error)
    }
}
