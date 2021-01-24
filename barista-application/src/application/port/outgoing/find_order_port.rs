use crate::domain::order::Order;
use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
use mockall::mock;

#[async_trait]
pub trait FindOrderPort {
    async fn find_order(&self, order_id: i32) -> Result<Option<Order>>;
}

#[cfg(test)]
mock! {
    pub FindOrderPort {}

    #[async_trait]
    impl FindOrderPort for FindOrder {
        async fn find_order(&self, order_id: i32) -> Result<Option<Order>>;
    }
}
