use crate::customer_entity::CustomerEntity;
use barista_application::domain::customer::Customer;

#[derive(Debug, Clone)]
pub struct CustomerMapper {}

impl CustomerMapper {
    pub fn map_to_domain_entity(&self, entity: CustomerEntity) -> Customer {
        Customer {
            id: entity.id,
            name: entity.name,
        }
    }
}
