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

#[derive(Clone, Debug, PartialEq, Eq, From, Hash)]
pub enum WarehouseIdentifier {
    Id(WarehouseId),
    Slug(WarehouseSlug),
}

/// Anything that can uniquely identify an Order
#[derive(Clone, Copy, Debug, Eq, From, PartialEq, Hash)]
pub enum OrderIdentifier {
    Id(OrderId),
    Slug(OrderSlug),
}

/// Anything that can uniquely identify a page
#[derive(Clone, Debug, Eq, From, PartialEq, Hash)]
pub enum PageIdentifier {
    Id(PageId),
    Slug(PageSlug),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, From, Hash, Serialize, Deserialize)]
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
