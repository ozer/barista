use crate::order_mapper::OrderMapper;
use crate::order_repository::OrderRepository;
use anyhow::Result;
use async_trait::async_trait;
use barista_application::application::port::outgoing::find_order_port::FindOrderPort;
use barista_application::{
    application::port::outgoing::save_order_port::SaveOrderPort,
    domain::{coffee::Coffee, order::Order},
};
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct OrderPersistenceAdapter {
    order_repository: OrderRepository,
    order_mapper: OrderMapper,
}

impl OrderPersistenceAdapter {
    pub fn new(pool: PgPool) -> Self {
        Self {
            order_mapper: OrderMapper {},
            order_repository: OrderRepository::new(pool),
        }
    }
}

#[async_trait]
impl SaveOrderPort for OrderPersistenceAdapter {
    async fn save_order(&self, coffee: Coffee, customer_id: i32) -> Result<Order> {
        let item = self
            .order_repository
            .save_order(
                coffee.coffee_type.to_string(),
                customer_id,
                coffee.get_price(),
            )
            .await?;

        let order = self.order_mapper.map_to_domain_entity(item);

        Ok(order)
    }
}

#[async_trait]
impl FindOrderPort for OrderPersistenceAdapter {
    async fn find_order(&self, order_id: i32) -> Result<Option<Order>> {
        match self.order_repository.find_order_by_id(order_id).await? {
            Some(order_entity) => {
                let order = self.order_mapper.map_to_domain_entity(order_entity);
                Ok(Some(order))
            }
            None => Ok(None),
        }
    }
}
