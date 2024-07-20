use crate::models::{InsertUrl, Url};
use crate::schema::urls;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use nanoid::nanoid;

pub fn create_url(conn: &mut PgConnection, long_url: &str) -> Url {
    let new_url = InsertUrl {
        short_url: nanoid!(10),
        long_url: long_url.to_string(),
    };

    diesel::insert_into(urls::table)
        .values(&new_url)
        .returning(Url::as_returning())
        .get_result(conn)
        .expect("Error saving new url")
}

pub fn get_urls(conn: &mut PgConnection) -> Vec<Url> {
    urls::table
        .select(Url::as_select())
        .load::<Url>(conn)
        .expect("Error loading urls")
}

pub fn delete_url(conn: &mut PgConnection, id: &i32) -> Url {
    diesel::delete(urls::table.filter(urls::id.eq(id)))
        .returning(Url::as_returning())
        .get_result(conn)
        .expect("Error deleting url")
}
