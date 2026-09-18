pub mod event;

use time::Timestamp;

use crate::domain::{instruments::Instrument, transactions::event::Event};

/// A record of a trade.
pub struct Transaction {
    instrument: Instrument,
    event: Event,
    time: Timestamp
}