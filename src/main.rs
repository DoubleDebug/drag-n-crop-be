use drag_and_crop::{ Point, ImageSize, ImageCropOptions, crop_image };

fn main() {
  let options = ImageCropOptions {
    file_path: "./imgs/bird.jpg".to_string(),
    result_file_path: None,
    top_left_point: Point {
      x: 100,
      y: 0,
    },
    size: ImageSize {
      width: 400,
      height: 400,
    },
  };

  println!("Hello, world!");
  println!("{:?}", crop_image(options));
}