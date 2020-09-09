use crate::models::user::User;
use crate::schema::users;
use diesel::dsl::count;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::result::Error;
use ring::error::Unspecified;
use ring::rand::SecureRandom;
use ring::{digest, pbkdf2, rand};
use std::num::NonZeroU32;

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub first: &'a str,
    pub last: &'a str,
    pub password: &'a Vec<u8>,
    pub salt: &'a Vec<u8>,
    pub num_iterations: i32,
    pub owner: bool,
}

pub fn has_existing_users(conn: &PgConnection) -> Result<bool, Error> {
    use users::dsl::id;

    users::table
        .select(count(id))
        .first(conn)
        .map(|num: i64| num > 0)
}

pub fn create(
    conn: &PgConnection,
    email: &str,
    first: &str,
    last: &str,
    password: &str,
    owner: bool,
) -> Result<User, Unspecified> {
    let num_iterations: i32 = 100_000;
    gen_salt().and_then(|salt| {
        let hashed_password: Vec<u8> = hash_password(
            password,
            &salt,
            NonZeroU32::new(num_iterations as u32).unwrap(),
        )
        .iter()
        .cloned()
        .collect();

        let salt_vec: Vec<u8> = salt.iter().cloned().collect();

        let new_user = NewUser {
            email,
            first,
            last,
            password: &hashed_password,
            salt: &salt_vec,
            num_iterations,
            owner,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .get_result(conn)
            .map_err(|_| Unspecified)
    })
}

fn gen_salt() -> Result<[u8; digest::SHA512_OUTPUT_LEN], Unspecified> {
    let rng = rand::SystemRandom::new();
    let mut salt = [0u8; digest::SHA512_OUTPUT_LEN];
    rng.fill(&mut salt).map(|_| salt)
}

fn hash_password(
    password: &str,
    salt: &[u8; digest::SHA512_OUTPUT_LEN],
    num_iterations: NonZeroU32,
) -> [u8; digest::SHA512_OUTPUT_LEN] {
    let mut pbkdf2_hash = [0u8; digest::SHA512_OUTPUT_LEN];
    pbkdf2::derive(
        pbkdf2::PBKDF2_HMAC_SHA512,
        num_iterations,
        salt,
        password.as_bytes(),
        &mut pbkdf2_hash,
    );
    pbkdf2_hash
}
