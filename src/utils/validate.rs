pub mod validate {
  use std::path::Path;
  use image::GenericImageView;

  use crate::ImageCropOptions;

  pub fn validate_options(options: &ImageCropOptions) -> Result<bool, String> {
    // 1) check if input file exists
    if !Path::new(&options.file_path).exists() {
      return Err(String::from("The specified file does not exist."));
    }

    // 2) check if output file exists
    if let Some(output_path) = &options.result_file_path {
      if Path::new(&output_path).exists() {
        return Err(String::from("The specified output file already exists."));
      }
    }

    // 3) check if top left point is within image dimensions
    let img = image::open(Path::new(&options.file_path));
    if img.is_err() {
      let err_message = img.err().unwrap().to_string();
      return Err(err_message);
    }
    let (img_x, img_y) = img.unwrap().dimensions();
    if options.top_left_point.x >= img_x || options.top_left_point.y >= img_y {
      return Err(String::from("The top left point is out of bounds."));
    }

    // 4) check if output size if larger than input image size
    if
      options.size.width > img_x - options.top_left_point.x ||
      options.size.height > img_y - options.top_left_point.y
    {
      return Err(String::from("The output size is larger than the input image size."));
    }

    Ok(true)
  }
}