use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Serialize, Deserialize, Debug, FromRow)]
pub struct Todo {
    pub id: i64,
    pub name: String,
    pub done: bool,
}