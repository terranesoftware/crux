pub mod currency;
pub mod derivatives;
pub mod underlyings;

use crate::domain::instruments::{currency::Currency, derivatives::Derivative, underlyings::Underlying};

/// A tradable financial entity.
pub struct Instrument {
    /// The unique identifier for this particular `Instrument`.
    symbol: String,
    kind: InstrumentKind,
    currency: Currency
}

/// Instrument types for `Instrument`.
pub enum InstrumentKind {
    Derivative(Derivative),
    Underlying(Underlying),
}