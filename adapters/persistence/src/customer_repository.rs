use crate::customer_entity::CustomerEntity;
use crate::persistence_exception::PersistenceException;
use anyhow::{Error, Result};
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
        let row: Result<(i32, String)> = query(
            "INSERT INTO customers (id, name) VALUES (nextval('seq_customers'), $1) RETURNING (id, name)",
        ).bind(name)
            .map(|row: PgRow| {
                let customer_row: (i32, String) = row.get(0);
                customer_row
            })
            .fetch_one(&self.pool)
            .await
            .map_err(|_| {
                Error::from(PersistenceException::DatabaseError(format!(
                    "DatabaseError!",
                )))
            });

        match row {
            Ok(row) => {
                let customer_entity = CustomerEntity {
                    id: row.0,
                    name: row.1,
                };

                Ok(customer_entity)
            }
            Err(_) => Err(Error::from(PersistenceException::DatabaseError(
                String::from("Cannot parse customer entity from DB"),
            ))),
        }
    }
}
