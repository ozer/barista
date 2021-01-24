use crate::order_entity::OrderEntity;
use barista_application::domain::coffee::{Coffee, CoffeeType};
use barista_application::domain::order::Order;

#[derive(Debug, Clone)]
pub struct OrderMapper {}

impl OrderMapper {
    pub fn map_to_domain_entity(&self, order_entity: OrderEntity) -> Order {
        let coffee_type: CoffeeType = order_entity.coffee_type.parse().unwrap();
        let coffee = Coffee { coffee_type };
        Order {
            id: order_entity.id,
            coffee,
            customer_id: order_entity.customer_id,
            price: order_entity.price,
        }
    }
}
