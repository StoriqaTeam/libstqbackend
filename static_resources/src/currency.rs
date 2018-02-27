use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

pub enum Currency {
    Rouble = 1,
    Euro,
    Dollar,
    Bitcoin,
    Etherium,
    Stq,
}

impl fmt::Display for Currency {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let currency = match *self {
            Currency::Rouble => "rouble",
            Currency::Euro => "euro",
            Currency::Dollar => "dollar",
            Currency::Bitcoin => "bitcoin",
            Currency::Etherium => "etherium",
            Currency::Stq => "stq",
        };
        write!(f, "{}", currency)
    }
}

impl FromStr for Currency {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "rouble" => Currency::Rouble,
            "euro" => Currency::Euro,
            "dollar" => Currency::Dollar,
            "bitcoin" => Currency::Bitcoin,
            "etherium" => Currency::Etherium,
            "stq" => Currency::Stq,
            _ => {
                return Err(FieldError::new(
                    "Unknown Currency",
                    graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve Currency name. Unknown Currency: '{}'", s)
                        }}),
                ))
            }
        })
    }
}
