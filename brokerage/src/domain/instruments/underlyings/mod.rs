pub mod equities;

use crate::domain::instruments::underlyings::equities::Equity;

/// An instrument with a direct relation to an entity.
pub enum Underlying {
    Equity(Equity)
}