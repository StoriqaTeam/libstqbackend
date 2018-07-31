use rpc_client::RpcClientImpl;
use types::*;
use util::*;

use serde_json;
use stq_roles;
use stq_types::*;

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
                "cart/{}/product/{}",
                cart_customer_route(customer),
                product_id
            ),
            CartProductQuantity {
                customer,
                product_id,
            } => format!(
                "cart/{}/product/{}/quantity",
                cart_customer_route(customer),
                product_id
            ),
            CartProductSelection {
                customer,
                product_id,
            } => format!(
                "cart/{}/product/{}/selection",
                cart_customer_route(customer),
                product_id
            ),
            CartProductComment {
                customer,
                product_id,
            } => format!(
                "cart/{}/product/{}/comment",
                cart_customer_route(customer),
                product_id
            ),
            CartClear { customer } => format!("cart/{}/clear$", cart_customer_route(customer)),
            CartMerge => "cart/merge$".to_string(),
            OrderFromCart => "orders/create_from_cart".to_string(),
            OrderFromCartRevert => "orders/create_from_cart/revert".to_string(),
            OrderSearch => "orders/search".to_string(),
            Orders => "orders".to_string(),
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
    pub from: SessionId,
    pub to: UserId,
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
    fn merge(&self, from: SessionId, to: UserId) -> ApiFuture<Cart>;
}

impl CartClient for RpcClientImpl {
    fn get_cart(&self, customer: CartCustomer) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .get(&self.build_route(&Route::Cart { customer })),
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
                .body(serde_json::to_string(&CartProductIncrementPayload { store_id }).unwrap()),
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
                .body(serde_json::to_string(&CartProductQuantityPayload { value }).unwrap()),
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
                .body(serde_json::to_string(&CartProductSelectionPayload { value }).unwrap()),
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
                .body(serde_json::to_string(&CartProductCommentPayload { value }).unwrap()),
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

    fn merge(&self, from: SessionId, to: UserId) -> ApiFuture<Cart> {
        http_req(
            self.http_client
                .post(&self.build_route(&Route::CartMerge))
                .body(serde_json::to_string(&CartMergePayload { from, to }).unwrap()),
        )
    }
}
