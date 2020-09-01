use chrono;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first: String,
    pub last: String,
    pub created: chrono::NaiveDateTime,
    pub deleted: Option<chrono::NaiveDateTime>,
    pub owner: bool,
}
