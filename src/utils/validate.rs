pub mod validate {
  use std::path::Path;
  use image::GenericImageView;

  use crate::{ CropParameters, utils::file::file::{ is_image_file, is_video_file } };

  pub fn validate_options(options: &CropParameters) -> Result<bool, String> {
    // 0) check if input file exists
    if !Path::new(&options.input_file_path).exists() {
      return Err(String::from("The input file does not exist."));
    }

    // 1) check if file is an image or video
    let is_image = is_image_file(&options.input_file_path);
    let is_video = is_video_file(&options.input_file_path);
    if !is_image && !is_video {
      return Err(String::from("The input file is not an image nor a video."));
    }

    // 2) check if output file exists
    if let Some(output_path) = &options.output_file_path {
      if Path::new(&output_path).exists() {
        return Err(String::from("The output file already exists."));
      }
    }

    if is_image {
      // 3) check if top left point is within image dimensions
      let img = image::open(Path::new(&options.input_file_path));
      if img.is_err() {
        let err_message = img.err().unwrap().to_string();
        return Err(err_message);
      }
      let (width, height) = img.unwrap().dimensions();
      if
        options.dimensions.top_left_point.x >= width ||
        options.dimensions.top_left_point.y >= height
      {
        return Err(String::from("The top left point is out of bounds."));
      }

      // 4) check if output size if larger than input size
      if
        options.dimensions.size.width > width - options.dimensions.top_left_point.x ||
        options.dimensions.size.height > height - options.dimensions.top_left_point.y
      {
        return Err(String::from("The output size is larger than the input image size."));
      }
    }

    Ok(true)
  }
}
