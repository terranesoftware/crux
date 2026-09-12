pub mod currency;
pub mod derivatives;
pub mod equities;

use crate::domain::instruments::{currency::Currency, equities::Equity, derivatives::{futures::Future, options::Option}};

/// A tradable financial entity.
pub struct Instrument {
    /// The unique identifier for this particular `Instrument`.
    symbol: String,
    kind: InstrumentKind,
    currency: Currency
}

pub enum InstrumentKind {
    Equity(Equity),
    Future(Future),
    Option(Option)
}