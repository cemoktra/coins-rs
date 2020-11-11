use crate::traits::ExchangeRate;
use crate::currency::Currency;
use crate::errors::Error;
use std::collections::HashMap;
use serde::Deserialize;

#[derive(Clone, Copy)]
pub struct EuropeanCentralBank;

#[derive(Deserialize)]
struct ApiResponse {
  rates: HashMap<String, f64>
}

impl ExchangeRate for EuropeanCentralBank {
  fn exchange_rate(self, from: Currency, to: Currency) -> Result<f64, Error>
  {
    let url = format!("https://api.exchangeratesapi.io/latest?base={}&symbols={}", from, to);
    match reqwest::blocking::get(&url) {
      Ok(body) => {
        let response: Result<ApiResponse, reqwest::Error> = body.json();
        match response {
          Ok(response) => {
            match response.rates.get(&to.abbreviation()) {
              Some(rate) => Ok(*rate),
              None => Err(Error::CouldNotGetExchangeRate)
            }
          }
          Err(_e) => Err(Error::CouldNotGetExchangeRate)
        }
      }
      Err(_e) => Err(Error::CouldNotGetExchangeRate)
    }
  }
}