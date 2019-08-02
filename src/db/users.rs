use crate::DATABASE;
// use rusqlite::NO_PARAMS;
use rusqlite::{Connection, Result};

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Option<String>,
    pub public_key: String,
    pub token: String,
    pub platform: String,
}

// Implements user
impl User {
    // Creates a new instance of "User"
    pub fn new() -> User {
        User {
            id: Some(String::new()),
            public_key: String::new(),
            token: String::new(),
            platform: String::new(),
        }
    }

    // Creates a new instance of "User" with data
    pub fn from(
        id: Option<String>,
        public_key: &String,
        token: &String,
        platform: &String,
    ) -> User {
        User {
            id: id,
            public_key: public_key.to_string(),
            token: token.to_string(),
            platform: platform.to_string(),
        }
    }

    //get a user's data by ID
    pub fn get(id: &u32) -> Result<()> {
        let conn = Connection::open(DATABASE.to_owned())?;

        let mut stmt = conn.prepare(
            "SELECT device_id, public_key, token, platform FROM users WHERE device_id = ?1",
        )?;

        let user = stmt.query_row(&[&id], |row| {
            Ok(User {
                id: row.get(0)?,
                public_key: row.get(1)?,
                token: row.get(2)?,
                platform: row.get(3)?,
            })
        })?;

        println!("Found user {:?}", user);

        Ok(())
    }

    //get a user's data by public key
    pub fn get_by_public_key(&self) -> Result<()> {
        let conn = Connection::open(DATABASE.to_owned())?;

        let mut stmt = conn.prepare(
            "SELECT device_id, public_key, token, platform FROM users WHERE public_key = ?1",
        )?;

        let user = stmt.query_row(&[&self.public_key], |row| {
            Ok(User {
                id: row.get(0)?,
                public_key: row.get(1)?,
                token: row.get(2)?,
                platform: row.get(3)?,
            })
        })?;

        println!("Found user {:?}", user);

        Ok(())
    }

    // checks if the user exists
    // true: if the user does exist
    // false: if the user doesn't exist
    pub fn exists(&self) -> Result<bool> {
        let mut count = 0;
        let conn = Connection::open(DATABASE.to_owned())?;
        let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE public_key = ?1")?;
        stmt.query_row(&[&self.public_key], |row| Ok(count = row.get(0)?))?;
        if count > 0 {
            return Ok(true);
        }
        return Ok(false);
    }

    // get the users public key
    pub fn get_key(id: &String) -> Result<(String)> {
        let conn = Connection::open(DATABASE.to_owned())?;
        let mut stmt = conn.prepare(
            "SELECT public_key FROM users WHERE device_id = ?1 ORDER BY id DESC LIMIT 1",
        )?;
        Ok(stmt.query_row(&[&id], |row| row.get(0))?)
    }

    // insert a new user into the database
    pub fn insert(&self) -> Result<()> {
        let conn = Connection::open(DATABASE.to_owned())?;
        conn.execute(
            "INSERT INTO users (device_id, public_key, token, platform) values (?1, ?2, ?3, ?4)",
            &[
                self.id.as_ref().unwrap(),
                &self.public_key,
                &self.token,
                &self.platform,
            ],
        )?;

        Ok(())
    }

    // Generate a random ID for a user based on their public key
    pub fn generate_id(&mut self) -> std::result::Result<i32, &str> {
        let pk_length = self.public_key.chars().count();
        if pk_length < 6 {
            return Err("invalid public key");
        }
        use rand::Rng;
        let charset: &[u8] = self.public_key.as_bytes();
        let mut rng = rand::thread_rng();

        // BY default we assume that this key
        // already exists in the database until we're proven wrong
        let mut exists = true;

        // We check every iteration of the device_id
        // to see if it exists in the database already,
        // this ensures uniqueness
        while exists == true {
            let device_id: String = (0..crate::USER_ID_LENGTH)
                .map(|_| {
                    let idx = rng.gen_range(0, charset.len());
                    // This is safe because `idx` is in range of `charset`
                    char::from(unsafe { *charset.get_unchecked(idx) })
                })
                .collect();

            // check if it exists in the database
            match device_id_exists(&device_id) {
                Ok(result) => {
                    exists = result;
                    if result == false {
                        self.id = std::prelude::v1::Option::Some(device_id);
                    }
                }
                Err(_) => {
                    return Err("cannot connect to sqlite");
                }
            }
        }

        // If not, set and then return OK

        return Ok(0);
    }
}

// checks if the device_id exists
// true: if the device_id does exist
// false: if the device_id doesn't exist
pub fn device_id_exists(id: &String) -> Result<bool> {
    let mut count = 0;
    let conn = Connection::open(DATABASE.to_owned())?;
    let mut stmt = conn.prepare("SELECT COUNT(*) FROM users WHERE device_id = ?1")?;
    stmt.query_row(&[id], |row| Ok(count = row.get(0)?))?;
    if count > 0 {
        return Ok(true);
    }
    return Ok(false);
}
