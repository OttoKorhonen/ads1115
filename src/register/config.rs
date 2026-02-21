use crate::bits::{ComparatorMode, ComparatorPolarity, ComparatorQueue, DataRate, GainAmp, LatchingComparator, MuxChannels, OperatingMode, OperatingStatus};

const MASK_OPERATIONAL_STATUS: u16 = 0b1000_0000_0000_0000;
const MASK_MUX: u16 = 0b0111_0000_0000_0000;
const MASK_GAIN_AMPLIFIER: u16 = 0b0000_1110_0000_0000;
const MASK_MODE: u16 = 0b0000_0001_0000_0000;
const MASK_DATA_RATE: u16 = 0b0000_0000_1110_0000;
const MASK_COMPARATOR_MODE: u16 = 0b0000_0000_0001_0000;
const MASK_COMPARATOR_POLARITY: u16 = 0b0000_0000_0000_1000;
const MASK_LATCHING_COMPARATOR: u16 = 0b0000_0000_0000_0100;
const MASK_COMPARATOR_Q_AND_D: u16 = 0b0000_0000_0000_0011;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Config {
    bits: u16,
}

impl Config {
    pub const fn new() -> Self {
        Self { bits: 0 }
    }
    pub const fn get_value(self) -> u16 {
        self.bits
    }
    pub const fn set_operational_status(mut self, operating_status: OperatingStatus) -> Self {
        self.bits = (self.bits & !MASK_OPERATIONAL_STATUS)
            | (operating_status.get_value() & MASK_OPERATIONAL_STATUS);
        self
    }
    pub const fn set_multiplexer_config(mut self, mux: MuxChannels) -> Self {
        self.bits = (self.bits & !MASK_MUX) | (mux.get_value() & MASK_MUX);
        self
    }
    pub const fn set_gain_amplifier_config(mut self, gain: GainAmp) -> Self {
        self.bits = (self.bits & !MASK_GAIN_AMPLIFIER) | (gain.get_value() & MASK_GAIN_AMPLIFIER);
        self
    }
    pub const fn set_device_operation_mode(mut self, operating_mode: OperatingMode) -> Self {
        self.bits = (self.bits & !MASK_MODE) | (operating_mode.get_value() & MASK_MODE);
        self
    }
    pub const fn set_data_rate(mut self, data_rate: DataRate) -> Self {
        self.bits = (self.bits & !MASK_DATA_RATE) | (data_rate.get_value() & MASK_DATA_RATE);
        self
    }
    pub const fn set_comparator_mode(mut self, comparator_mode: ComparatorMode) -> Self {
        self.bits = (self.bits & !MASK_COMPARATOR_MODE)
            | (comparator_mode.get_value() & MASK_COMPARATOR_MODE);
        self
    }
    pub const fn set_comparator_polarity(mut self, comparator_polarity:ComparatorPolarity) -> Self {
        self.bits = (self.bits & !MASK_COMPARATOR_POLARITY) | (comparator_polarity.get_value() & MASK_COMPARATOR_POLARITY);
        self
    }
    pub const fn set_latching_comparator(mut self, latching_comparator: LatchingComparator) -> Self {
        self.bits = (self.bits & !MASK_LATCHING_COMPARATOR)
            | (latching_comparator.get_value() & MASK_LATCHING_COMPARATOR);
        self
    }
    pub const fn set_comparator_queue(mut self, comparator_queue: ComparatorQueue) -> Self {
        self.bits = (self.bits & !MASK_COMPARATOR_Q_AND_D)
            | (comparator_queue.get_value() & MASK_COMPARATOR_Q_AND_D);
        self
    }
}

impl Default for Config {
    fn default() -> Self {
        let default_bits = 0b0000_0101_1000_0011;
        Self { bits: default_bits }
    }
}
