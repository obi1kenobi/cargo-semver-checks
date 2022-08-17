pub trait BaselineLoader {
    fn load_rustdoc(&self, name: &str) -> anyhow::Result<std::path::PathBuf>;
}

pub struct RustdocBaseline {
    path: std::path::PathBuf,
}

impl RustdocBaseline {
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path }
    }
}

impl BaselineLoader for RustdocBaseline {
    fn load_rustdoc(&self, _name: &str) -> anyhow::Result<std::path::PathBuf> {
        Ok(self.path.clone())
    }
}
