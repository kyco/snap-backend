use crate::db::paired::create;
use crate::db::users::device_id_exists;
use crate::routes::json_generic::{JsonGeneric, JsonGenericCodes};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Pairing {
    pub my_device: String,
    pub other_device: String,
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
