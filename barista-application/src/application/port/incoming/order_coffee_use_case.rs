use crate::domain::coffee::CoffeeType;
use crate::domain::order::Order;
use anyhow::Result;
use async_trait::async_trait;

#[derive(Debug)]
pub struct OrderCoffeeCommand {
    pub customer_name: String,
    pub coffee_type: CoffeeType,
}

impl OrderCoffeeCommand {
    pub fn new(customer_name: String, coffee_type: CoffeeType) -> Self {
        Self {
            customer_name,
            coffee_type,
        }
    }
}

#[async_trait]
pub trait OrderCoffeeUseCase {
    async fn order_coffee(&self, command: &OrderCoffeeCommand) -> Result<Order>;
}
