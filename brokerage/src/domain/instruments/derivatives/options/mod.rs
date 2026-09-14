use rust_decimal::Decimal;
use time::Date;

/// A derivative contract granting a right on an underlying instrument.
pub struct Option {
    right: Right,
    strike: Decimal,
    expiration: Date
}

/// The right granted by an `Option`.
pub enum Right {
    Call,
    Put
}