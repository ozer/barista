use crate::domain::customer::Customer;
use anyhow::Result;
use async_trait::async_trait;

#[cfg(test)]
use mockall::mock;

#[async_trait]
pub trait SaveCustomerPort {
    async fn save_customer(&self, name: String) -> Result<Customer>;
}

#[cfg(test)]
mock! {
    pub SaveCustomerPort {}

    #[async_trait]
    impl SaveCustomerPort for SaveCustomer {
        async fn save_customer(&self, name: String) -> Result<Customer>;
    }
}
