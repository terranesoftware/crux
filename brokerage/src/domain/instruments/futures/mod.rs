use rust_decimal::Decimal;

use crate::domain::instruments::settlement::Settlement;

pub struct Future {
    leverage: Decimal,
    multiplier: Decimal,
    tick: Decimal,
    settlement: Settlement,
}
