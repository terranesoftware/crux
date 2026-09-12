use rust_decimal::Decimal;
use time::Date;

pub struct Option {
    right: Right,
    strike: Decimal,
    expiration: Date
}

pub enum Right {
    Call,
    Put
}