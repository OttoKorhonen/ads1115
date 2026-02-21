/// Comparator Queue and Disable
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ComparatorQueue {
    CompQueAssert1 = 0b00,
    CompQueAssert2 = 0b01,
    CompQueAssert3 = 0b10,
    DisableComparator = 0b11, // Default
}

impl ComparatorQueue {
    pub const fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for ComparatorQueue {
    fn default() -> Self {
        ComparatorQueue::DisableComparator
    }
}
