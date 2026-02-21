///This bit determines the operational status of the device.
/// This bit can only be written when in power-down mode.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum OperatingStatus {
    OSWriteNoEffect = 0b0,
    OSWriteSingleConversion = 0b1,
}
impl OperatingStatus {
    pub const fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for OperatingStatus {
    fn default() -> Self {
        OperatingStatus::OSWriteSingleConversion
    }
}
