pub mod routes {
  use drag_and_crop::{ crop_image, ApiResponse, CropRequest, CropParameters };
  use rocket::serde::json::{ Json, json, Value };

  use crate::web::firebase::{ download_file, get_access_token };

  #[post("/crop-image", format = "json", data = "<options>")]
  pub async fn post_crop_image(options: Json<CropRequest>) -> Value {
    let token_result = get_access_token().await;
    if token_result.is_err() {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(String::from("Failed to authenticate the request.")),
        data: None,
      });
    }
    let token = token_result.as_ref().unwrap().as_str();
    let download_result = download_file(token, &options.file_url).await;
    if download_result.is_err() {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(String::from("There was an error with the image url.")),
        data: None,
      });
    }
    let file_name = download_result.unwrap();
    let cropped_file_name = format!(".\\tmp\\{}", file_name);

    let options = CropParameters {
      input_file_path: file_name,
      output_file_path: Some(cropped_file_name),
      dimensions: options.into_inner().dimensions,
    };
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
