pub enum Data {
    Int(i64),
    String(String),
    #[cfg(custom)]
    Bool(bool),
}
