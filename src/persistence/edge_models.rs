use edgedb_derive::Queryable;
use edgedb_protocol::model::{Datetime, LocalDatetime};

#[derive(Queryable, Debug)]
pub struct User {
    username: String,
    discriminator: String,
    uniqueId: String,
}

#[derive(Queryable, Debug)]
pub struct ErrorLog {
    level: String,
    log: String,
    channel_name: String,
    created_local: LocalDatetime,
    created: Datetime
}