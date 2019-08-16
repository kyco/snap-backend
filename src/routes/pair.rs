use crate::db::paired::create;
use crate::db::users::device_id_exists;
use crate::routes::json_generic::{JsonGeneric, JsonGenericCodes};
use rocket::response::status::BadRequest;
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pairing {
    pub my_device: String,
    pub other_device: String,
}

// Pair two devices
#[post("/", format = "json", data = "<pair>")]
pub fn pair(pair: Json<Pairing>) -> Result<Json<JsonGeneric>, BadRequest<Json<JsonGeneric>>> {
    // Check if my_device exists
    match device_id_exists(&pair.my_device) {
        Ok(result) => {
            if result == false {
                return Err(JsonGeneric::new_bad_request(
                    JsonGenericCodes::NotFound,
                    "my_device: ".to_owned() + &pair.my_device + " doesn't exist",
                ));
            }
        }
        Err(err) => {
            return Err(JsonGeneric::new_bad_request_generic(err.to_string()));
        }
    };

    // check if other_device exists
    match device_id_exists(&pair.other_device) {
        Ok(result) => {
            if result == false {
                return Err(JsonGeneric::new_bad_request(
                    JsonGenericCodes::NotFound,
                    "other_device: ".to_owned() + &pair.other_device + " doesn't exist",
                ));
            }
        }
        Err(err) => {
            return Err(JsonGeneric::new_bad_request_generic(err.to_string()));
        }
    };

    // If both exist, then add them into the relationship database
    match create(&pair.my_device, &pair.other_device) {
        Ok(result) => {
            if result == true {
                return Ok(JsonGeneric::new_ok("ok".to_string()));
            } else {
                return Err(JsonGeneric::new_bad_request(
                    JsonGenericCodes::NotFound,
                    "could not find devices".to_string(),
                ));
            }
        }
        Err(e) => Err(JsonGeneric::new_bad_request_generic(e.to_string())),
    }
}
