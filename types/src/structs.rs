use std::cmp::{Ord, Ordering};
use std::fmt;

use uuid::Uuid;

use stq_static_resources::Currency;

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct UserId(pub i32);

impl From<UserId> for i32 {
    fn from(prod: UserId) -> i32 {
        prod.0
    }
}

#[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct RoleEntryId(pub Uuid);

impl RoleEntryId {
    pub fn new() -> Self {
        RoleEntryId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Default, FromStr, Display, Eq, PartialEq, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct RoleId(pub Uuid);

impl RoleId {
    pub fn new() -> Self {
        RoleId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct ProductId(pub i32);

impl From<ProductId> for i32 {
    fn from(prod: ProductId) -> i32 {
        prod.0
    }
}

impl Ord for ProductId {
    fn cmp(&self, other: &ProductId) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for ProductId {
    fn partial_cmp(&self, other: &ProductId) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct Quantity(pub i32);

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct StoreId(pub i32);

impl From<StoreId> for i32 {
    fn from(prod: StoreId) -> i32 {
        prod.0
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct WarehouseId(pub Uuid);
impl WarehouseId {
    pub fn new() -> Self {
        WarehouseId(Uuid::new_v4())
    }
}

#[derive(Clone, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize, Deserialize,
         DieselTypes)]
pub struct WarehouseSlug(pub String);

#[derive(Clone, Debug, PartialEq, Hash)]
pub enum WarehouseIdentifier {
    Id(WarehouseId),
    Slug(WarehouseSlug),
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct StockId(pub Uuid);
impl StockId {
    pub fn new() -> Self {
        StockId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct InvoiceId(pub Uuid);

impl InvoiceId {
    pub fn new() -> Self {
        InvoiceId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct SagaId(pub Uuid);

impl SagaId {
    pub fn new() -> Self {
        SagaId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct MerchantId(pub Uuid);

impl MerchantId {
    pub fn new() -> Self {
        MerchantId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct OrderId(pub Uuid);

impl OrderId {
    pub fn new() -> Self {
        OrderId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct OrderInfoId(pub Uuid);

impl OrderInfoId {
    pub fn new() -> Self {
        OrderInfoId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, FromStr, Hash, Serialize,
         Deserialize, DieselTypes)]
pub struct CallbackId(pub Uuid);

impl CallbackId {
    pub fn new() -> Self {
        CallbackId(Uuid::new_v4())
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, FromStr, Hash, Serialize, Deserialize,
         DieselTypes)]
pub struct CurrencyId(pub i32);

impl fmt::Display for CurrencyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                x if x == Currency::Euro as i32 => Currency::Euro.to_string(),
                x if x == Currency::Dollar as i32 => Currency::Dollar.to_string(),
                x if x == Currency::Bitcoin as i32 => Currency::Bitcoin.to_string(),
                x if x == Currency::Etherium as i32 => Currency::Etherium.to_string(),
                x if x == Currency::Stq as i32 => Currency::Stq.to_string(),
                _ => "".to_string(),
            }
        )
    }
}
