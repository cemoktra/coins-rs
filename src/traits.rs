use crate::currency::Currency;
use crate::errors::Error;

pub trait ExchangeRate {
  fn exchange_rate(self, from: Currency, to: Currency) -> Result<f64, Error>;
}