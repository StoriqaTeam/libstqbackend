use super::*;

use std::collections::HashMap;

fn return_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductSellerPrice {
    pub price: ProductPrice,
    pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartItemInfo {
    pub quantity: Quantity,
    #[serde(default = "return_true")]
    pub selected: bool,
    pub comment: String,
    pub store_id: StoreId,
}

pub type Cart = HashMap<ProductId, CartItemInfo>;
