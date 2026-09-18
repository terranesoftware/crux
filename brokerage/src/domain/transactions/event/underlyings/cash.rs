use rust_decimal::Decimal;

pub enum Cash {
    Deposit(Decimal),
    Withdraw(Decimal)
}