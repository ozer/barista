use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum PersistenceException {
    #[error("`{0}`")]
    DatabaseError(String),
}
