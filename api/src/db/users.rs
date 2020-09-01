use crate::models::user::User;
use crate::schema::users;
use diesel::pg::PgConnection;
use diesel::prelude::*;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first: &'a str,
    pub last: &'a str,
    pub password: &'a str,
    pub owner: bool,
}

// pub fn create(
//     conn: &PgConnection,
//     email: &str,
//     first: &str,
//     last: &str,
//     password: &str,
//     owner: bool,
// ) -> Result<User, str> {
//     let new_user = NewUser {
//         email,
//         first,
//         last,
//         password,
//         owner,
//     };

//     diesel::insert_into(users::table)
//         .values(&new_user)
//         .get_result(conn)
// }
