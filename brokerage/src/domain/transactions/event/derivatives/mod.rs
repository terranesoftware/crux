use crate::domain::transactions::event::derivatives::{future::Future, option::Option};

pub mod future;
pub mod option;

pub enum Derivative {
    Future(Future),
    Option(Option)
}