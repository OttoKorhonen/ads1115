use core::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum Ads1115error<E> {
    I2cError(E),
    DeviceNotFound,
    DivideByZeroError
}

impl<E> fmt::Display for Ads1115error<E>
where
    E: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::I2cError(e) => write!(f, "I2C error: {:?}", e),
            Self::DeviceNotFound => write!(f, "Device not found"),
            Self::DivideByZeroError => write!(f, "Cannot divide value by zero")
        }
    }
}

impl<E> core::error::Error for Ads1115error<E> where E: fmt::Debug {}

impl<E> From<E> for Ads1115error<E> {
    fn from(e: E) -> Self {
        Self::I2cError(e)
    }
}