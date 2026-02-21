

/// Device Operating Mode
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatingMode {
    ModeContinuous = 0b0,
    ModeSingleShot = 0b1  // Default
}

impl OperatingMode {
    pub const fn get_value(self) -> u8 {self as u8}
}

impl Default for OperatingMode {
    fn default() -> Self {
        OperatingMode::ModeSingleShot
    }
}