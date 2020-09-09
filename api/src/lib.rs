#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate validator_derive;
extern crate validator;

use rocket_contrib::templates::Template;

mod db;
mod models;
mod routes;
mod schema;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite()
        .mount(
            "/",
            routes![
                routes::index,
                routes::users::has_first_user,
                routes::users::first_user,
            ],
        )
        .attach(db::Conn::fairing())
        .attach(Template::fairing())
}
