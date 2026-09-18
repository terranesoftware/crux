pub mod derivatives;
pub mod underlyings;

use crate::domain::transactions::event::{derivatives::Derivative, underlyings::Underlying};

pub enum Event {
    Derivative(Derivative),
    Underlying(Underlying)
}
