use crate::currency::Currency;
use crate::errors::Error;
use crate::traits::ExchangeRate;
use std::fmt;
use std::ops::{Add,Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Amount {
    pub amount: i64,
    pub currency: Currency
}

impl Amount {
    pub fn new(amount: i64, currency: Currency) -> Self {
        Amount {
            amount: amount,
            currency: currency
        }
    }

    pub fn major(self) -> i64 {
        if self.currency.decimals() > 0 {
            return self.amount / 10i64.pow(self.currency.decimals() as u32);
        }
        self.amount
    }

    pub fn minor(self) -> Option<i64> {
        if self.currency.decimals() > 0 {
            return Some(self.amount % 10i64.pow(self.currency.decimals() as u32));
        }
        None
    }

    pub fn exchange<T: ExchangeRate + Copy>(self, currency: Currency, rate: &T) -> Result<Amount, Error> {
        let exchange_rate = rate.exchange_rate(self.currency, currency)?;
        Ok(Amount {
            amount: (self.amount as f64 * exchange_rate).round() as i64,
            currency: currency
        })
    }
}

impl fmt::Display for Amount {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.amount, self.currency)
    }
}

impl Add for Amount {
    type Output = Result<Amount, Error>;

    fn add(self, other: Amount) -> Self::Output {
        if self.currency != other.currency {
            return Err(Error::CurrencyMismatch);
        }
        Ok(Amount {
            amount: self.amount + other.amount,
            currency: self.currency
        })
    }
}

impl Sub for Amount {
    type Output = Result<Amount, Error>;

    fn sub(self, other: Amount) -> Self::Output {
        if self.currency != other.currency {
            return Err(Error::CurrencyMismatch);
        }
        Ok(Amount {
            amount: self.amount - other.amount,
            currency: self.currency
        })
    }
}