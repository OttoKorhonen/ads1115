/// Latching Comparator
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LatchingComparator {
    CompLatNonLatching = 0b0, // Default
    CompLatLatching = 0b1,
}
impl LatchingComparator {
    pub const fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for LatchingComparator {
    fn default() -> Self {
        LatchingComparator::CompLatNonLatching
    }
}
