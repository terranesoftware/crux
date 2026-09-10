use rust_decimal::Decimal;
use time::Timestamp;

use crate::domain::instruments::Instrument;

pub struct Transaction {
    instrument: Instrument,
    quantity: Decimal,
    price: Decimal,
    time: Timestamp
}