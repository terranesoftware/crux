pub mod equities;

use crate::domain::instruments::{currency::Currency, underlyings::equities::Equity};

/// An instrument with a direct relation to an entity.
pub enum Underlying {
    Cash(Currency),
    Equity(Equity)
}