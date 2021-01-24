use crate::domain::coffee::Coffee;
use serde::{Deserialize, Serialize};

#[derive(Debug, Eq, PartialEq, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: i32,
    pub customer_id: i32,
    pub coffee: Coffee,
    pub price: i32,
}

impl Order {
    pub fn new_order(id: i32, customer_id: i32, price: i32, coffee: Coffee) -> Self {
        Self {
            id,
            customer_id,
            price,
            coffee,
        }
    }
}
