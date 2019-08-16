use crate::routes::json_generic::JsonGeneric;
use rocket_contrib::json::Json;

#[catch(404)]
pub fn not_found() -> Json<JsonGeneric> {
    JsonGeneric::new_generic("Resource was not found.".to_string())
}