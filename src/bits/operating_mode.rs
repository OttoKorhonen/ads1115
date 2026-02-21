/// Device Operating Mode
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatingMode {
    ModeContinuous = 0b0,
    ModeSingleShot = 0b1, // Default
}

impl OperatingMode {
    pub const fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for OperatingMode {
    fn default() -> Self {
        OperatingMode::ModeSingleShot
    }
}
