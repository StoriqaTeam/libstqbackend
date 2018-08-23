use std::fmt;

use uuid::Uuid;

use stq_static_resources::Currency;

macro_rules! f64_newtype {
    ($x:ident) => {
        #[derive(Clone, Copy, Debug, Display, Default, PartialEq, PartialOrd, From, FromStr,
                 Into, Serialize, Deserialize, DieselTypes)]
        pub struct $x(pub f64);
    };
}
macro_rules! i32_newtype {
    ($x:ident) => {
        #[derive(Clone, Copy, Debug, Display, Default, PartialEq, Eq, PartialOrd, Ord, From,
                 FromStr, Into, Hash, Serialize, Deserialize, DieselTypes)]
        pub struct $x(pub i32);
    };
}
macro_rules! string_newtype {
    ($x:ident) => {
        #[derive(Clone, Debug, Display, Default, PartialEq, Eq, PartialOrd, Ord, From, FromStr,
                 Into, Hash, Serialize, Deserialize, DieselTypes)]
        pub struct $x(pub String);
    };
}
macro_rules! uuid_newtype {
    ($x:ident) => {
        #[derive(Clone, Copy, Debug, Default, Display, PartialEq, Eq, PartialOrd, Ord, From,
                 FromStr, Into, Hash, Serialize, Deserialize, DieselTypes)]
        pub struct $x(pub Uuid);

        impl $x {
            pub fn new() -> Self {
                $x(Uuid::new_v4())
            }
        }
    };
}

i32_newtype!(UserId);
i32_newtype!(SessionId);
i32_newtype!(ProductId);
i32_newtype!(BaseProductId);
i32_newtype!(Quantity);
i32_newtype!(StoreId);
i32_newtype!(OrderSlug);

string_newtype!(WarehouseSlug);

uuid_newtype!(RoleEntryId);
uuid_newtype!(RoleId);
uuid_newtype!(StockId);
uuid_newtype!(InvoiceId);
uuid_newtype!(SagaId);
uuid_newtype!(MerchantId);
uuid_newtype!(CartItemId);
uuid_newtype!(OrderId);
uuid_newtype!(OrderDiffId);
uuid_newtype!(OrderInfoId);
uuid_newtype!(CallbackId);
uuid_newtype!(ConversionId);
uuid_newtype!(WarehouseId);

f64_newtype!(ProductPrice);

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, From, FromStr, Into, Hash,
         Serialize, Deserialize, DieselTypes)]
pub struct CurrencyId(pub i32);

impl fmt::Display for CurrencyId {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                x if x == Currency::Rouble as i32 => Currency::Rouble.to_string(),
                x if x == Currency::Euro as i32 => Currency::Euro.to_string(),
                x if x == Currency::Dollar as i32 => Currency::Dollar.to_string(),
                x if x == Currency::Bitcoin as i32 => Currency::Bitcoin.to_string(),
                x if x == Currency::Etherium as i32 => Currency::Etherium.to_string(),
                x if x == Currency::Stq as i32 => Currency::Stq.to_string(),
                _ => "unknown".to_string(),
            }
        )
    }
}

impl<'a> From<&'a ProductPrice> for f64 {
    fn from(p: &ProductPrice) -> f64 {
        p.0
    }
}
