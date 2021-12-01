use database::connect_db;


#[macro_use]
extern crate rocket;
extern crate serde;
extern crate sea_orm;

mod entity;
mod database;
mod controllers;

#[rocket::launch]
async fn launch() -> _ {
    rocket::build()
    .manage(connect_db().await.expect("Can not connect to database"))
}