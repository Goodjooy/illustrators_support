/**
 * @Author: Your name
 * @Date:   2021-12-01 18:00:02
 * @Last Modified by:   Your name
 * @Last Modified time: 2021-12-11 14:14:34
 */
use crate::controllers::{file_server::FileServerController, illustrator::IllustratorController};
use std::collections::HashMap;

use controllers::{admin::AdminController, user::UserController, Controller};
use database::Database;
use figment::{
    providers::{Format, Toml},
    Figment,
};
use std::io::Write;
use utils::{config::Config, cors::Cors, cors_handle, lifetime_hashmap::LifeTimeHashMap, auth_switch::AuthSwitch, net_logging::NetLogger};
use chrono::Local;
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
    env_logger::Builder::new()
    .format(|buf,record|{
        writeln!(
            buf,"{} [{}] - {}",
            Local::now().format("%Y-%m-%dT%H:%M:%S"),
            record.level(),
            record.args()
        )
    })
    .filter_level(log::LevelFilter::Info)

    .init();

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
