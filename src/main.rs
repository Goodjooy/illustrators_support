/**
 * @Author: Your name
 * @Date:   2021-12-01 18:00:02
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 18:40:17
 */
use crate::controllers::{file_server::FileServerController, illustrator::IllustratorController};
use std::collections::HashMap;

use controllers::{admin::AdminController, user::UserController, Controller};
use database::Database;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use utils::{
    lifetime_hashmap::LifeTimeHashMap,
    mid_wares::{auth_switch::AuthSwitch, cors::Cors, net_logging::NetLogger}, config::Config, cors_handle,
};
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

    let database = Database::connect_db(&config.database)
        .await
        .expect("Can not connect to database");
    if let Some(cfig) = &config.invite_codes {
        database
            .add_default_code(cfig.clone())
            .await
            .expect("Add config invite code failure");
    }
    // app start
    rocket::build()
        // attached midware
        .attach(Cors)
        .attach(NetLogger)
        .attach(AuthSwitch::new())
        // golbal manage vars
        .manage(database)
        .manage(config.clone())
        .manage(std::sync::Mutex::new(HashMap::<String, i64>::new()))
        .manage(LifeTimeHashMap::<String, i64>::new())
        // rounts
        .mount("/", rocket::routes![cors_handle])
        .mount(FileServerController::base(), FileServerController::routes())
        .mount(UserController::base(), UserController::routes())
        .mount(
            IllustratorController::base(),
            IllustratorController::routes(),
        )
        .mount(AdminController::base(), AdminController::routes())
        //err catch
        .register("/", catchers![utils::catch])
}
