use image::{ GenericImageView, imageops };

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
  let mut img = image::open(&options.file_path).unwrap();
  let cropped_img = imageops::crop(
    &mut img,
    options.top_left_point.x,
    options.top_left_point.y,
    options.size.width,
    options.size.height
  );

  Ok(format!("{:?}", cropped_img.dimensions()))
}