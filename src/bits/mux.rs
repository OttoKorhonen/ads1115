/// Input multiplexer configuration (ADS1115 only)
/// These bits configure the input multiplexer.

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Mux {
    Ain0nd = 0b100,
    Ain1Gnd = 0b101,
    Ain2Gnd = 0b110,
    Ain3Gnd = 0b111,
    Ain0Ain1 = 0b000,
    Ain0Ain3 = 0b001,
    Ain1Ain3 = 0b010,
    Ain2Ain3 = 0b011,
}

impl Mux {
    pub fn get_value(self) -> u8 {
        self as u8
    }
}

impl Default for Mux {
    fn default() -> Self {
        Mux::Ain0Ain1
    }
}
