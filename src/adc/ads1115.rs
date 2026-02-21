use crate::bits::GainAmp;
use crate::error::Ads1115error;
use crate::register::Config;
use embedded_hal::i2c::SevenBitAddress;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Ads1115<I2C> {
    i2c: I2C,
    address: SevenBitAddress,
}

impl<I2C> Ads1115<I2C>
where
    I2C: embedded_hal::i2c::I2c,
{
    pub fn new(i2c: I2C, address: SevenBitAddress) -> Self {
        Self { i2c, address }
    }

    // MUX configurations are now available in crate::register::mux
    // Example: crate::register::mux::AIN0_GND

    pub fn set_config(&mut self, config: Config) -> Result<(), Ads1115error<I2C::Error>> {
        let msb = (config.get_value() >> 8) as u8;
        let lsb = config.get_value() as u8;
        self.i2c.write(self.address, &[0x01, msb, lsb])?;
        Ok(())
    }

    /// Reads the raw ADC result from the Conversion Register (0x00).
    ///
    /// NOTE: This returns the result of the *last completed conversion* only.
    /// It corresponds to the MUX channel that was active during that conversion.
    /// It is NOT a combined value of all pins.
    pub fn read_raw_data(&mut self) -> Result<[u8; 2], Ads1115error<I2C::Error>> {
        let mut buffer = [0u8; 2];
        self.i2c.write_read(self.address, &[0x00], &mut buffer)?;
        Ok(buffer)
    }

    ///function returns value converted from raw data
    pub fn read_data(&mut self) -> Result<i16, Ads1115error<I2C::Error>> {
        let raw = self.read_raw_data()?;
        Ok(Self::raw_data_to_value(&raw))
    }

    ///function takes i16 value and converts it into millivolts
    pub fn convert_to_mv(value: i16, gain: GainAmp) -> f32 {
        let lsb_value = match gain {
            GainAmp::Pga6144V => 0.1875,
            GainAmp::Pga4096V => 0.125,
            GainAmp::Pga2048V => 0.0625,
            GainAmp::Pga1024V => 0.03125,
            GainAmp::Pga0512V => 0.015625,
            GainAmp::Pga0256V => 0.0078125,
        };
        (value as f32) * lsb_value
    }

    fn raw_data_to_value(raw_data: &[u8; 2]) -> i16 {
        // Ads1115 sends data MSB first (Big Endian)
        i16::from_be_bytes(*raw_data)
    }
}
