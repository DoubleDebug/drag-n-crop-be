use std::time::SystemTime;
use image::{ DynamicImage, ImageBuffer, Rgba };

fn create_random_bytes(size: u32) -> Vec<u8> {
  let mut array = Vec::with_capacity(size as usize);

  let current_time = SystemTime::now();
  let mut rng = current_time.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_nanos() as u64;

  for _ in 0..size {
    rng = (rng.wrapping_mul(1103515245) + 12345) % 256;
    array.push(rng as u8);
    rng = (rng.wrapping_mul(1103515245) + 12345) % 256;
    array.push(rng as u8);
    rng = (rng.wrapping_mul(1103515245) + 12345) % 256;
    array.push(rng as u8);
  }

  array
}

fn create_dummy_image(width: u32, height: u32) -> DynamicImage {
  let bytes = create_random_bytes(width * height);
  let mut image_buffer = ImageBuffer::<Rgba<u8>, Vec<u8>>::new(width, height);
  let pixel_count = (width * height) as usize;

  for (i, pixel) in image_buffer.pixels_mut().enumerate() {
    let byte_index = (i % pixel_count) * 3;

    if byte_index >= bytes.len() {
      break;
    }

    let red = bytes[byte_index];
    let green = bytes[byte_index + 1];
    let blue = bytes[byte_index + 2];

    *pixel = Rgba([red, green, blue, 255]);
  }

  DynamicImage::ImageRgba8(image_buffer)
}

#[cfg(test)]
pub mod tests {
  use std::fs;
  use drag_and_crop::{ crop_image, CropParameters, CropDimensions, ImageSize, Point };
  use image::GenericImageView;

  use super::*;

  #[test]
  pub fn test_crop_image() {
    // 1) prepare dummy image
    const WIDTH: u32 = 1024;
    const HEIGHT: u32 = 720;
    let input_file_path = "./test.jpg";
    let dummy_image = create_dummy_image(WIDTH, HEIGHT);
    let _ = dummy_image.save(input_file_path);

    // 2) prepare cropping parameters
    const WANTED_WIDTH: u32 = WIDTH / 2;
    const WANTED_HEIGHT: u32 = HEIGHT / 2;
    let output_file_path = "./test-cropped.jpg";

    let params = CropParameters {
      input_file_path: String::from("./test.jpg"),
      output_file_path: Some(String::from(output_file_path)),
      dimensions: CropDimensions {
        size: ImageSize { width: WANTED_WIDTH, height: WANTED_HEIGHT },
        top_left_point: Point { x: 50, y: 50 },
      },
    };
    let result = crop_image(&params).unwrap();

    // 3) test if result file matches
    assert_eq!(result.as_str(), output_file_path);

    // 4) test if resulting image dimensions match
    let metadata = image::open(output_file_path);
    let (w, h) = metadata.unwrap().dimensions();
    assert_eq!(WANTED_WIDTH, w);
    assert_eq!(WANTED_HEIGHT, h);

    // 5) cleanup
    let _ = fs::remove_file(input_file_path).unwrap();
    let _ = fs::remove_file(output_file_path).unwrap();
  }
}
