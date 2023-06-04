pub mod routes {
  use std::path::Path;

  use drag_and_crop::{ crop_image, ApiResponse, CropRequest, CropParameters };
  use rocket::serde::json::{ Json, json, Value };

  use crate::web::firebase::{ download_file, get_access_token, upload_image };

  #[post("/crop-image", format = "json", data = "<options>")]
  pub async fn post_crop_image(options: Json<CropRequest>) -> Value {
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
        message: Some(String::from("There was an error with the image url.")),
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

    // 4) crop image
    let result = crop_image(&options);
    if let Err(crop_error) = result {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(crop_error),
        data: None,
      });
    }
    let cropped_file_path = result.unwrap();

    // 5) upload result to Firebase storage
    let upload_result = upload_image(token, cropped_file_path.as_str()).await;
    if upload_result.is_err() {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(String::from("There was an error while getting the cropped image URL.")),
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
}
