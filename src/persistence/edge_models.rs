use edgedb_derive::Queryable;
use edgedb_protocol::model::{LocalDatetime, Uuid};

#[derive(Queryable, Debug)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub unique_id: String,
    pub display_name: String,
}

#[derive(Queryable, Debug)]
pub struct ErrorLog {
    pub id: Uuid,
    pub log: String,
    pub created_local: LocalDatetime,
    pub level: Option<String>,
    pub channel_name: Option<String>,
}

#[derive(Queryable, Debug)]
pub struct ChannelMessage {
    pub id: Uuid,
    pub channel_id: Option<String>,
    pub author: Option<User>,
    pub is_bot: bool,
    pub message: String,
    pub created_local: LocalDatetime,
}
