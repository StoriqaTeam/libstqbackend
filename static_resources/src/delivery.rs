use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

#[derive(GraphQLEnum, Deserialize, Serialize, Clone, PartialEq, Eq, Debug, DieselTypes)]
#[graphql(name = "DeliveryCompany", description = "delivery company")]
pub enum DeliveryCompany {
    #[graphql(description = "UPS")]
    UPS,
    #[graphql(description = "DHL")]
    DHL,
}

impl fmt::Display for DeliveryCompany {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            DeliveryCompany::DHL => write!(f, "dhl"),
            DeliveryCompany::UPS => write!(f, "ups"),
        }
    }
}

impl FromStr for DeliveryCompany {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "dhl" => DeliveryCompany::DHL,
            "ups" => DeliveryCompany::UPS,
            _ => {
                return Err(FieldError::new(
                    "Unknown DeliveryCompany",
                    graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve delivery company name. Unknown DeliveryCompany: '{}'", s)
                        }}),
                ))
            }
        })
    }
}
