use crate::amount::Amount;

pub fn add_tax(amount: Amount, tax_rate: f64) -> Amount {
    Amount::new(amount.amount + (amount.amount as f64 * tax_rate) as i64, amount.currency)
}

pub fn get_tax(amount: Amount, tax_rate: f64) -> Amount {
    Amount::new((amount.amount as f64 * tax_rate) as i64, amount.currency)
}

pub fn remove_tax(amount: Amount, tax_rate: f64) -> Amount {
    Amount::new((amount.amount as f64 / (1.0 + tax_rate)) as i64, amount.currency)
}
