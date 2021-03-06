use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Currency {
    AED,
    ALL,
    AMD,
    ANG,
    AOA,
    ARS,
    AUD,
    AWG,
    AZN,
    BAM,
    BBD,
    BDT,
    BGN,
    BHD,
    BMD,
    BND,
    BOB,
    BRL,
    BSD,
    BWP,
    BYN,
    BZD,
    CAN,
    CHF,
    CLP,
    CNY,
    COP,
    CRC,
    CSD,
    CUP,
    CVE,
    CZK,
    DJF,
    DKK,
    DOP,
    DZD,
    EEK,
    EGP,
    ETB,
    EUR,
    FJD,
    FKP,
    GBP,
    GEL,
    GHS,
    GIP,
    GMD,
    GNF,
    GTQ,
    GYD,
    HKD,
    HNL,
    HRK,
    HTG,
    HUF,
    IDR,
    ILS,
    INR,
    IQD,
    ISK,
    JMD,
    JOD,
    JPY,
    KES,
    KGS,
    KHR,
    KMF,
    KRW,
    KWD,
    KYD,
    KZT,
    LAK,
    LBP,
    LKR,
    LTL,
    LVL,
    LYD,
    MAD,
    MDL,
    MKD,
    MMK,
    MNT,
    MOP,
    MRU,
    MUR,
    MVR,
    MWK,
    MXN,
    MYR,
    MZN,
    NAD,
    NGN,
    NIO,
    NOK,
    NPR,
    NZD,
    OMR,
    PAB,
    PEN,
    PGK,
    PHP,
    PKR,
    PLN,
    PYG,
    QAR,
    RON,
    RSD,
    RUB,
    RWF,
    SAR,
    SBD,
    SCR,
    SEK,
    SGD,
    SHP,
    SKK,
    SLL,
    SOS,
    SRD,
    STN,
    SVC,
    SZL,
    THB,
    TND,
    TOP,
    TRY,
    TTD,
    TWD,
    TZS,
    UAH,
    UGX,
    USD,
    UYU,
    UZS,
    VEF,
    VND,
    VUV,
    WST,
    XAF,
    XCD,
    XOF,
    XPF,
    YER,
    ZAR,
    ZMW
}

impl Currency {
    pub fn decimals(self) -> u32 {
        match self {
            Currency::CVE | Currency::DJF | Currency::GNF | Currency::IDR | 
            Currency::JPY | Currency::KMF | Currency::KRW | Currency::PYG |
            Currency::RWF | Currency::UGX | Currency::VND | Currency::VUV |
            Currency::XAF | Currency::XOF | Currency::XPF => 0,
            Currency::BHD | Currency::IQD | Currency::JOD | Currency::KWD |
            Currency::LYD | Currency::OMR | Currency::TND => 3,
            _ => 2
        }
    }

    pub fn abbreviation(self) -> String {
        format!("{:?}", self)
    }

    pub fn character(self) -> Option<&'static str> {
        match self {
            Currency::EUR => Some("€"),
            Currency::USD => Some("$"),
            _ => None
        }
    }
}

impl fmt::Display for Currency {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.abbreviation())
  }
}