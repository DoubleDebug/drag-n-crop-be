pub mod handlers {
  use std::path::Path;
  use drag_and_crop::{ CropRequest, crop_video, UploadRequest };
  use drag_and_crop::{ crop_image, ApiResponse, CropParameters };
  use rocket::serde::json::{ json, Json, Value };
  use crate::web::firebase::{ download_file, get_access_token, upload_file };
  use crate::web::url::url::{ download_from_url, is_image_url, is_video_url };

  pub async fn handle_crop_request(options: Json<CropRequest>, is_image: bool) -> Value {
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
    let media_type = if is_image { "image" } else { "video" };

    // 2) download file from URL or firebase storage
    let mut file_name = String::new();
    if let Some(storage_path) = &options.storage_file_path {
      let download_result = download_file(token, &storage_path).await;
      if download_result.is_err() {
        return json!(ApiResponse::<String> {
          success: false,
          message: Some(format!("There was an error with the {} url.", media_type)),
          data: None,
        });
      }
      file_name = download_result.unwrap();
    } else if let Some(url) = &options.url {
      let download_result = download_from_url(url).await;
      if download_result.is_none() {
        return json!(ApiResponse::<String> {
          success: false,
          message: Some(format!("There was an error with the {} url.", media_type)),
          data: None,
        });
      }
      file_name = download_result.unwrap();
    }

    if file_name == "" {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(format!("Bad request - both storage file path and URL fields are empty.")),
        data: None,
      });
    }

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
    let upload_result = upload_file(token, cropped_file_path.as_str(), is_image, false).await;
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

  pub async fn handle_upload_request(options: Json<UploadRequest>) -> Value {
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

    // 2) validate url file type
    let url = options.url.as_str();
    let is_image = is_image_url(url);
    let is_video = is_video_url(url);
    if !is_image && !is_video {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(String::from("The url contains an unsupported file type.")),
        data: None,
      });
    }
    let media_type = if is_image { "image" } else { "video" };

    // 3) download file locally
    let download_result = download_from_url(url).await;
    if download_result.is_none() {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(format!("There was an error with the {} url.", media_type)),
        data: None,
      });
    }
    let file_path = download_result.unwrap();

    // 4) upload file to storage
    let upload_result = upload_file(token, file_path.as_str(), is_image, true).await;
    if upload_result.is_err() {
      return json!(ApiResponse::<String> {
        success: false,
        message: Some(format!("There was an error while uploading the {} to storage.", media_type)),
        data: None,
      });
    }
    let storage_path = upload_result.unwrap();

    json!(ApiResponse {
      success: true,
      message: None,
      data: Some(storage_path),
    })
  }
}
