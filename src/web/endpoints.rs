use std::path::Path;
use drag_and_crop::{ CropRequest, crop_video };
use drag_and_crop::{ crop_image, ApiResponse, CropParameters };
use rocket::serde::json::{ json, Json, Value };
use crate::web::firebase::{ download_file, get_access_token, upload_file };

pub mod routes {
  use rocket::{ serde::json::{ Json, Value }, response::status, http::Status };
  use drag_and_crop::CropRequest;

  #[post("/crop-image", format = "json", data = "<options>")]
  pub async fn post_crop_image(options: Json<CropRequest>) -> Value {
    super::handle_crop_request(options, true).await
  }
  #[post("/crop-video", format = "json", data = "<options>")]
  pub async fn post_crop_video(options: Json<CropRequest>) -> Value {
    super::handle_crop_request(options, false).await
  }
  #[options("/crop-image")]
  pub fn options_crop_image() -> status::Custom<String> {
    status::Custom(Status::NoContent, String::new())
  }
  #[options("/crop-video")]
  pub fn options_crop_video() -> status::Custom<String> {
    status::Custom(Status::NoContent, String::new())
  }
}

async fn handle_crop_request(options: Json<CropRequest>, is_image: bool) -> Value {
  let media_type = if is_image { "image" } else { "video" };
  // 1) get access token
  let token_result = get_access_token().await;
  if token_result.is_err() {
    return json!(ApiResponse::<String> {
      success: false,
      message: Some(String::from("Failed to authenticate the request.")),
      data: None,
    });
  }
  let token = token_result.as_ref().unwrap().as_str();

  // 2) download specified file from Firebase storage
  let download_result = download_file(token, &options.storage_file_path).await;
  if download_result.is_err() {
    return json!(ApiResponse::<String> {
      success: false,
      message: Some(format!("There was an error with the {} url.", media_type)),
      data: None,
    });
  }
  let file_name = download_result.unwrap();

  // 3) prepare cropping parameteres
  let only_file_name = Path::new(&file_name).file_name().unwrap().to_str().unwrap();
  let cropped_file_name = format!("./tmp/cropped-{}", only_file_name);
  let options = CropParameters {
    input_file_path: file_name,
    output_file_path: Some(cropped_file_name),
    dimensions: options.into_inner().dimensions,
  };

  // 4) crop image/video
  let result = if is_image { crop_image(&options) } else { crop_video(&options) };
  if let Err(crop_error) = result {
    return json!(ApiResponse::<String> {
      success: false,
      message: Some(crop_error),
      data: None,
    });
  }
  let cropped_file_path = result.unwrap();

  // 5) upload result to Firebase storage
  let upload_result = upload_file(token, cropped_file_path.as_str(), is_image).await;
  if upload_result.is_err() {
    return json!(ApiResponse::<String> {
      success: false,
      message: Some(format!("There was an error while getting the cropped {} URL.", media_type)),
      data: None,
    });
  }
  let cropped_file_url = upload_result.unwrap();

  // 6) return result
  json!(ApiResponse {
    success: true,
    message: None,
    data: Some(cropped_file_url),
  })
}
