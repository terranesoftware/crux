pub mod common_stock;

use crate::domain::instruments::underlyings::equities::common_stock::CommonStock;

/// An underlying representing ownership in an entity.
pub struct Equity {
    kind: EquityKind
}

/// Equity types for `Equity`.
pub enum EquityKind {
    CommonStock(CommonStock)
}