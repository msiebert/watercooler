use crate::db;

use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct NewUser {
    email: String,
}

#[post("/users/first", format = "json", data = "<new_user>")]
pub fn first_user(new_user: Json<NewUser>, conn: db::Conn) -> JsonValue {
    json!({
        "test": new_user.email,
    })
}
