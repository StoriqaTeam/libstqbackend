use super::*;

use std::fmt;

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes)]
pub enum StoresRole {
    Superuser,
    User,
    StoreManager,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes)]
pub enum UsersRole {
    Superuser,
    User,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize, DieselTypes)]
pub enum MerchantType {
    Store,
    User,
}

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum WarehouseIdentifier {
    Id(WarehouseId),
    Slug(WarehouseSlug),
}

/// Anything that can uniquely identify an Order
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum OrderIdentifier {
    Id(OrderId),
    Slug(OrderSlug),
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Serialize, Deserialize)]
pub enum CartCustomer {
    User(UserId),
    Anonymous(SessionId),
}

impl fmt::Display for CartCustomer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::CartCustomer::*;

        write!(
            f,
            "{}",
            match self {
                User(id) => format!("user / {}", id),
                Anonymous(id) => format!("session / {}", id),
            }
        )
    }
}
