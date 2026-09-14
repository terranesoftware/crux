use rust_decimal::Decimal;

use crate::domain::instruments::derivatives::settlement::Settlement;

/// A derivative contract establishing a future obligation on an underlying instrument.
pub struct Future {
    leverage: Decimal,
    multiplier: Decimal,
    tick: Decimal,
    settlement: Settlement
}