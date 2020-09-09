use chrono;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub first: String,
    pub last: String,
    pub created: chrono::NaiveDateTime,
    pub deleted: Option<chrono::NaiveDateTime>,
    pub password: Vec<u8>,
    pub salt: Vec<u8>,
    pub num_iterations: i32,
    pub owner: bool,
}
