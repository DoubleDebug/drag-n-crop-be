pub mod url {
  use reqwest::Response;
  use regex::Regex;
  use uuid::Uuid;
  use std::{ io::{ Cursor, copy }, path::Path, fs::File };

  /// Returns `true` if the url points to an image with one of the supported extensions.
  ///
  /// ## Usage
  /// ```
  /// use drag_and_crop::utils::file::file::is_image_url;
  ///
  /// let url = "https://i.imgur.com/mXxCzXG.png?a=b";
  /// let result = is_image_url(url);
  /// assert_eq!(result, true);
  /// ```
  pub fn is_image_url(url: &str) -> bool {
    let regex = Regex::new(r"https:\/\/(.+?)(\.(png|jpg|jpeg|gif|bmp|webp|svg))(\?(.*))?").unwrap();
    if let Some(matches) = regex.captures(url) {
      return matches.get(2).is_some();
    }

    return false;
  }

  /// Returns `true` if the url points to a video with one of the supported extensions.
  ///
  /// ## Usage
  /// ```
  /// use drag_and_crop::utils::file::file::is_video_url;
  ///
  /// let url = "https://i.imgur.com/mXxCzXG.mp4?c=d";
  /// let result = is_video_url(url);
  /// assert_eq!(result, true);
  /// ```
  pub fn is_video_url(url: &str) -> bool {
    let regex = Regex::new(r"https:\/\/(.+?)(\.(mp4|mkv|flv|avi|mov|wmv|webm))(\?(.*))?").unwrap();
    if let Some(matches) = regex.captures(url) {
      return matches.get(2).is_some();
    }

    return false;
  }

  /// Return `true` if the response headers have a valid content type (image or video).
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::web::url::url::has_valid_content_type;
  ///
  /// let mut image_headers = HeaderMap::new();
  /// image_headers.insert("Content-Type", "image/jpeg");
  /// let is_valid_image = has_valid_content_type(image_headers);
  /// assert_eq!(is_valid_image, true);
  ///
  /// let mut video_headers = HeaderMap::new();
  /// video_headers.insert("Content-Type", "video/mp4");
  /// let is_valid_video = has_valid_content_type(video_headers);
  /// assert_eq!(is_valid_video, true);
  /// ```
  pub fn has_valid_content_type(response: &Response) -> bool {
    let content_type_option = response.headers().get("Content-Type");
    if content_type_option.is_none() {
      return false;
    }

    let content_type_str = content_type_option.unwrap().to_str().unwrap();
    if !content_type_str.starts_with("image") && !content_type_str.starts_with("video") {
      return false;
    }

    return true;
  }

  /// Parses extension of a URL that points to a media file.
  ///
  /// ## Example:
  /// input: "https://i.imgur.com/mXxCzXG.mp4?c=d"
  /// output: ".mp4"
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::web::url::url::get_extension;
  ///
  /// let url = String::from("https://i.imgur.com/mXxCzXG.mp4?c=d");  ///
  /// let file_name = get_extension(url);
  /// assert_eq!(file_name, Some(".mp4"));
  /// ```
  pub fn get_extension(url: &str) -> Option<String> {
    let path = Path::new(url);
    let path_file_name = path.file_name();
    if path_file_name.is_none() {
      return None;
    }

    let path_file_name_str = path_file_name.unwrap().to_str().unwrap();
    let regex = Regex::new(
      r"((.+?)(\.(png|jpg|jpeg|gif|bmp|webp|svg|mp4|mkv|flv|avi|mov|wmv|webm)))(\?(.*))?"
    ).unwrap();
    let matches = regex.captures(path_file_name_str);
    if matches.is_none() {
      return None;
    }

    let extension = matches.unwrap().get(3);
    if extension.is_none() {
      return None;
    }

    return Some(String::from(extension.unwrap().as_str()));
  }

  /// Downloads file from URL.
  /// Returns `None` if the downloaded content isn't an image or a video.
  /// Otherwise, it writes the content to a file and returns the file name.
  pub async fn download_from_url(url: &str) -> Option<String> {
    let result = reqwest::get(url).await;
    if result.is_err() {
      return None;
    }

    let response = result.unwrap();
    if !has_valid_content_type(&response) {
      return None;
    }

    let extension = get_extension(url);
    if extension.is_none() {
      return None;
    }
    let uuid = Uuid::new_v4().to_string();
    let file_name = format!("{}{}", uuid, extension.unwrap());
    let final_file_path = Path::new("./tmp").join(file_name);
    let final_file_name = final_file_path.to_str().unwrap();

    let file = File::create(final_file_name);
    if file.is_err() {
      return None;
    }

    let response_bytes = response.bytes().await;
    if response_bytes.is_err() {
      return None;
    }

    let mut content = Cursor::new(response_bytes.unwrap());
    let copy_result = copy(&mut content, &mut file.unwrap());

    if copy_result.is_err() {
      return None;
    }

    Some(String::from(final_file_name))
  }
}
