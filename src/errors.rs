
#[derive(Clone, Debug,)]
pub enum Error {
  CurrencyMismatch,
  CouldNotGetExchangeRate
}

impl std::convert::From<reqwest::Error> for Error {
  fn from(_err: reqwest::Error) -> Error {
    Error::CouldNotGetExchangeRate
  }
}