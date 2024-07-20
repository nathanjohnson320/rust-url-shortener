use crate::schema::urls;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Deserialize, Serialize, Insertable)]
#[diesel(table_name = urls)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Url {
    pub id: i32,
    pub short_url: String,
    pub long_url: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = urls)]
pub struct NewUrl {
    pub long_url: String,
}
#[derive(Insertable, Serialize, Deserialize)]
#[diesel(table_name = urls)]
pub struct InsertUrl {
    pub long_url: String,
    pub short_url: String,
}
