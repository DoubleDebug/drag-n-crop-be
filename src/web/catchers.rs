use drag_and_crop::ApiResponse;
use rocket::serde::json::{ json, Value };

#[catch(422)]
pub fn unprocessable_entity() -> Value {
  json!(ApiResponse::<String> {
    success: false,
    message: Some(String::from("Bad request - there was a type mismatch with the request data.")),
    data: None,
  })
}

#[catch(default)]
pub fn default_catcher() -> Value {
  json!(ApiResponse::<String> {
    success: false,
    message: Some(String::from("There was an unknown error while processing the request.")),
    data: None,
  })
}
