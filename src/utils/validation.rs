pub mod validation {
  use std::path::Path;
  use image::GenericImageView;

  use crate::{ CropParameters, utils::file::file::{ is_image_file, is_video_file } };

  /// Validates cropping parameters.
  /// If the validation is successful, it returns `true`, or an error message otherwise.
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::utils::validation::validation::validate_options;
  /// use drag_and_crop::{CropParameters, CropDimensions, ImageSize, Point};
  ///
  /// let mut params = CropParameters {
  ///   input_file_path: String::from("./media/bird.jpg"),
  ///   output_file_path: Some(String::from("./media/bird-cropped.jpg")),
  ///   dimensions: CropDimensions {
  ///     size: ImageSize { width: 100, height: 100 },
  ///     top_left_point: Point { x: 50, y: 50 },
  ///   },
  /// };
  ///
  /// let mut result = validate_options(&params);
  /// assert_eq!(result.unwrap(), true);
  ///
  /// params.input_file_path = String::from("./non-existing-file.jpg");
  /// result = validate_options(&params);
  /// assert_eq!(result, Err(String::from("The input file does not exist.")));
  ///
  /// params.input_file_path = String::from("./media/bird.jpg");
  /// params.dimensions.top_left_point.x = 2000;
  /// result = validate_options(&params);
  /// assert_eq!(result, Err(String::from("The top left point is out of bounds.")));
  ///
  /// params.dimensions.top_left_point.x = 50;
  /// params.dimensions.size.width = 5000;
  /// result = validate_options(&params);
  /// assert_eq!(result, Err(String::from("The output size is larger than the input image size.")));
  /// ```
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
