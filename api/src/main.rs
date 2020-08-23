#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

use rocket_contrib::databases::diesel::PgConnection;
use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[get("/")]
fn index(_db_conn: DbConn) -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

#[database("watercooler")]
struct DbConn(PgConnection);

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(DbConn::fairing())
        .attach(Template::fairing())
        .launch();
}
