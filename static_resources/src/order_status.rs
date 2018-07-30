use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq, DieselTypes)]
#[graphql(name = "OrderState", description = "Current order status")]
pub enum OrderState {
    #[graphql(description = "State set on order creation.")]
    #[serde(rename = "new")]
    New,

    #[graphql(description = "State set on order wallet creation.")]
    #[serde(rename = "payment_awaited")]
    PaymentAwaited,

    #[graphql(description = "State set on user's transaction appeared in blockchain, but is not included.")]
    #[serde(rename = "transaction_pending")]
    TransactionPending,

    #[graphql(description = "Set after price timeout has passed. Amount recalculation needed.")]
    #[serde(rename = "amount_expired")]
    AmountExpired,

    #[graphql(description = "Set after payment is accepted by blockchain by request of billing")]
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

impl FromStr for OrderState {
    type Err = Box<Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "New" => OrderState::New,
            "Payment Awaited" => OrderState::PaymentAwaited,
            "Transaction pending" => OrderState::TransactionPending,
            "Amount expired" => OrderState::AmountExpired,
            "Paid" => OrderState::Paid,
            "In processing" => OrderState::InProcessing,
            "Cancelled" => OrderState::Cancelled,
            "Sent" => OrderState::Sent,
            "Delivered" => OrderState::Delivered,
            "Received" => OrderState::Received,
            "Complete" => OrderState::Complete,
            other => {
                return Err(format!("Unrecognized enum variant: {}", other).to_string().into());
            }
        })
    }
}

impl Display for OrderState {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        use self::OrderState::*;

        write!(
            f,
            "{}",
            match self {
                New => "New",
                PaymentAwaited => "Payment Awaited",
                TransactionPending => "Transaction pending",
                AmountExpired => "Amount expired",
                Paid => "Paid",
                InProcessing => "In processing",
                Cancelled => "Cancelled",
                Sent => "Sent",
                Delivered => "Delivered",
                Received => "Received",
                Complete => "Complete",
            }
        )
    }
}

impl OrderState {
    pub fn as_vec() -> Vec<OrderState> {
        vec![
            OrderState::New,
            OrderState::PaymentAwaited,
            OrderState::TransactionPending,
            OrderState::AmountExpired,
            OrderState::Paid,
            OrderState::InProcessing,
            OrderState::Cancelled,
            OrderState::Sent,
            OrderState::Delivered,
            OrderState::Received,
            OrderState::Complete,
        ]
    }
}
