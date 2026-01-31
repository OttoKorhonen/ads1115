use embedded_hal::{i2c::SevenBitAddress, delay::DelayNs};
use crate::error::Ads1115error;
use core::fmt;

pub struct Ads1115<I2C, D> {
    i2c: I2C,
    address: SevenBitAddress,
    delay: D,
}

impl<I2C, D> Ads1115<I2C, D>
where
    I2C: embedded_hal::i2c::I2c,
    I2C::Error: fmt::Debug,
    D: DelayNs,
{
    pub fn new(i2c: I2C, address: SevenBitAddress, delay: D) -> Self {
        Self {
            i2c,
            address,
            delay,
        }
    }

    // MUX configurations are now available in crate::constants::mux
    // Example: crate::constants::mux::AIN0_GND

    /// Configures the device.
    /// To measure battery (single-ended AIN0, 8SPS, +/- 4.096V range, Single-shot):
    /// config = 0b1_100_001_1_100_0_0_0_11
    /// OS=1 (start), MUX=100 (AIN0-GND), PGA=001 (4.096V), MODE=1 (Single-shot)
    /// DR=000 (8SPS), COMP bits=default
    pub fn configure(&mut self, config: u16) -> Result<(), Ads1115error<I2C::Error>> {
        let high_byte = (config >> 8) as u8;
        let low_byte = config as u8;
        // Register 0x01 is the Config Register
        self.i2c.write(self.address, &[0x01, high_byte, low_byte])?;
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
        Ok(self.raw_data_to_value(&raw))
    }

    /// Reads a specific channel in single-shot mode.
    /// This combines configuration, waiting for conversion, and reading the result.
    pub fn read_single_shot(&mut self, config: u16) -> Result<i16, Ads1115error<I2C::Error>> {
        self.configure(config)?;

        // Wait for conversion to complete.
        // At 8 SPS, one sample takes 125ms. We wait 130ms to be safe.
        // If you use faster data rates, you can optimize this delay or implement OS-bit polling.
        self.delay.delay_ms(130);

        self.read_data()
    }

    ///function takes i16 value and converts it into millivolts
    pub fn convert_to_mv(value: i16) -> Result<i16, Ads1115error<I2C::Error>> {
        let divisor = 8;
        if divisor == 0 {
            return Err(Ads1115error::DivideByZeroError);
        }
        Ok(value / divisor)
    }

    fn raw_data_to_value(&mut self, raw_data: &[u8; 2]) -> i16 {
        // Ads1115 sends data MSB first (Big Endian)
        i16::from_be_bytes(*raw_data)
    }
}