use drag_and_crop::{ crop_image, ImageCropOptions, ImageSize, Point };

fn main() {
  let options = ImageCropOptions {
    file_path: ".\\imgs\\bird.jpg".to_string(),
    result_file_path: None,
    top_left_point: Point { x: 10, y: 0 },
    size: ImageSize {
      width: 710,
      height: 600,
    },
  };

  println!("Hello, world!");
  match crop_image(options) {
    Ok(result) => println!("Cropped image path: {}", result),
    Err(message) => println!("Failed to crop image: {}", message),
  }
}