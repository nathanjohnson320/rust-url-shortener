// @generated automatically by Diesel CLI.

diesel::table! {
  urls (id) {
      id -> Int4,
      long_url -> Varchar,
      short_url -> Varchar,
  }
}
