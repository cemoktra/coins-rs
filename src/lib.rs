pub mod currency;
pub mod amount;
pub mod errors;
pub mod traits;
pub mod ecb;
pub mod tax;

#[macro_use]
#[cfg(test)]
extern crate more_asserts;

#[cfg(test)]
mod tests {
    use crate::currency::Currency;
    use crate::amount::Amount;
    use crate::traits::ExchangeRate;
    use crate::errors::Error;
    use crate::ecb::EuropeanCentralBank;
    use crate::tax;

    #[test]
    fn test_currency_decimals() {
        assert_eq!(0, Currency::CVE.decimals());
        assert_eq!(2, Currency::EUR.decimals());
        assert_eq!(3, Currency::BHD.decimals());
    }

    #[test]
    fn test_currency_abbreviation() {
        assert!("EUR" == Currency::EUR.abbreviation());
        assert!("USD" == Currency::USD.abbreviation());
    }
    
    #[test]
    fn test_amount_major_minor() {
        let amount = Amount::new(525, Currency::CVE);
        assert_eq!(525, amount.major());
        assert!(amount.minor().is_none());

        let amount = Amount::new(525, Currency::EUR);
        assert_eq!(5, amount.major());
        assert!(amount.minor().is_some());
        assert_eq!(25, amount.minor().unwrap());

        let amount = Amount::new(525, Currency::BHD);
        assert_eq!(0, amount.major());
        assert!(amount.minor().is_some());
        assert_eq!(525, amount.minor().unwrap());
    }

    #[test]
    fn test_amount_add_sub() {
        let amount1 = Amount::new(100, Currency::EUR);
        let amount2 = Amount::new(150, Currency::EUR);
        let amount3 = Amount::new(200, Currency::USD);
        let expect = Amount::new(250, Currency::EUR);

        let res = amount1 + amount2;
        assert!(res.is_ok());
        let res = res.unwrap();
        assert_eq!(expect, res);
        let res = res - amount2;
        assert!(res.is_ok());
        assert_eq!(amount1, res.unwrap());

        let res = amount1 + amount3;
        assert!(res.is_err());
    }

    #[derive(Clone, Copy)]
    struct ExchangeRateMock;

    #[derive(Clone, Copy)]
    struct ExchangeRateErrorMock;

    impl ExchangeRate for ExchangeRateMock {
        fn exchange_rate(self, _from: Currency, _to: Currency) -> Result<f64, Error>
        {
            Ok(2.0f64)
        }
    }

    impl ExchangeRate for ExchangeRateErrorMock {
        fn exchange_rate(self, _from: Currency, _to: Currency) -> Result<f64, Error>
        {
            Err(Error::CouldNotGetExchangeRate)
        }
  }

    #[test]
    fn test_amount_exchange() {
        let exchange_mock = ExchangeRateMock;
        let exchange_error_mock = ExchangeRateErrorMock;
        let amount_eur = Amount::new(100, Currency::EUR);
        let amount_usd = amount_eur.exchange(Currency::USD, &exchange_mock);
        assert!(amount_usd.is_ok());
        assert_eq!(amount_usd.unwrap().amount, 200);
        let amount_usd = amount_eur.exchange(Currency::USD, &exchange_error_mock);
        assert!(amount_usd.is_err());
    }

    #[test]
    fn test_ecb_exchange_rate() {
        let ecb = EuropeanCentralBank;
        let rate = ecb.exchange_rate(Currency::EUR, Currency::JPY);
        assert!(rate.is_ok());
        assert_gt!(rate.unwrap(), 1.0);
    }

    #[test]
    fn test_tax_add() {
        let net_amount = Amount::new(500, Currency::EUR);
        let gross_amount = tax::add_tax(net_amount, 0.16);
        assert_eq!(580, gross_amount.amount);
        let tax_amount = tax::get_tax(net_amount, 0.16);
        assert_eq!(80, tax_amount.amount);
        let net_amount = tax::remove_tax(gross_amount, 0.16);
        assert_eq!(500, net_amount.amount);
    }
}
