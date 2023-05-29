pub mod routes {
  use drag_and_crop::{ CropOptions, crop_image, ApiResponse };
  use rocket::serde::json::{ Json, json, Value };

  #[post("/crop-image", format = "json", data = "<options>")]
  pub fn post_crop_image(options: Json<CropOptions>) -> Value {
    let result = crop_image(&options);

    match result {
      Ok(result) =>
        json!(ApiResponse {
          success: true,
          message: None,
          data: Some(result),
        }),
      Err(error) =>
        json!(ApiResponse::<String> {
          success: false,
          message: Some(error),
          data: None,
        }),
    }
  }
}
