/// Rocket routes
use crate::db::paired::{create, is_paired};
use crate::db::users::device_id_exists;
use crate::db::users::User;
use crate::json_generic::{JsonGeneric, JsonGenericCodes};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub contents: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Pairing {
    pub my_device: String,
    pub other_device: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub id: String,
    pub key: String,
}

// Send push notification to paired device
#[post("/", format = "json", data = "<message>")]
pub fn send(message: Json<Message>) -> Json<JsonGeneric> {
    // Check if the from device ID exists
    match device_id_exists(&message.from) {
        Ok(result) => {
            if result == false {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotFound,
                    "from: ".to_owned() + &message.from + " doesn't exist",
                );
            }
        }
        Err(err) => {
            return JsonGeneric::new_generic(err.to_string());
        }
    };

    // Check if the to device ID exists
    match device_id_exists(&message.to) {
        Ok(result) => {
            if result == false {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotFound,
                    "to: ".to_owned() + &message.to + " doesn't exist",
                );
            }
        }
        Err(err) => {
            return JsonGeneric::new_generic(err.to_string());
        }
    };

    // Check if from is paired with to
    match is_paired(&message.from, &message.to) {
        Ok(result) => {
            if result == false {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotPaired,
                    message.from.to_string() + " is not paired with " + &message.to,
                );
            }
        }
        Err(err) => {
            return JsonGeneric::new_generic(err.to_string());
        }
    };

    return JsonGeneric::new_ok("ok".to_string());
}

// Pair two devices
#[post("/", format = "json", data = "<pair>")]
pub fn pair(pair: Json<Pairing>) -> Json<JsonGeneric> {
    // Check if my_device exists
    match device_id_exists(&pair.my_device) {
        Ok(result) => {
            if result == false {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotFound,
                    "my_device: ".to_owned() + &pair.my_device + " doesn't exist",
                );
            }
        }
        Err(err) => {
            return JsonGeneric::new_generic(err.to_string());
        }
    };

    // check if other_device exists
    match device_id_exists(&pair.other_device) {
        Ok(result) => {
            if result == false {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotFound,
                    "other_device: ".to_owned() + &pair.other_device + " doesn't exist",
                );
            }
        }
        Err(err) => {
            return JsonGeneric::new_generic(err.to_string());
        }
    };

    // If both exist, then add them into the relationship database
    match create(&pair.my_device, &pair.other_device) {
        Ok(result) => {
            if result == true {
                return JsonGeneric::new_ok("ok".to_string());
            } else {
                return JsonGeneric::new_response(
                    JsonGenericCodes::NotFound,
                    "could not find devices".to_string(),
                );
            }
        }
        Err(e) => JsonGeneric::new_generic(e.to_string()),
    }
}

// Register a new device
#[post("/", format = "json", data = "<request_data>")]
pub fn register(mut request_data: Json<User>) -> Result<Json<User>, Json<JsonGeneric>> {
    match register_do(&mut request_data) {
        Ok(_) => Ok(request_data),
        Err(e) => Err(JsonGeneric::new_generic(e.to_string())),
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

    Ok(0)
}

// Get public key of paired device
#[post("/", format = "json", data = "<request_data>")]
pub fn get_key(request_data: Json<Pairing>) -> Result<Json<Key>, Json<JsonGeneric>> {
    // check if the devices are paired.
    // only paired devices are allowed to see each other's keys
    match is_paired(&request_data.my_device, &request_data.other_device) {
        Ok(result) => {
            if result == false {
                return Err(JsonGeneric::new_response(
                    JsonGenericCodes::NotPaired,
                    request_data.my_device.to_string()
                        + " is not paired with "
                        + &request_data.other_device,
                ));
            }
        }
        Err(err) => {
            return Err(JsonGeneric::new_generic(err.to_string()));
        }
    };

    // Get the user's key
    match User::get_key(&request_data.other_device) {
        Ok(result) => Ok(Json(Key {
            id: request_data.other_device.to_owned(),
            key: result,
        })),
        Err(e) => {
            println!("{:?}", e);
            return Err(not_found());
        }
    }
}

#[catch(404)]
pub fn not_found() -> Json<JsonGeneric> {
    JsonGeneric::new_generic("Resource was not found.".to_string())
}
