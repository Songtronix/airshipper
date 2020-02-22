#![feature(proc_macro_hygiene, decl_macro)]
use rocket::*;

pub mod config;
mod db;
mod error;
mod guards;
mod models;
mod params;
mod routes;
mod webhook;

use crate::error::ServerError;
use config::ServerConfig;

pub type Result<T> = std::result::Result<T, ServerError>;
pub use db::Database;

lazy_static::lazy_static! {
    /// Contains all configuration needed.
    pub static ref CONFIG: ServerConfig = {
            match ServerConfig::load() {
                Ok(x) => x,
                Err(e) => { println!("Failed to load config: {:?}", e); std::process::exit(-1); },
            }
    };
}

pub fn rocket() -> Result<rocket::Rocket> {
    // Base of the config and attach everything else
    Ok(CONFIG
        .rocket()
        .manage(db::init()?)
        .mount(
            "/",
            routes![
                routes::gitlab::post_pipeline_update,
                routes::user::index,
                routes::user::robots,
                routes::user::favicon,
                routes::api::version,
                routes::api::channel_version,
                routes::api::download,
                routes::api::channel_download,
            ],
        )
        .register(catchers![
            routes::catchers::not_found,
            routes::catchers::internal_error
        ]))
}
