use crate::domain::coffee::Coffee;
use crate::domain::coffee_shop_exception::CoffeeShopException;
use crate::domain::order::Order;
use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
use mockall::predicate::*;

use crate::application::port::outgoing::find_order_port::FindOrderPort;
use crate::application::port::outgoing::save_customer_port::SaveCustomerPort;
use crate::application::port::{
    incoming::order_coffee_use_case::{OrderCoffeeCommand, OrderCoffeeUseCase},
    outgoing::save_order_port::SaveOrderPort,
};

pub struct OrderCoffeeService {
    order_coffee: Box<dyn SaveOrderPort + Send + Sync>,
    save_customer: Box<dyn SaveCustomerPort + Send + Sync>,
    find_order: Box<dyn FindOrderPort + Send + Sync>,
}

impl OrderCoffeeService {
    pub fn new(
        order_coffee: Box<dyn SaveOrderPort + Send + Sync>,
        save_customer: Box<dyn SaveCustomerPort + Send + Sync>,
        find_order: Box<dyn FindOrderPort + Send + Sync>,
    ) -> Self {
        Self {
            order_coffee,
            save_customer,
            find_order,
        }
    }
}

#[async_trait]
impl OrderCoffeeUseCase for OrderCoffeeService {
    async fn order_coffee(&self, command: &OrderCoffeeCommand) -> Result<Order> {
        let coffee = Coffee {
            coffee_type: command.coffee_type,
        };

        let customer_name = command.customer_name.clone();
        let customer = self.save_customer.save_customer(customer_name).await?;
        let order = self.order_coffee.save_order(coffee, customer.id).await?;

        Ok(order)
    }

    async fn get_order_by_id(&self, order_id: i32) -> Result<Option<Order>> {
        let order = match self.find_order.find_order(order_id).await? {
            Some(order) => Some(order),
            None => Err(CoffeeShopException::OrderNotFound(order_id))?,
        };

        Ok(order)
    }
}

#[cfg(test)]
#[allow(unused_imports)]
pub mod tests {
    use super::*;
    use crate::application::port::outgoing::find_order_port::*;
    use crate::application::port::outgoing::save_customer_port::*;
    use crate::application::port::outgoing::save_order_port::*;
    use crate::domain::coffee::CoffeeType;
    use crate::domain::customer::Customer;

    #[async_std::test]
    async fn test_order_coffee() {
        let command = OrderCoffeeCommand {
            coffee_type: CoffeeType::Espresso,
            customer_name: String::from("ozer"),
        };

        let coffee = Coffee {
            coffee_type: command.coffee_type.clone(),
        };

        let mut mock_save_order = MockSaveOrderPort::new();
        let mut mock_save_customer = MockSaveCustomerPort::new();
        mock_save_order
            .expect_save_order()
            .with(eq(coffee), eq(1))
            .returning(move |coffee, customer_id| {
                Ok(Order {
                    id: 1,
                    customer_id,
                    coffee,
                    price: 5,
                })
            });

        mock_save_customer
            .expect_save_customer()
            .with(eq(String::from("ozer")))
            .times(1)
            .returning(move |name| Ok(Customer { id: 1, name }));

        let service = OrderCoffeeService::new(
            Box::new(mock_save_order),
            Box::new(mock_save_customer),
            Box::new(MockFindOrderPort::new()),
        );

        let order = service.order_coffee(&command).await.unwrap();

        assert_eq!(1, order.id);
        assert_eq!(1, order.customer_id);
        assert_eq!(5, order.price);
        assert_eq!(CoffeeType::Espresso, order.coffee.coffee_type);
    }
}
