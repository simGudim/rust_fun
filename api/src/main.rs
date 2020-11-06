#![feature(plugin, const_fn, decl_macro, proc_macro_hygiene)]
#![allow(proc_macro_derive_resolution_fallback, unused_attributes)]
// #![plugin(rocket_codegen)]

#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate rocket;
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;


use dotenv::dotenv;
use std::env;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use routes::*;


mod schema;
mod models;
mod db;
mod static_files;
mod routes;

fn rocket() -> rocket::Rocket {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("set DATABASE_URL");
    // with this method the db has to open each time instead of maintaing a connection
    // let conn = PgConnection::establish(&database_url).unwrap();
    let pool = db::init_pool(database_url);
    rocket::ignite()
        .manage(pool)
        .mount(
            "/api/v1",
            routes![index, new, show, delete, author, update], 
        )
        .mount("/", routes![static_files::all, static_files::index])
        .register(catchers![not_found])
        
}

fn main() {
    rocket().launch();
}