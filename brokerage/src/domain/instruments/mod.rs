pub mod currency;
pub mod equities;
pub mod futures;
pub mod options;

use crate::domain::instruments::{currency::Currency, equities::Equity, futures::Future, options::Option};

pub struct Instrument {
    symbol: String,
    kind: InstrumentKind,
    currency: Currency
}

pub enum InstrumentKind {
    Equity(Equity),
    Future(Future),
    Option(Option)
}