use edgedb_derive::Queryable;
use edgedb_protocol::model::LocalDatetime;

#[derive(Queryable, Debug)]
pub struct User {
    pub username: String,
    pub unique_id: String,
    pub display_name: String,
}

#[derive(Queryable, Debug)]
pub struct ErrorLog {
    pub log: String,
    pub created_local: LocalDatetime,
    pub level: Option<String>,
    pub channel_name: Option<String>,
}