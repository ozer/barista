use sqlx::FromRow;

#[derive(FromRow, Debug, Eq, PartialEq, Clone)]
pub struct OrderEntity {
    pub id: i32,
    pub coffee_type: String,
    pub customer_id: i32,
    pub price: i32,
}

impl OrderEntity {
    #[allow(dead_code)]
    pub fn new(id: i32, coffee_type: String, customer_id: i32, price: i32) -> Self {
        Self {
            id,
            coffee_type,
            customer_id,
            price,
        }
    }
}
