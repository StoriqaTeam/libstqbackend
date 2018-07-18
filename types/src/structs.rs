use super::*;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ProductSellerPrice {
    pub price: ProductPrice,
    pub currency_id: CurrencyId,
}
