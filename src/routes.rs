/// Rocket routes
use crate::db::relationships::{create, has_relationship};
use crate::db::users::device_id_exists;
use crate::db::users::User;
use rocket_contrib::json::{Json, JsonValue};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pair {
    pub device_one: String,
    pub device_two: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub id: String,
    pub key: String,
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/", format = "json", data = "<message>")]
pub fn send(message: Json<Message>) -> JsonValue {
    // Check if the from device ID exists
    match device_id_exists(&message.from) {
        Ok(result) => {
            if result == false {
                return json!({ "status": "from: ".to_owned() + &message.from + " doesn't exist" });
            }
        }
        Err(err) => {
            let e = err.to_string();
            return json!({ "status": e.to_string()});
        }
    };

    // Check if the to device ID exists
    match device_id_exists(&message.to) {
        Ok(result) => {
            if result == false {
                return json!({ "status": "to: ".to_owned() + &message.to + " doesn't exist" });
            }
        }
        Err(err) => {
            let e = err.to_string();
            return json!({ "status": e.to_string()});
        }
    };

    // Check if from has a relationship with to
    match has_relationship(&message.from, &message.to) {
        Ok(result) => {
            if result == false {
                return json!({ "status": message.from.to_string() + " has not relationship to "  + &message.to });
            }
        }
        Err(err) => {
            let e = err.to_string();
            return json!({ "status": e.to_string()});
        }
    };

    println!("{:#?}", message);
    return json!({ "status": "ok" });
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/", format = "json", data = "<pair>")]
pub fn pair(pair: Json<Pair>) -> JsonValue {
    println!("{:?}", pair);
    // Check if device_one exists
    match device_id_exists(&pair.device_one) {
        Ok(result) => {
            if result == false {
                return json!({ "status": "device_one: ".to_owned() + &pair.device_one + " doesn't exist" });
            }
        }
        Err(err) => {
            let e = err.to_string();
            return json!({ "status": e.to_string()});
        }
    };

    // check if device_two exists
    match device_id_exists(&pair.device_two) {
        Ok(result) => {
            if result == false {
                return json!({ "status": "device_two: ".to_owned() + &pair.device_two + " doesn't exist" });
            }
        }
        Err(err) => {
            let e = err.to_string();
            return json!({ "status": e.to_string()});
        }
    };

    // If both exist, then add them into the relationship database
    match create(&pair.device_one, &pair.device_two) {
        Ok(result) => {
            if result == true {
                return json!({"status":"ok"});
            } else {
                return json!({"status":"could not find devices"});
            }
        }
        Err(e) => json!({"status":e.to_string()}),
    }
}

// TODO: This example can be improved by using `route` with multiple HTTP verbs.
#[post("/", format = "json", data = "<request_data>")]
pub fn register(mut request_data: Json<User>) -> Result<Json<User>, JsonValue> {
    match register_do(&mut request_data) {
        Ok(_) => Ok(request_data),
        Err(e) => Err(json!({ "status": e.to_string() })),
    }
}

// performs the registration
pub fn register_do(user_data: &mut User) -> Result<i32, String> {
    // Generate FNV from token
    match user_data.generate_id() {
        Ok(_) => {
            // Check if the user exists, if he does, then return
            let exists = match user_data.exists() {
                Ok(exists) => exists,
                Err(e) => {
                    let err = e.to_string();
                    return Err(err.to_string());
                }
            };

            // If the user record doesn't exist, then insert it
            if exists == false {
                match user_data.insert() {
                    Ok(_) => println!("fine"),
                    Err(e) => println!("{:?}", e),
                };
            } else {
                return Err("user already exists".to_string());
            }
        }
        Err(e) => {
            let err = e.to_string();
            return Err(err.to_string());
        }
    };

    println!("{:#?}", user_data);

    Ok(0)
}

#[get("/<id>", format = "json")]
pub fn get_key(id: String) -> Result<Json<Key>, JsonValue> {
    // Get the user's key
    match User::get_key(&id) {
        Ok(result) => Ok(Json(Key {
            id: id,
            key: result,
        })),
        Err(e) => {
            println!("{:?}", e);
            return Err(not_found());
        }
    }
}

#[catch(404)]
pub fn not_found() -> JsonValue {
    json!({
        "status": "error",
        "reason": "Resource was not found."
    })
}
