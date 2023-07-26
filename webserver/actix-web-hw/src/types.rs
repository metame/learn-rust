use rusqlite::Connection;
use serde::Deserialize;
use std::sync::Mutex;

pub type Greeting = String;
pub type Name = String;

#[derive(Deserialize)]
pub struct GreetName {
    pub greeting: Greeting,
    pub name: Name,
}

pub type LockedConnection = Mutex<Connection>;

pub struct State {
    pub conn: LockedConnection,
}
