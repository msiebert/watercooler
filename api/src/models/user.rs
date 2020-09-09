use chrono;
use serde::Serialize;

#[derive(Queryable, Serialize)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first: String,
    pub last: String,
    pub created: chrono::NaiveDateTime,
    pub deleted: Option<chrono::NaiveDateTime>,
    #[serde(skip_serializing)]
    pub password: Vec<u8>,
    #[serde(skip_serializing)]
    pub salt: Vec<u8>,
    #[serde(skip_serializing)]
    pub num_iterations: i32,
    pub owner: bool,
}
