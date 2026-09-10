pub mod common_stock;

use crate::domain::instruments::equities::common_stock::CommonStock;

pub struct Equity {
    kind: EquityKind
}

pub enum EquityKind {
    CommonStock(CommonStock)
}