// Programmable Gain Amplifier (PGA)
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GainAmp {
    Pga6144V = 0b000,
    Pga4096V = 0b001,
    Pga2048V = 0b010,
    Pga1024V = 0b011,
    Pga0512V = 0b100,
    Pga0256V = 0b101,
}

impl GainAmp {
    pub const fn get_value(self) -> u8 {self as u8} 
}

impl Default for GainAmp {
    fn default() -> Self {
        GainAmp::Pga2048V
    }
}