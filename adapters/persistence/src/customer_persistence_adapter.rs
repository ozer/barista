use crate::customer_mapper::CustomerMapper;
use crate::customer_repository::CustomerRepository;
use anyhow::Result;
use async_trait::async_trait;
use barista_application::application::port::outgoing::save_customer_port::SaveCustomerPort;
use barista_application::domain::customer::Customer;
use sqlx::PgPool;

#[derive(Debug, Clone)]
pub struct CustomerPersistenceAdapter {
    customer_repository: CustomerRepository,
    customer_mapper: CustomerMapper,
}

impl CustomerPersistenceAdapter {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self {
            customer_repository: CustomerRepository::new(pool),
            customer_mapper: CustomerMapper {},
        }
    }
}

#[async_trait]
impl SaveCustomerPort for CustomerPersistenceAdapter {
    async fn save_customer(&self, name: String) -> Result<Customer> {
        let customer_entity = self.customer_repository.save_customer(name).await?;
        let customer = self.customer_mapper.map_to_domain_entity(customer_entity);
        Ok(customer)
    }
}
