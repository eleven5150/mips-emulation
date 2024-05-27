#[derive(Debug, Clone, PartialEq)]
pub struct MpLabel {
    label: String,
    col: u32,
    col_end: u32,
}

impl MpLabel {
    pub fn label(&self) -> String {
        self.label.to_string()
    }

    pub fn col(&self) -> u32 {
        self.col
    }

    pub fn col_end(&self) -> u32 {
        self.col_end
    }
}