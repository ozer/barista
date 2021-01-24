use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct OrderCoffeeRequest {
    #[serde(rename = "customerName")]
    pub customer_name: String,
    #[serde(rename = "coffeeType")]
    pub coffee_type: String,
}

impl OrderCoffeeRequest {
    #[allow(dead_code)]
    pub fn new(customer_name: String, coffee_type: String) -> Self {
        Self {
            customer_name,
            coffee_type,
        }
    }
}
