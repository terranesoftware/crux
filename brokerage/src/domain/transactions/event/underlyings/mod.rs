pub mod cash;
pub mod equities;

use crate::domain::transactions::event::underlyings::{cash::Cash, equities::Equity};

pub enum Underlying {
    Cash(Cash),
    Equity(Equity)
}