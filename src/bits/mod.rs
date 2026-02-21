pub mod gain_amp;
pub use gain_amp::GainAmp;

pub mod operating_mode;
pub use operating_mode::OperatingMode;

pub mod mux;
pub use mux::MuxChannels;
pub mod data_rate;
pub use data_rate::DataRate;
pub mod operating_status;
pub use operating_status::OperatingStatus;
pub mod comparator_mode;
pub use comparator_mode::ComparatorMode;
pub mod comparator_polarity;
pub use comparator_polarity::ComparatorPolarity;
pub mod latching_comparator;
pub use latching_comparator::LatchingComparator;
pub mod comparator_queue;
pub use comparator_queue::ComparatorQueue;
