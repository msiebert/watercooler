#![feature(proc_macro_hygiene, decl_macro)]

use rocket_contrib::templates::Template;
use std::collections::HashMap;

#[macro_use]
extern crate rocket;

#[get("/")]
fn index() -> Template {
    let context: HashMap<String, String> = HashMap::new();
    Template::render("index", &context)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index])
        .attach(Template::fairing())
        .launch();
}
