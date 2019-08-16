use crate::db::users::User;
use crate::routes::json_generic::{JsonGeneric,JsonGenericCodes};
use rocket_contrib::json::Json;
use rocket::response::status;

// Register a new device
#[post("/", format = "json", data = "<request_data>")]
pub fn register(mut request_data: Json<User>) -> Result<Json<User>, status<Json<JsonGeneric>>> {
    match register_do(&mut request_data) {
        Ok(_) => Ok(request_data),
        Err(e) => Err(JsonGeneric::new_bad_request_generic(e.to_string())),
    }
}

// performs the registration
pub fn register_do(user_data: &mut User) -> Result<i32, String> {
    // Generate random id from token
    match user_data.generate_id() {
        Ok(_) => {
            // Check if the user exists, if he does, then return
            let exists: bool = match user_data.exists() {
                Ok(exists) => exists,
                Err(e) => {
                    return Err(e.to_string());
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
