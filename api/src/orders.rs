use rpc_client::RestApiClient;
use types::*;
use util::*;

use chrono::prelude::*;
use geo::Point as GeoPoint;
use regex::Regex;
use std::collections::HashMap;
use stq_roles;
use stq_static_resources::{Currency, OrderState};
use stq_types::*;
use validator::{Validate, ValidationError};

#[derive(Clone, Debug)]
pub enum Route {
    Cart {
        customer: CartCustomer,
    },
    CartProducts {
        customer: CartCustomer,
    },
    CartIncrementProduct {
        customer: CartCustomer,
        product_id: ProductId,
    },
    CartProduct {
        customer: CartCustomer,
        product_id: ProductId,
    },
    CartProductQuantity {
        customer: CartCustomer,
        product_id: ProductId,
    },
    CartProductSelection {
        customer: CartCustomer,
        product_id: ProductId,
    },
    CartProductComment {
        customer: CartCustomer,
        product_id: ProductId,
    },
    CartClear {
        customer: CartCustomer,
    },
    CartMerge,
    OrderFromCart,
    OrderFromCartRevert,
    OrderSearch,
    Orders,
    OrdersByUser {
        user: UserId,
    },
    OrdersByStore {
        store_id: StoreId,
    },
    Order {
        order_id: OrderIdentifier,
    },
    OrderDiff {
        order_id: OrderIdentifier,
    },
    OrderStatus {
        order_id: OrderIdentifier,
    },
    OrdersAllowedStatuses,
    Roles(stq_roles::routing::Route),
}

impl From<stq_roles::routing::Route> for Route {
    fn from(v: stq_roles::routing::Route) -> Self {
        Route::Roles(v)
    }
}

fn cart_customer_route(id: &CartCustomer) -> String {
    use self::CartCustomer::*;

    match id {
        User(user_id) => format!("by-user/{}", user_id),
        Anonymous(session_id) => format!("by-session/{}", session_id),
    }
}

fn order_identifier_route(id: &OrderIdentifier) -> String {
    use self::OrderIdentifier::*;

    match id {
        Id(id) => format!("by-id/{}", id),
        Slug(slug) => format!("by-slug/{}", slug),
    }
}

