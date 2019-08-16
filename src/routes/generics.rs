use crate::routes::json_generic::JsonGeneric;
use rocket_contrib::json::Json;
use rocket::response::status;

#[catch(404)]
pub fn not_found() ->  status<Json<JsonGeneric>> {
    status::NotFound(Some(JsonGeneric::new_generic("Resource was not found.".to_string())))
}