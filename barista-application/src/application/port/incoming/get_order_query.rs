use crate::domain::order::Order;
use anyhow::Result;
use async_trait::async_trait;

#[async_trait]
pub trait GetOrderQuery {
    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>>;
}
