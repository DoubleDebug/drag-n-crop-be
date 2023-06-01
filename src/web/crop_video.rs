pub mod routes {
  use drag_and_crop::{ CropOptions, crop_video, ApiResponse };
  use rocket::serde::json::{ Json, json, Value };

  #[post("/crop-video", format = "json", data = "<options>")]
  pub fn post_crop_video(options: Json<CropOptions>) -> Value {
    let result = crop_video(&options);

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
