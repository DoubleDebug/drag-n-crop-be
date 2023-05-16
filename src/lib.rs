use std::path::{ PathBuf };
use image::{ imageops };

pub mod utils {
  pub mod file;
  pub mod validate;
}
use utils::file::file::{ get_output_path };
use utils::validate::validate::validate_options;

#[derive(Debug)]
pub struct Point {
  pub x: u32,
  pub y: u32,
}

#[derive(Debug)]
pub struct ImageSize {
  pub width: u32,
  pub height: u32,
}

#[derive(Debug)]
pub struct ImageCropOptions {
  pub file_path: String,
  pub result_file_path: Option<String>,
  pub top_left_point: Point,
  pub size: ImageSize,
}

/**
 * Crops image with the given options and returns the file path of the newly created cropped image.
 */
pub fn crop_image(options: ImageCropOptions) -> Result<String, String> {
  // 1) validate crop options
  validate_options(&options)?;

  let mut img = image::open(&options.file_path).unwrap();
  let cropped_img = imageops::crop(
    &mut img,
    options.top_left_point.x,
    options.top_left_point.y,
    options.size.width,
    options.size.height
  );

  let output_file_path;
  if let Some(output_path) = options.result_file_path {
    output_file_path = PathBuf::from(&output_path);
  } else {
    output_file_path = get_output_path(options.file_path);
  }
  cropped_img.to_image().save(&output_file_path).unwrap();

  Ok(output_file_path.to_str().unwrap().to_string())
}