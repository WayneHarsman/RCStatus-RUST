use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use crate::models::{ Config, NewConfig };
use utils::error::Result;


pub fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn add_config(_config: &NewConfig) {
    use super::schema::configs::dsl::configs;

    let connection = &mut establish_connection();
    diesel::insert_into(configs)
        .values(_config)
        .execute(connection)
        .expect("Error saving new config");
}

pub fn delete_config(_config: &Config) {
    use super::schema::configs::dsl::*;

    let connection = &mut establish_connection();
    diesel::delete(configs.filter(target.like(&_config.target)))
        .execute(connection)
        .expect("Error deleting config");
}

pub fn retrieve_config(_target: &String) -> Option<Config> {
    use super::schema::configs::dsl::*;

    let connection = &mut establish_connection();

    let config = configs
        .filter(target.like(format!("{}%", _target)))
        .first::<Config>(connection);

    match config {
        Ok(config) => Some(config),
        Err(_) => None
    }
}

pub fn contains_config(_target_dir: &str) -> Result<bool> {
    use super::schema::configs::dsl::*;

    let connection = &mut establish_connection();
    let result = configs
        .filter(target.like(format!("{}%", _target_dir)))
        .first::<Config>(connection);

    if result.is_err() {
        return Ok(false)
    }

    Ok(true)
}