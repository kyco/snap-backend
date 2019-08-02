use crate::DATABASE;
use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

// Creates the database in case it doesn't exist
pub fn create() -> Result<()> {
    let conn = Connection::open(DATABASE.to_owned())?;

    conn.execute(
        "create table if not exists users (
             id integer primary key,
             device_id text not null,
             public_key text not null,
             token text not null,
             platform text not null
         )",
        NO_PARAMS,
    )?;

    conn.execute(
        "create table if not exists relationships (
             id integer primary key,
             device_one text not null,
             device_two text not null,
             pairing intefer not null
         )",
        NO_PARAMS,
    )?;

    Ok(())
}
