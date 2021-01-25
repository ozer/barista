use anyhow::Result;
use async_trait::async_trait;

use crate::domain::coffee::Coffee;
use crate::domain::order::Order;

#[cfg(test)]
use mockall::mock;

#[async_trait]
pub trait SaveOrderPort {
    async fn save_order(&self, coffee: Coffee, customer_id: i32) -> Result<Order>;
}

#[cfg(test)]
mock! {
    pub SaveOrderPort {}

    #[async_trait]
    impl SaveOrderPort for SaveOrder {
        async fn save_order(&self, coffee: Coffee, customer_id: i32) -> Result<Order>;
    }
}
