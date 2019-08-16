use crate::db::paired::is_paired;
use crate::db::users::device_id_exists;
use crate::push::push_notification;
use crate::routes::json_generic::{JsonGeneric, JsonGenericCodes};
use rocket_contrib::json::Json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: String,
    pub to: String,
    pub contents: String,
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

    // Send push notification to device
    let push =
        push_notification::PushNotification::new(&message.from, &message.to, &message.contents);

    // Send push notification
    match push.send() {
        Ok(_) => return JsonGeneric::new_ok("ok".to_string()),
        Err(e) => return JsonGeneric::new_generic(e.to_string()),
    }
}
