/// Latching Comparator
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LatchingComparator {
    CompLatNonLatching = 0b0, // Default
    CompLatLatching = 0b1,
}
impl LatchingComparator {
    pub const fn get_value(self) -> u8 {
        self as u8
    }
}

impl Default for LatchingComparator {
    fn default() -> Self {
        LatchingComparator::CompLatNonLatching
    }
}
