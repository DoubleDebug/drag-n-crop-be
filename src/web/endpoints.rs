pub mod routes {
  use drag_and_crop::{ CropRequest, UploadRequest };
  use crate::web::handlers::handlers::{ handle_crop_request, handle_upload_request };
  use rocket::{
    serde::json::{ Json, Value },
    response::{ status, stream::{ Event, EventStream } },
    http::Status,
  };

  #[post("/crop-image", format = "json", data = "<options>")]
  pub async fn post_crop_image(options: Json<CropRequest>) -> Value {
    handle_crop_request(options, true).await
  }
  #[post("/crop-video", format = "json", data = "<options>")]
  pub async fn post_crop_video(options: Json<CropRequest>) -> EventStream![] {
    EventStream! {
      yield Event::data("Processing...");
      let result = handle_crop_request(options, false).await;
      yield Event::data(result.to_string());
    }
  }
  #[post("/upload-media", format = "json", data = "<options>")]
  pub async fn post_upload_media(options: Json<UploadRequest>) -> Value {
    handle_upload_request(options).await
  }
  #[options("/crop-image")]
  pub fn options_crop_image() -> status::Custom<String> {
    status::Custom(Status::NoContent, String::new())
  }
  #[options("/crop-video")]
  pub fn options_crop_video() -> status::Custom<String> {
    status::Custom(Status::NoContent, String::new())
  }
  #[options("/upload-media")]
  pub fn options_upload_media() -> status::Custom<String> {
    status::Custom(Status::NoContent, String::new())
  }
}
