#[derive(GraphQLEnum, Deserialize, Serialize, Debug, Clone, PartialEq)]
#[graphql(name = "OrderStatus", description = "Current order status")]
pub enum OrderStatus {
    #[graphql(description = "State set on order creation.")]
    #[serde(rename = "payment_awaited")]
    PaimentAwaited,

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
            OrderStatus::PaimentAwaited,
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

mod diesel_impl {
    use diesel::expression::bound::Bound;
    use diesel::expression::AsExpression;
    use diesel::pg::Pg;
    use diesel::row::Row;
    use diesel::serialize::Output;
    use diesel::sql_types::VarChar;
    use diesel::types::{FromSqlRow, IsNull, NotNull, SingleValue, ToSql};
    use diesel::Queryable;
    use std::error::Error;
    use std::io::Write;
    use std::str;

    use super::OrderStatus;

    impl NotNull for OrderStatus {}
    impl SingleValue for OrderStatus {}

    impl FromSqlRow<VarChar, Pg> for OrderStatus {
        fn build_from_row<R: Row<Pg>>(row: &mut R) -> Result<Self, Box<Error + Send + Sync>> {
            match row.take() {
                Some(b"payment_awaited") => Ok(OrderStatus::PaimentAwaited),
                Some(b"paid") => Ok(OrderStatus::Paid),
                Some(b"in_processing") => Ok(OrderStatus::InProcessing),
                Some(b"cancelled") => Ok(OrderStatus::Cancelled),
                Some(b"sent") => Ok(OrderStatus::Sent),
                Some(b"delivered") => Ok(OrderStatus::Delivered),
                Some(b"received") => Ok(OrderStatus::Received),
                Some(b"complete") => Ok(OrderStatus::Complete),
                Some(value) => Err(format!(
                    "Unrecognized enum variant for OrderStatus: {}",
                    str::from_utf8(value).unwrap_or("unreadable value")
                ).into()),
                None => Err("Unexpected null for non-null column `state`".into()),
            }
        }
    }

    impl Queryable<VarChar, Pg> for OrderStatus {
        type Row = OrderStatus;
        fn build(row: Self::Row) -> Self {
            row
        }
    }

    impl ToSql<VarChar, Pg> for OrderStatus {
        fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> Result<IsNull, Box<Error + Send + Sync>> {
            match *self {
                OrderStatus::PaimentAwaited => out.write_all(b"payment_awaited")?,
                OrderStatus::Paid => out.write_all(b"paid")?,
                OrderStatus::InProcessing => out.write_all(b"in_processing")?,
                OrderStatus::Cancelled => out.write_all(b"cancelled")?,
                OrderStatus::Sent => out.write_all(b"sent")?,
                OrderStatus::Delivered => out.write_all(b"delivered")?,
                OrderStatus::Received => out.write_all(b"received")?,
                OrderStatus::Complete => out.write_all(b"complete")?,
            }
            Ok(IsNull::No)
        }
    }

    impl AsExpression<VarChar> for OrderStatus {
        type Expression = Bound<VarChar, OrderStatus>;
        fn as_expression(self) -> Self::Expression {
            Bound::new(self)
        }
    }

    impl<'a> AsExpression<VarChar> for &'a OrderStatus {
        type Expression = Bound<VarChar, &'a OrderStatus>;
        fn as_expression(self) -> Self::Expression {
            Bound::new(self)
        }
    }
}
