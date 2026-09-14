pub mod right;

use rust_decimal::Decimal;
use time::Date;

use crate::domain::instruments::{derivatives::{options::right::Right, settlement::Settlement}, underlyings::Underlying};

/// A derivative granting a right on an underlying instrument.
pub struct Option {
    underlying: Underlying,
    right: Right,
    strike: Decimal,
    expiration: Date,
    settlement: Settlement
}