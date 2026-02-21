/// Comparator Polarity
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ComparatorPolarity {
    CompPolActiveLow = 0b0, // Default
    CompPolActiveHigh = 0b1,
}

impl ComparatorPolarity {
    pub const fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for ComparatorPolarity {
    fn default() -> Self {
        ComparatorPolarity::CompPolActiveLow
    }
}
