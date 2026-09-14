pub mod futures;
pub mod options;
pub mod settlement;

use crate::domain::instruments::derivatives::{futures::Future, options::Option};

/// An instrument with a derivative relationship to an entity.
pub enum Derivative {
    Future(Future),
    Option(Option)
}