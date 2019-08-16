use crate::db::paired::is_paired;
use crate::db::users::User;
use crate::routes::json_generic::{JsonGeneric, JsonGenericCodes};
use crate::routes::pair::Pairing;
use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct Key {
    pub id: String,
    pub key: String,
}

// Get public key of paired device
#[post("/", format = "json", data = "<request_data>")]
pub fn get_key(request_data: Json<Pairing>) -> Result<Json<Key>, BadRequest<Json<JsonGeneric>>> {
    // check if the devices are paired.
    // only paired devices are allowed to see each other's keys
    match is_paired(&request_data.my_device, &request_data.other_device) {
        Ok(result) => {
            if result == false {
                return Err(JsonGeneric::new_bad_request(
                    JsonGenericCodes::NotPaired,
                    request_data.my_device.to_string()
                        + " is not paired with "
                        + &request_data.other_device,
                ));
            }
        }
        Err(err) => {
            return Err(JsonGeneric::new_bad_request_generic(err.to_string()));
        }
    };

    // Get the user's key
    match User::get_key(&request_data.other_device) {
        Ok(result) => Ok(Json(Key {
            id: request_data.other_device.to_owned(),
            key: result,
        })),
        Err(e) => {
            return Err(JsonGeneric::new_bad_request_generic(e.to_string()));
        }
    }
}
