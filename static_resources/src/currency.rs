use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, EnumIterator)]
pub enum Currency {
    Rouble = 1,
    Euro,
    Dollar,
    Bitcoin,
    Etherium,
    Stq,
}

#[derive(GraphQLObject, Serialize, Deserialize, Debug)]
pub struct CurrencyGraphQl {
    pub key: i32,
    pub name: String,
}

impl CurrencyGraphQl {
    pub fn new(key: i32, name: String) -> Self {
        Self { key, name }
    }
}

impl Currency {
    pub fn as_vec() -> Vec<CurrencyGraphQl> {
        Currency::enum_iter()
            .map(|value| CurrencyGraphQl::new(value as i32, value.to_string()))
            .collect()
    }
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
