use sqlx::FromRow;

#[derive(FromRow, Debug, Eq, PartialEq, Clone)]
pub struct CustomerEntity {
    pub id: i32,
    pub name: String,
}

impl CustomerEntity {
    #[allow(dead_code)]
    pub fn new(id: i32, name: String) -> Self {
        Self { id, name }
    }
}
