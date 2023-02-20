use diesel::prelude::*;
use crate::schema::configs;

#[derive(Insertable)]
#[diesel(table_name = configs)]
pub struct NewConfig {
    pub target: String,
    pub remote: String,
}

#[derive(Queryable, AsChangeset)]
pub struct Config {
    pub id: i32,
    pub target: String,
    pub remote: String,
}