impl RouteBuilder for Route {
    fn route(&self) -> String {
        use self::Route::*;

        match self {
            Cart { customer } => format!("cart/{}", cart_customer_route(customer)),
            CartProducts { customer } => format!("cart/{}/products", cart_customer_route(customer)),
            CartIncrementProduct {
                customer,
                product_id,
            } => format!(
                "cart/{}/products/{}/increment",
                cart_customer_route(customer),
                product_id
            ),
            CartProduct {
                customer,
                product_id,
            } => format!(
                "cart/{}/products/{}",
                cart_customer_route(customer),
                product_id
            ),
            CartProductQuantity {
                customer,
                product_id,
            } => format!(
                "cart/{}/products/{}/quantity",
                cart_customer_route(customer),
                product_id
            ),
            CartProductSelection {
                customer,
                product_id,
            } => format!(
                "cart/{}/products/{}/selection",
                cart_customer_route(customer),
                product_id
            ),
            CartProductComment {
                customer,
                product_id,
            } => format!(
                "cart/{}/products/{}/comment",
                cart_customer_route(customer),
                product_id
            ),
            CartClear { customer } => format!("cart/{}/clear", cart_customer_route(customer)),
            CartMerge => "cart/merge".to_string(),
            OrderFromCart => "orders/create_from_cart".to_string(),
            OrderFromCartRevert => "orders/create_from_cart/revert".to_string(),
            OrderSearch => "orders/search".to_string(),
            Orders => "orders".to_string(),
            OrdersByUser { user } => format!("orders/by-user/{}", user),
            OrdersByStore { store_id } => format!("orders/by-store/{}", store_id),
            Order { order_id } => format!("orders/{}", order_identifier_route(order_id)),
            OrderDiff { order_id } => format!("order_diffs/{}", order_identifier_route(order_id)),
            OrderStatus { order_id } => {
                format!("orders/{}/status", order_identifier_route(order_id))
            }
            OrdersAllowedStatuses => "orders/allowed_statuses".to_string(),
            Roles(route) => route.route(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct SetterPayload<T> {
    pub value: T,
}

pub type CartProductQuantityPayload = SetterPayload<Quantity>;
pub type CartProductSelectionPayload = SetterPayload<bool>;
pub type CartProductCommentPayload = SetterPayload<String>;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartProductIncrementPayload {
    pub store_id: StoreId,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CartMergePayload {
    pub from: CartCustomer,
    pub to: CartCustomer,
}

/// Service that provides operations for interacting with user carts
pub trait CartClient {
    /// Get user's cart contents
    fn get_cart(&self, customer: CartCustomer) -> ApiFuture<Cart>;
    /// Increase item's quantity by 1
    fn increment_item(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        store_id: StoreId,
    ) -> ApiFuture<Cart>;
    /// Set item to desired quantity in user's cart
    fn set_quantity(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: Quantity,
    ) -> ApiFuture<Cart>;
    /// Set selection of the item in user's cart
    fn set_selection(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: bool,
    ) -> ApiFuture<Cart>;
    /// Set comment for item in user's cart
    fn set_comment(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: String,
    ) -> ApiFuture<Cart>;
    /// Delete item from user's cart
    fn delete_item(&self, customer: CartCustomer, product_id: ProductId) -> ApiFuture<Cart>;
    /// Clear user's cart
    fn clear_cart(&self, customer: CartCustomer) -> ApiFuture<Cart>;
    /// Iterate over cart
    fn list(&self, customer: CartCustomer, from: ProductId, count: i32) -> ApiFuture<Cart>;
    /// Merge carts
    fn merge(&self, from: CartCustomer, to: CartCustomer) -> ApiFuture<Cart>;
}

impl CartClient for RestApiClient {
    fn get_cart(&self, customer: CartCustomer) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::CartProducts { customer })),
        )
    }

    fn increment_item(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        store_id: StoreId,
    ) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::CartIncrementProduct {
                    customer,
                    product_id,
                }))
                .body(JsonPayload(&CartProductIncrementPayload { store_id })),
        )
    }

    fn set_quantity(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: Quantity,
    ) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .put(&self.build_route(&Route::CartProductQuantity {
                    customer,
                    product_id,
                }))
                .body(JsonPayload(&CartProductQuantityPayload { value })),
        )
    }

    fn set_selection(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: bool,
    ) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .put(&self.build_route(&Route::CartProductSelection {
                    customer,
                    product_id,
                }))
                .body(JsonPayload(&CartProductSelectionPayload { value })),
        )
    }

    fn set_comment(
        &self,
        customer: CartCustomer,
        product_id: ProductId,
        value: String,
    ) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .put(&self.build_route(&Route::CartProductComment {
                    customer,
                    product_id,
                }))
                .body(JsonPayload(&CartProductCommentPayload { value })),
        )
    }

    fn delete_item(&self, customer: CartCustomer, product_id: ProductId) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .delete(&self.build_route(&Route::CartProduct {
                    customer,
                    product_id,
                })),
        )
    }

    fn clear_cart(&self, customer: CartCustomer) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::CartClear { customer })),
        )
    }

    fn list(&self, customer: CartCustomer, from: ProductId, count: i32) -> ApiFuture<Cart> {
        http_req(self.http_client.get(&format!(
            "{}?offset={}&count={}",
            self.build_route(&Route::Cart { customer }),
            from,
            count
        )))
    }

    fn merge(&self, from: CartCustomer, to: CartCustomer) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::CartMerge))
                .body(JsonPayload(&CartMergePayload { from, to })),
        )
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AddressFull {
    pub location: Option<GeoPoint<f64>>,
    pub administrative_area_level_1: Option<String>,
    pub administrative_area_level_2: Option<String>,
    pub country: Option<String>,
    pub locality: Option<String>,
    pub political: Option<String>,
    pub postal_code: Option<String>,
    pub route: Option<String>,
    pub street_number: Option<String>,
    pub address: Option<String>,
    pub place_id: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Order {
    pub id: OrderId,
    pub created_from: CartItemId,
    pub conversion_id: ConversionId,
    pub slug: OrderSlug,
    pub customer: UserId,
    pub store: StoreId,
    pub product: ProductId,
    pub price: ProductPrice,
    pub currency: Currency,
    pub quantity: Quantity,
    pub address: AddressFull,
    pub receiver_name: String,
    pub receiver_phone: String,
    pub state: OrderState,
    pub payment_status: bool,
    pub delivery_company: Option<String>,
    pub track_id: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub fn validate_phone(phone: &str) -> Result<(), ValidationError> {
    lazy_static! {
        static ref PHONE_VALIDATION_RE: Regex = Regex::new(r"^\+?\d{7}\d*$").unwrap();
    }

    if PHONE_VALIDATION_RE.is_match(phone) {
        Ok(())
    } else {
        Err(ValidationError {
            code: "phone".into(),
            message: Some("Incorrect phone format".into()),
            params: HashMap::new(),
        })
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Validate)]
pub struct ConvertCartPayload {
    pub conversion_id: Option<ConversionId>,
    pub user_id: UserId,
    pub receiver_name: String,
    #[validate(custom = "validate_phone")]
    pub receiver_phone: String,
    #[serde(flatten)]
    pub address: AddressFull,
    pub seller_prices: HashMap<ProductId, ProductSellerPrice>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ConvertCartRevertPayload {
    pub conversion_id: ConversionId,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OrderSearchTerms {
    pub slug: Option<OrderSlug>,
    pub created_from: Option<DateTime<Utc>>,
    pub created_to: Option<DateTime<Utc>>,
    pub payment_status: Option<bool>,
    pub customer: Option<UserId>,
    pub store: Option<StoreId>,
    pub state: Option<OrderState>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OrderDiff {
    pub id: OrderDiffId,
    pub parent: OrderId,
    pub committer: UserId,
    pub committed_at: DateTime<Utc>,
    pub state: OrderState,
    pub comment: Option<String>,
}

pub trait OrderClient {
    fn convert_cart(
        &self,
        conversion_id: Option<ConversionId>,
        user_id: UserId,
        seller_prices: HashMap<ProductId, ProductSellerPrice>,
        address: AddressFull,
        receiver_name: String,
        receiver_phone: String,
    ) -> ApiFuture<Vec<Order>>;
    fn revert_cart_conversion(&self, conversion_id: ConversionId) -> ApiFuture<()>;
    fn get_order(&self, id: OrderIdentifier) -> ApiFuture<Option<Order>>;
    fn get_order_diff(&self, id: OrderIdentifier) -> ApiFuture<Vec<OrderDiff>>;
    fn get_orders_for_user(&self, user_id: UserId) -> ApiFuture<Vec<Order>>;
    fn get_orders_for_store(&self, store_id: StoreId) -> ApiFuture<Vec<Order>>;
    fn delete_order(&self, id: OrderIdentifier) -> ApiFuture<()>;
    fn set_order_state(
        &self,
        order_id: OrderIdentifier,
        state: OrderState,
        comment: Option<String>,
        track_id: Option<String>,
    ) -> ApiFuture<Option<Order>>;
    /// Search using the terms provided.
    fn search(&self, terms: OrderSearchTerms) -> ApiFuture<Vec<Order>>;
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct UpdateStatePayload {
    pub state: OrderState,
    pub track_id: Option<String>,
    pub comment: Option<String>,
}

impl OrderClient for RestApiClient {
    fn convert_cart(
        &self,
        conversion_id: Option<ConversionId>,
        user_id: UserId,
        seller_prices: HashMap<ProductId, ProductSellerPrice>,
        address: AddressFull,
        receiver_name: String,
        receiver_phone: String,
    ) -> ApiFuture<Vec<Order>> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::OrderFromCart))
                .body(JsonPayload(ConvertCartPayload {
                    conversion_id,
                    user_id,
                    seller_prices,
                    address,
                    receiver_name,
                    receiver_phone,
                })),
        )
    }
    fn revert_cart_conversion(&self, conversion_id: ConversionId) -> ApiFuture<()> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::OrderFromCartRevert))
                .body(JsonPayload(ConvertCartRevertPayload { conversion_id })),
        )
    }
    fn get_order(&self, order_id: OrderIdentifier) -> ApiFuture<Option<Order>> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::Order { order_id })),
        )
    }
    fn get_order_diff(&self, order_id: OrderIdentifier) -> ApiFuture<Vec<OrderDiff>> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::OrderDiff { order_id })),
        )
    }
    fn get_orders_for_user(&self, user: UserId) -> ApiFuture<Vec<Order>> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::OrdersByUser { user })),
        )
    }
    fn get_orders_for_store(&self, store_id: StoreId) -> ApiFuture<Vec<Order>> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::OrdersByStore { store_id })),
        )
    }
    fn delete_order(&self, order_id: OrderIdentifier) -> ApiFuture<()> {
        http_req(
            self.http_client
                .delete(&self.build_route(&Route::Order { order_id })),
        )
    }
    fn set_order_state(
        &self,
        order_id: OrderIdentifier,
        state: OrderState,
        comment: Option<String>,
        track_id: Option<String>,
    ) -> ApiFuture<Option<Order>> {
        http_req(
            self.http_client
                .put(&self.build_route(&Route::OrderStatus { order_id }))
                .body(JsonPayload(UpdateStatePayload {
                    state,
                    comment,
                    track_id,
                })),
        )
    }
    fn search(&self, terms: OrderSearchTerms) -> ApiFuture<Vec<Order>> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::OrderSearch))
                .body(JsonPayload(terms)),
        )
    }
}
