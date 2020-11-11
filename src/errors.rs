#[derive(Clone, Debug,)]
pub enum Error {
  CurrencyMismatch,
  CouldNotGetExchangeRate
}
