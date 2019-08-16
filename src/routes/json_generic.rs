// Generic json responses
use rocket_contrib::json::Json;
use rocket::response::status::BadRequest;

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonGeneric {
    pub satus: JsonGenericCodes,
    pub reason: String,
}

impl JsonGeneric {
    // return a new json code
    pub fn new_response(c: JsonGenericCodes, r: String) -> Json<JsonGeneric> {
        Json(JsonGeneric {
            satus: c,
            reason: r,
        })
    }
    // return a new json generic error
    pub fn new_generic(r: String) -> Json<JsonGeneric> {
        Json(JsonGeneric {
            satus: JsonGenericCodes::Generic,
            reason: r,
        })
    }
    // return a new json generic error
    pub fn new_bad_request(c: JsonGenericCodes, r: String) -> BadRequest<Json<JsonGeneric>> {
        BadRequest(Some(Json(JsonGeneric {
            satus: c,
            reason: r,
        })))
    }
    // return a new json generic error
    pub fn new_bad_request_generic(r: String) -> BadRequest<Json<JsonGeneric>> {
        BadRequest(Some(Json(JsonGeneric {
            satus: JsonGenericCodes::Generic,
            reason: r,
        })))
    }
    // return a new json ok response
    pub fn new_ok(r: String) -> Json<JsonGeneric> {
        Json(JsonGeneric {
            satus: JsonGenericCodes::OK,
            reason: r,
        })
    }
}

// Contains error codes for json responses
#[derive(Serialize, Deserialize, Debug)]
pub enum JsonGenericCodes {
    OK,
    Generic,
    NotFound,
    NotRegistered,
    NotPaired,
    AlreadyRegistered,
}
