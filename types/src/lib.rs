#[macro_use]
extern crate derive_more;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate uuid;

use uuid::Uuid;

#[derive(Clone, Copy, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct UserId(pub i32);

#[derive(Clone, Copy, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct RoleEntryId(pub Uuid);

impl RoleEntryId {
    pub fn new() -> Self {
        RoleEntryId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, FromStr, Hash, Serialize, Deserialize)]
pub struct ProductId(pub i32);

#[derive(Clone, Copy, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct Quantity(pub i32);

#[derive(Clone, Copy, Debug, Display, PartialEq, Eq, FromStr, Hash, Serialize, Deserialize)]
pub struct StoreId(pub i32);

#[derive(Clone, Copy, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct WarehouseId(pub Uuid);
impl WarehouseId {
    pub fn new() -> Self {
        WarehouseId(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct WarehouseSlug(pub String);

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum WarehouseIdentifier {
    Id(WarehouseId),
    Slug(WarehouseSlug),
}

#[derive(Clone, Copy, Debug, Display, PartialEq, FromStr, Hash, Serialize, Deserialize)]
pub struct StockId(pub Uuid);
impl StockId {
    pub fn new() -> Self {
        StockId(Uuid::new_v4())
    }
}
