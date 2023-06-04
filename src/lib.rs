use image::imageops;
use std::path::PathBuf;
use std::process::Command;
use serde::{ Serialize, Deserialize };

pub mod utils {
  pub mod file;
  pub mod validate;
}
use utils::file::file::get_output_path;
use utils::validate::validate::validate_options;

#[derive(Serialize, Deserialize, Debug)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSize {
  pub width: u32,
  pub height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CropDimensions {
  pub top_left_point: Point,
  pub size: ImageSize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CropParameters {
  pub input_file_path: String,
  pub output_file_path: Option<String>,
  pub dimensions: CropDimensions,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CropRequest {
  pub storage_file_path: String,
  pub dimensions: CropDimensions,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
  pub success: bool,
  pub message: Option<String>,
  pub data: Option<T>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct UploadResponse {
  pub name: String,
}

/**
 * Crops image with the given options and returns the file path of the newly created cropped image.
 */
pub fn crop_image(options: &CropParameters) -> Result<String, String> {
  validate_options(&options)?;

  let mut img = match image::open(&options.input_file_path) {
    Ok(img) => img,
    Err(error) => {
      return Err(format!("There was an error while opening the image file: {:?}", error));
    }
  };
  let cropped_img = imageops::crop(
    &mut img,
    options.dimensions.top_left_point.x,
    options.dimensions.top_left_point.y,
    options.dimensions.size.width,
    options.dimensions.size.height
  );

  let output_file_path;
  if let Some(output_path) = &options.output_file_path {
    output_file_path = PathBuf::from(&output_path);
  } else {
    output_file_path = get_output_path(&options.input_file_path);
  }
  match cropped_img.to_image().save(&output_file_path) {
    Ok(_) => (),
    Err(error) => {
      return Err(format!("There was an error while saving the cropped image: {:?}", error));
    }
  }

  Ok(output_file_path.to_str().unwrap().to_string())
}

/**
 * Crops video with the given options and returns the file path of the newly created cropped video file.
 */
pub fn crop_video(options: &CropParameters) -> Result<String, String> {
  validate_options(&options)?;

  let crop_dimensions = format!(
    "crop={}:{}:{}:{}",
    &options.dimensions.size.width,
    &options.dimensions.size.height,
    &options.dimensions.top_left_point.x,
    &options.dimensions.top_left_point.y
  ); // width:height:x:y
  let output_file_path;
  if let Some(output_path) = &options.output_file_path {
    output_file_path = PathBuf::from(&output_path);
  } else {
    output_file_path = get_output_path(&options.input_file_path);
  }

  let output = Command::new("./ffmpeg.exe")
    .arg("-i")
    .arg(&options.input_file_path)
    .arg("-filter:v")
    .arg(&crop_dimensions)
    .arg(&output_file_path)
    .arg("-loglevel")
    .arg("error")
    .output();

  let status = match output {
    Ok(ref output) => output.status,
    Err(error) => {
      return Err(format!("There was an error while processing the video: {}", error));
    }
  };

  if !status.success() {
    let error_message = format!(
      "{}\n{}",
      status.to_string(),
      std::str::from_utf8(&output.unwrap().stderr).unwrap()
    );
    return Err(error_message);
  }

  return Ok(output_file_path.to_string_lossy().to_string());
}
