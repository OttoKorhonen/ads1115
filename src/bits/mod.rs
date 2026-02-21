pub mod gain_amp;
pub use gain_amp::GainAmp;

pub mod operating_mode;
pub use operating_mode::OperatingMode;

pub mod mux;
mod data_rate;
mod operating_status;
mod comparator_mode;
mod comparator_polarity;
mod latching_comparator;
mod comparator_queue;

pub use mux::MuxChannels;