use drag_and_crop::{ crop_image, crop_video, CropOptions, ImageSize, Point };

fn main() {
  println!("Drag and crop!");
  println!("-------------------------------");

  // 1) testing image cropping
  let image_options = CropOptions {
    input_file_path: ".\\media\\bird.jpg".to_string(),
    output_file_path: Some(".\\media\\test.jpg".to_string()),
    top_left_point: Point { x: 10, y: 0 },
    size: ImageSize {
      width: 710,
      height: 600,
    },
  };

  match crop_image(&image_options) {
    Ok(result) => println!("Cropped image path: {}", result),
    Err(message) => println!("Failed to crop image: {}", message),
  }

  // 2) testing video cropping
  let video_options = CropOptions {
    input_file_path: ".\\media\\sample.mp4".to_string(),
    output_file_path: Some(".\\media\\sample-cropped.mp4".to_string()),
    top_left_point: Point { x: 0, y: 0 },
    size: ImageSize {
      width: 720,
      height: 400
    }
  };

  match crop_video(&video_options) {
    Ok(result) => println!("Cropped video path: {}", result),
    Err(message) => println!("Failed to crop video: {}", message),
  }
}