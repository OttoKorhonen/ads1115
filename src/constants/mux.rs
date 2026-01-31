/// Input multiplexer configuration (ADS1115 only)
/// These bits configure the input multiplexer.
pub const AIN0_GND: u16 = 0b100 << 12;
pub const AIN1_GND: u16 = 0b101 << 12;
pub const AIN2_GND: u16 = 0b110 << 12;
pub const AIN3_GND: u16 = 0b111 << 12;

pub const AIN0_AIN1: u16 = 0b000 << 12;
pub const AIN0_AIN3: u16 = 0b001 << 12;
pub const AIN1_AIN3: u16 = 0b010 << 12;
pub const AIN2_AIN3: u16 = 0b011 << 12;
