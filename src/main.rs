use crate::controllers::illustrator::IllustratorController;
use std::{collections::HashMap};

use controllers::{user::UserController, Controller};
use database::Database;
use rocket::fs::FileServer;

#[macro_use]
extern crate rocket;
extern crate sea_orm;
extern crate serde;

mod controllers;
mod data_containers;
mod database;
mod entity;
mod utils;

#[rocket::launch]
async fn launch() -> _ {
    rocket::build()
        .manage(
            Database::connect_db()
                .await
                .expect("Can not connect to database"),
        )
        .manage(std::sync::Mutex::new(HashMap::<String, i64>::new()))
        .mount("/images", FileServer::from("./SAVES"))
        .mount(UserController::base(), UserController::routes())
        .mount(IllustratorController::base(), IllustratorController::routes())
}
