/// Operational Status or Single-Shot Conversion Start
/// This bit determines the operational status on the ADS1115.
/// OS=0 : No effect
/// OS=1 : Start a single conversion (when in power-down state)
pub const OS_START_SINGLE: u16 = 0b1 << 15;

/// Programmable Gain Amplifier (PGA)
pub const PGA_6_144V: u16 = 0b000 << 9;
pub const PGA_4_096V: u16 = 0b001 << 9;
pub const PGA_2_048V: u16 = 0b010 << 9; // Default
pub const PGA_1_024V: u16 = 0b011 << 9;
pub const PGA_0_512V: u16 = 0b100 << 9;
pub const PGA_0_256V: u16 = 0b101 << 9;

/// Device Operating Mode
pub const MODE_CONTINUOUS: u16 = 0b0 << 8;
pub const MODE_SINGLE_SHOT: u16 = 0b1 << 8; // Default

/// Data Rate
pub const DR_8SPS: u16 = 0b000 << 5;
pub const DR_16SPS: u16 = 0b001 << 5;
pub const DR_32SPS: u16 = 0b010 << 5;
pub const DR_64SPS: u16 = 0b011 << 5;
pub const DR_128SPS: u16 = 0b100 << 5; // Default
pub const DR_250SPS: u16 = 0b101 << 5;
pub const DR_475SPS: u16 = 0b110 << 5;
pub const DR_860SPS: u16 = 0b111 << 5;

/// Comparator Mode
pub const COMP_MODE_TRADITIONAL: u16 = 0b0 << 4; // Default
pub const COMP_MODE_WINDOW: u16 = 0b1 << 4;

/// Comparator Polarity
pub const COMP_POL_ACTIVE_LOW: u16 = 0b0 << 3; // Default
pub const COMP_POL_ACTIVE_HIGH: u16 = 0b1 << 3;

/// Latching Comparator
pub const COMP_LAT_NON_LATCHING: u16 = 0b0 << 2; // Default
pub const COMP_LAT_LATCHING: u16 = 0b1 << 2;

/// Comparator Queue and Disable
pub const COMP_QUE_ASSERT_1: u16 = 0b00;
pub const COMP_QUE_ASSERT_2: u16 = 0b01;
pub const COMP_QUE_ASSERT_4: u16 = 0b10;
pub const COMP_QUE_DISABLE: u16 = 0b11; // Default
