use crate::db;

use rocket::http::Status;
use rocket_contrib::json::{Json, JsonValue};
use serde::Deserialize;
use validator::Validate;

#[get("/users/first")]
pub fn has_first_user(conn: db::Conn) -> Result<JsonValue, Status> {
    db::users::has_existing_users(&conn)
        .map(|has_existing_users| {
            json!({
                "has_existing_users": has_existing_users,
            })
        })
        .map_err(|_error| Status::InternalServerError)
}

#[derive(Deserialize, Validate)]
pub struct NewUser {
    #[validate(email)]
    email: String,
    #[validate(length(min = 1))]
    first: String,
    #[validate(length(min = 1))]
    last: String,
    #[validate(length(min = 8))]
    password: String,
}

#[post("/users/first", format = "json", data = "<new_user>")]
pub fn first_user(new_user: Json<NewUser>, conn: db::Conn) -> Result<JsonValue, Status> {
    new_user
        .validate()
        .map_err(|_| Status::BadRequest)
        .and_then(|_| match db::users::has_existing_users(&conn) {
            Ok(has_existing_users) => {
                if !has_existing_users {
                    db::users::create(
                        &conn,
                        &new_user.email,
                        &new_user.first,
                        &new_user.last,
                        &new_user.password,
                        true,
                    )
                    .map_err(|_| Status::InternalServerError)
                    .map(|user| json!(user))
                } else {
                    Err(Status::Forbidden)
                }
            }
            Err(_) => Err(Status::InternalServerError),
        })
}
