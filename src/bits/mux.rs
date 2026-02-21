/// Input multiplexer configuration (ADS1115 only)
/// These bits configure the input multiplexer.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum MuxChannels {
    Ain0Gnd = 0b100,
    Ain1Gnd = 0b101,
    Ain2Gnd = 0b110,
    Ain3Gnd = 0b111,
    Ain0Ain1 = 0b000,
    Ain0Ain3 = 0b001,
    Ain1Ain3 = 0b010,
    Ain2Ain3 = 0b011,
}

impl MuxChannels {
    pub const  fn get_value(self) -> u16 {
        self as u16
    }
}

impl Default for MuxChannels {
    fn default() -> Self {
        MuxChannels::Ain0Ain1
    }
}
