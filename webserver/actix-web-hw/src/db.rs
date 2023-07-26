use rusqlite::{Connection, Result};

use crate::types::Name;

pub fn ensure_table(conn: &Connection) -> Result<usize> {
    Ok(conn.execute(
        "CREATE TABLE greeted (
            name TEXT NOT NULL
         )",
        (),
    )?)
}

pub fn store_name(conn: &Connection, name: &Name) -> Result<usize> {
    Ok(conn.execute("INSERT INTO greeted (`name`) VALUES (?)", &[name])?)
}
