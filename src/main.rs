use crate::controllers::illustrator::IllustratorController;
use std::collections::HashMap;

use controllers::{admin::AdminController, user::UserController, Controller};
use database::Database;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use rocket::fs::FileServer;
use utils::{config::Config, cors::Cors};

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
    // load config
    let config: Config = Figment::new()
        .merge(Toml::file("./Config.toml"))
        .extract()
        .expect("Load Config file Failure");

    // create save dir
    let save_path = &config.consts.save_dir;
    std::fs::create_dir_all(save_path).expect("Failure create save dir, please create it manel");

    // app start
    rocket::build()
        // attached midware
        .attach(Cors)
        // golbal manage vars
        .manage(
            Database::connect_db(&config.database)
                .await
                .expect("Can not connect to database"),
        )
        .manage(config.clone())
        .manage(std::sync::Mutex::new(HashMap::<String, i64>::new()))
        // rounts
        .mount("/images", FileServer::from(save_path))
        .mount(UserController::base(), UserController::routes())
        .mount(
            IllustratorController::base(),
            IllustratorController::routes(),
        )
        .mount(AdminController::base(), AdminController::routes())
}
