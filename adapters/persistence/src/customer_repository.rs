use crate::customer_entity::CustomerEntity;
use anyhow::Result;
use sqlx::postgres::PgRow;
use sqlx::{query, PgPool, Row};

#[derive(Debug, Clone)]
pub struct CustomerRepository {
    pool: PgPool,
}

impl CustomerRepository {
    #[allow(dead_code)]
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_customer(&self, name: String) -> Result<CustomerEntity> {
        let row: (i32, String) = query(
            "INSERT INTO customers (id, name) VALUES (nextval('seq_customers'), $1) RETURNING (id, name)",
        ).bind(name)
            .map(|row: PgRow| {
                let customer_row: (i32, String) = row.get(0);
                customer_row
            })
            .fetch_one(&self.pool)
            .await?;

        let customer_entity = CustomerEntity {
            id: row.0,
            name: row.1,
        };

        Ok(customer_entity)
    }
}
