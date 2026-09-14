pub mod common_stock;

use crate::domain::instruments::equities::common_stock::CommonStock;

/// An instrument representing ownership in an entity.
pub struct Equity {
    kind: EquityKind
}

/// Equity types for `Equity`.
pub enum EquityKind {
    CommonStock(CommonStock)
}