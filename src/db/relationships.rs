// Check if two device_id's have a relationship
// true: They do
// false: They don't
use crate::DATABASE;
use rusqlite::{Connection, Result};

#[derive(Debug)]
pub struct Relationship {
    pub id: i32,
    pub device_one: String,
    pub device_two: String,
    pub pairing: i32,
}

// checks if relationship exists
// true: if one does
// false: if one does not
pub fn has_relationship(from: &String, to: &String) -> Result<bool> {
    let mut count = 0;
    let conn = Connection::open(DATABASE.to_owned())?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM relationships WHERE pairing != 0 AND ((device_one = ?1 AND device_two = ?2) OR (device_one = ?2 AND device_two = ?1))")?;
    stmt.query_row(&[from, to], |row| Ok(count = row.get(0)?))?;
    if count > 0 {
        return Ok(true);
    }
    return Ok(false);
}

// gets the realtionship from the db
pub fn get(device_one: &String, device_two: &String) -> Result<Relationship> {
    let conn = Connection::open(DATABASE.to_owned())?;
    let mut stmt = conn.prepare("SELECT id, device_one, device_two, pairing FROM relationships WHERE (device_one = ?1 AND device_two = ?2)")?;
    let rel = stmt.query_row(&[device_one, device_two], |row| {
        Ok(Relationship {
            id: row.get(0)?,
            device_one: row.get(1)?,
            device_two: row.get(2)?,
            pairing: row.get(3)?,
        })
    })?;

    return Ok(rel);
}

// creates a relationship between two devices
pub fn create(device_one: &String, device_two: &String) -> Result<bool> {
    // if a relationship exists, then return with an ok
    let exists = has_relationship(device_one, device_two)?;
    if exists == true {
        println!(
            "A relationship between {:?} and {:?} already exists",
            device_one, device_two
        );
        return Ok(true);
    }
    let conn = Connection::open(DATABASE.to_owned())?;

    let dev_one = get(device_one, device_two);
    let dev_two = get(device_two, device_one);

    println!("dev_one: {:?}", dev_one.is_err());
    println!("dev_two: {:?}", dev_two.is_err());

    // If no pairing has occured
    if dev_one.is_err() == true && dev_two.is_err() == true {
        // Insert a new record into the database
        let mut stmt = conn.prepare(
            "INSERT INTO relationships (device_one, device_two, pairing) VALUES (?1, ?2, 0)",
        )?;
        stmt.execute(&[device_one, device_two])?;
        return Ok(true);
    }
    // if dev_one exists, use it's ID to increment the pairing to 1
    if dev_one.is_err() == true && dev_two.is_err() == false {
        println!("Before stmt");
        let mut stmt = conn.prepare("UPDATE relationships SET pairing = 1 WHERE id = ?1")?;
        println!("after stmt");
        stmt.execute(&[&dev_two?.id])?;
        println!("after execute");
    } else {
        return Ok(true);
    }

    Ok(true)
}
