use std::fmt;
use std::str::FromStr;

use juniper::FieldError;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Service {
    Users,
    Stores,
    Orders,
}

impl fmt::Display for Service {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Service::Users => write!(f, "users"),
            Service::Stores => write!(f, "stores"),
            Service::Orders => write!(f, "orders"),
        }
    }
}

impl FromStr for Service {
    type Err = FieldError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "users" => Ok(Service::Users),
            "stores" => Ok(Service::Stores),
            "orders" => Ok(Service::Orders),
            _ => Err(FieldError::new(
                "Unknown service",
                graphql_value!({ "code": 300, "details": {
                        format!("Can not resolve service name. Unknown service: '{}'", s)
                        }}),
            )),
        }
    }
}
