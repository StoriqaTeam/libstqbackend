#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq, DieselTypes)]
#[graphql(name = "OrderStatus", description = "Current order status")]
pub enum OrderStatus {
    #[graphql(description = "State set on order creation.")]
    #[serde(rename = "payment_awaited")]
    PaymentAwaited,

    #[graphql(description = "Set after payment by request of billing")]
    #[serde(rename = "paid")]
    Paid,

    #[graphql(description = "Order is being processed by store management")]
    #[serde(rename = "in_processing")]
    InProcessing,

    #[graphql(description = "Can be cancelled by any party before order being sent.")]
    #[serde(rename = "cancelled")]
    Cancelled,

    #[graphql(description = "Wares are on their way to the customer. Tracking ID must be set.")]
    #[serde(rename = "sent")]
    Sent,

    #[graphql(description = "Wares are delivered to the customer.")]
    #[serde(rename = "delivered")]
    Delivered,

    #[graphql(description = "Wares are received by the customer.")]
    #[serde(rename = "received")]
    Received,

    #[graphql(description = "Order is complete.")]
    #[serde(rename = "complete")]
    Complete,
}

impl OrderStatus {
    pub fn as_vec() -> Vec<OrderStatus> {
        vec![
            OrderStatus::PaymentAwaited,
            OrderStatus::Paid,
            OrderStatus::InProcessing,
            OrderStatus::Cancelled,
            OrderStatus::Sent,
            OrderStatus::Delivered,
            OrderStatus::Received,
            OrderStatus::Complete,
        ]
    }
}
