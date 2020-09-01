pub mod users;

#[database("watercooler")]
pub struct Conn(diesel::PgConnection);
