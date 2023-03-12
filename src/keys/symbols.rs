pub struct KeySymbols {
    pub enter: String,
}

impl Default for KeySymbols {
    fn default() -> Self {
        Self {
            enter: "\u{23ce}".into(),
        }
    }
}