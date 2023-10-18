use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct CreateTodo{
    pub name: String,
    pub done: bool
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UpdateTodo{
    pub id: i64,
    pub name: String,
    pub done: bool
}