/// Comparator Mode
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ComparatorMode {
    CompModeTraditional = 0b0, // Default
    CompModeWindow = 0b1,
}
impl ComparatorMode {
    pub const fn get_value(self) -> u8 {
        self as u8
    }
}
impl Default for ComparatorMode {
    fn default() -> Self {
        ComparatorMode::CompModeTraditional
    }
}
