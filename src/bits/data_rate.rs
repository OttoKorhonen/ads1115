/// Data Rate
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum DataRate {
    Dr8SPs = 0b000,
    Dr16SPs = 0b001,
    Dr32SPs = 0b010,
    Dr64SPs = 0b011,
    Dr128SPs = 0b100, // Default
    Dr250SPs = 0b101,
    Dr475SPs = 0b110,
    Dr860SPs = 0b111,
}

impl DataRate {
    pub fn get_value(self) -> u8 {self as u8}
}

impl Default for DataRate {
    fn default() -> Self {
        DataRate::Dr128SPs
    }
}