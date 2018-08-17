use super::*;

use std::collections::HashSet;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductSellerPrice {
    pub price: ProductPrice,
    pub currency_id: CurrencyId,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CartItem {
    pub id: CartItemId,
    pub customer: CartCustomer,
    pub product_id: ProductId,
    pub quantity: Quantity,
    pub selected: bool,
    pub comment: String,
    pub store_id: StoreId,
}

pub type Cart = HashSet<CartItem>;
