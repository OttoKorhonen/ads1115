use crate::bits::gain_amp::GainAmp;
use crate::bits::mux::MuxChannels;

const MASK_OPERATIONAL_STATUS: u16 = 0b1000_0000_0000_0000;
const MASK_MUX: u16 = 0b0111_0000_0000_0000;
const MASK_GAIN_AMPLIFIER: u16 = 0b0000_1110_0000_0000;
const MASK_MODE: u16 = 0b0000_0001_0000_0000;
const MASK_DATA_RATE:u16 = 0b0000_0000_1110_0000;
const MASK_COMPARATOR_MODE: u16 = 0b0000_0000_0000_1000;
const MASK_COMPARATOR_POLARITY: u16 = 0b0000_0000_0000_0100;
const MASK_COMPARATOR_Q_AND_D: u16 = 0b0000_0000_0000_0011;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    bits: u16
}

impl Config {
    pub const fn new() -> Self {Self{bits: 0}}
    pub const fn get_value(self) -> u16 { self.bits }
    pub const fn set_operational_status(mut self) -> Self{
        todo!()
    }
    pub const fn set_multiplexer_config(mut self, mux: MuxChannels) -> Self {
        todo!()
    }
    pub const fn set_gain_amplifier_config(mut self, gain: GainAmp) -> Self {
        todo!()
    }
    pub const fn set_device_operation_mode(mut self) -> Self {
        todo!()
    }
    pub const fn set_data_rate(mut self) -> Self {
        todo!()
    }
    pub const fn set_comparator_mode(mut self) -> Self {
        todo!()
    }
    pub const fn set_latching_comparator(mut self) -> Self {
        todo!()
    }
    pub const fn set_comparator_queue(mut self) -> Self {
        todo!()
    }
}

impl Default for Config{
    fn default() -> Self {
        let default_bits = 0b0000_0101_1000_0011;
        Self {bits: default_bits}
    }
}




