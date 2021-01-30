use anyhow::{Error, Result};
use sqlx::postgres::PgRow;
use sqlx::{query, query_as, PgPool, Row};

use crate::order_entity::OrderEntity;
use crate::persistence_exception::PersistenceException;

#[derive(Debug, Clone)]
pub struct OrderRepository {
    pool: PgPool,
}

impl OrderRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn save_order(
        &self,
        coffee_type: String,
        customer_id: i32,
        price: i32,
    ) -> Result<OrderEntity> {
        let row: (i32, String, i32, i32) = query(
            "INSERT INTO orders (id, coffee_type, customer_id, price) VALUES (nextval('seq_orders'), $1, $2, $3) RETURNING (id, coffee_type, customer_id, price)",
        ).bind(coffee_type).bind(customer_id).bind(price)
            .map(|row: PgRow| {
                let order_row: (i32, String, i32, i32) = row.get(0);
                order_row
            })
            .fetch_one(&self.pool)
            .await?;

        let order_entity = OrderEntity {
            id: row.0,
            coffee_type: row.1,
            customer_id: row.2,
            price: row.3,
        };

        Ok(order_entity)
    }

    pub async fn find_order_by_id(&self, id: i32) -> Result<Option<OrderEntity>> {
        // let order_entity: Option<OrderEntity> =
        //     match query("SELECT id, coffee_type, customer_id, price FROM orders WHERE id = $1")
        //         .bind(id)
        //         .map(|row: PgRow| {
        //             let id: Option<i32> = row.get("id");
        //             let coffee_type: Option<String> = row.get("coffee_type");
        //             let customer_id: Option<i32> = row.get("customer_id");
        //             let price: Option<i32> = row.get("price");
        //             let order_entity = match (id, coffee_type, customer_id, price) {
        //                 (Some(id), Some(coffee_type), Some(customer_id), Some(price)) => {
        //                     Some(OrderEntity {
        //                         id,
        //                         coffee_type,
        //                         customer_id,
        //                         price,
        //                     })
        //                 }
        //                 _ => None,
        //             };
        //             order_entity
        //         })
        //         .fetch_optional(&self.pool)
        //         .await?
        //     {
        //         Some(order_entity) => order_entity,
        //         None => None,
        //     };

        let order_entity = query_as::<_, OrderEntity>(
            "SELECT id, coffee_type, customer_id, price FROM orders WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| {
            Error::from(PersistenceException::DatabaseError(format!(
                "DatabaseError!",
            )))
        });

        order_entity
    }
}
