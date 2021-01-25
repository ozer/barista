use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum CoffeeShopException {
    #[error("Coffee Type: `{0}` is invalid")]
    InvalidCoffeeType(String),
    #[error("Order with id: `{0}` was not found")]
    OrderNotFound(i32),
}
