use super::*;

use std::collections::HashSet;
use stq_static_resources::Currency;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductSellerPrice {
    pub price: ProductPrice,
    pub currency: Currency,
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
    pub pre_order: bool,
    pub pre_order_days: i32,
    pub coupon_id: Option<CouponId>,
}

pub type Cart = HashSet<CartItem>;
