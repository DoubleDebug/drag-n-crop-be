use std::{ path::Path, fs::File, io::Write, error::Error };
use drag_and_crop::{ utils::file::file::get_file_name, UploadResponse };
use yup_oauth2::{ ServiceAccountAuthenticator, read_service_account_key };
use rocket::serde::json::serde_json;

const STORAGE_SCOPE: &str = "https://www.googleapis.com/auth/devstorage.read_write";
const DOWNLOAD_URL: &str =
  "https://storage.googleapis.com/download/storage/v1/b/drag-n-crop.appspot.com/o";
const UPLOAD_URL: &str =
  "https://storage.googleapis.com/upload/storage/v1/b/drag-n-crop.appspot.com/o";

/**
 * Returns Oath2 access token with Firebase storage permissions
 */
pub async fn get_access_token() -> Result<String, Box<dyn Error>> {
  let path = Path::new("service-account.json");
  let sa_key = read_service_account_key(&path).await?;
  let auth = ServiceAccountAuthenticator::builder(sa_key).build().await?;
  let token = auth.token(&[STORAGE_SCOPE]).await?;
  let access_token = token.token();

  if let Some(result) = access_token {
    Ok(String::from(result))
  } else {
    Err("Failed to get access token.".into())
  }
}

/**
 * Uploads image to Firebase storage and returns resource path within the storage bucket
 */
pub async fn upload_image(access_token: &str, file_path: &str) -> Result<String, Box<dyn Error>> {
  upload_file(access_token, file_path, true).await
}

/**
 * Uploads video to Firebase storage and returns resource path within the storage bucket
 */
pub async fn upload_video(access_token: &str, file_path: &str) -> Result<String, Box<dyn Error>> {
  upload_file(access_token, file_path, false).await
}

pub async fn download_file(access_token: &str, file_name: &str) -> Result<String, Box<dyn Error>> {
  let client = reqwest::Client::builder().build()?;

  // headers
  let mut headers = reqwest::header::HeaderMap::new();
  headers.insert("Authorization", format!("Bearer {}", access_token).parse()?);

  // request
  let request = client
    .request(reqwest::Method::GET, format!("{}/{}", DOWNLOAD_URL, file_name))
    .headers(headers);
  let response = request.send().await?;

  // write response to file
  let file_name = format!(".\\tmp\\{}", get_file_name(&file_name));
  let mut file = File::create(&file_name)?;
  let bytes = response.bytes().await?;
  tokio::task::spawn_blocking(move || file.write_all(&bytes)).await;

  Ok(file_name)
}

async fn upload_file(
  access_token: &str,
  file_path: &str,
  is_image: bool
) -> Result<String, Box<dyn Error>> {
  let client = reqwest::Client::builder().build()?;

  // headers
  let mut headers = reqwest::header::HeaderMap::new();
  let file_type = if is_image { "image/jpeg" } else { "video/mp4" };
  headers.insert("Content-Type", file_type.parse()?);
  headers.insert("Authorization", format!("Bearer {}", access_token).parse()?);

  // body
  let bytes = std::fs::read(file_path)?;

  // query parameters
  let file_name = get_file_name(&file_path);
  let folder_name = if is_image { "images" } else { "videos" };
  let storage_file_name = format!("cropped/{}/{}", &folder_name, &file_name);
  let query: [(&str, &str); 2] = [
    ("uploadType", "media"),
    ("name", &storage_file_name),
  ];

  // request
  let request = client
    .request(reqwest::Method::POST, UPLOAD_URL)
    .headers(headers)
    .query(&query)
    .body(bytes);

  let response = request.send().await?;
  let body = response.text().await?;
  let upload_response: UploadResponse = serde_json::from_str(body.as_str())?;

  Ok(upload_response.name)
}
