use rust_decimal::Decimal;
use time::Timestamp;

use crate::domain::instruments::Instrument;

/// A record of a trade.
pub struct Transaction {
    instrument: Instrument,
    quantity: Decimal,
    price: Decimal,
    time: Timestamp
}