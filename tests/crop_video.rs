#[cfg(test)]
pub mod tests {
  use std::fs;
  use drag_and_crop::{ crop_video, CropParameters, CropDimensions, ImageSize, Point };

  #[tokio::test]
  pub async fn test_crop_video() {
    // 1) download dummy video
    const VIDEO_URL: &str = "https://i.imgur.com/MSMEI6H.mp4";
    let client = reqwest::Client::builder().build().unwrap();
    let response = client.get(VIDEO_URL).send().await.unwrap();
    let input_file_name = "./test.mp4";
    let bytes = response.bytes().await.unwrap();
    let _ = fs::write(&input_file_name, &bytes);

    // 2) prepare cropping parameters
    const WANTED_WIDTH: u32 = 500;
    const WANTED_HEIGHT: u32 = 400;
    let output_file_name = "./test-cropped.mp4";

    let params = CropParameters {
      input_file_path: String::from(input_file_name),
      output_file_path: Some(String::from(output_file_name)),
      dimensions: CropDimensions {
        size: ImageSize { width: WANTED_WIDTH, height: WANTED_HEIGHT },
        top_left_point: Point { x: 250, y: 300 },
      },
    };
    let result = crop_video(&params).unwrap();

    // 3) test if result file matches
    assert_eq!(result.as_str(), output_file_name);

    // 4) cleanup
    let _ = fs::remove_file(input_file_name).unwrap();
    let _ = fs::remove_file(output_file_name).unwrap();
  }
}
