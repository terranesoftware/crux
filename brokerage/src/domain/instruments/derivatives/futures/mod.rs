use rust_decimal::Decimal;
use time::Date;

use crate::domain::instruments::{derivatives::settlement::Settlement, underlyings::Underlying};

/// A derivative establishing a future obligation on an underlying instrument.
pub struct Future {
    underlying: Underlying,
    multiplier: Decimal,
    tick: Decimal,
    expiration: Date,
    settlement: Settlement
}