use std::path::{ PathBuf };
use image::{ imageops };
use ffmpeg_next::dictionary::Context;
use ffmpeg_next::format::input;
use ffmpeg_next::format::output;
use ffmpeg_next::codec::decoder::video::Video as VideoDecoder;
use ffmpeg_next::codec::encoder::video::Video as VideoEncoder;
use ffmpeg_next::util::frame::video::Video as VideoFrame;

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

pub fn crop_video() {
  ffmpeg::init().unwrap();

  // Input video file path
  let input_path = ".\\media\\sample.mp4";
  // Output video file path
  let output_path = ".\\media\\cropped-sample.mp4";

  // Open input video file
  let input_format = input(&input_path).unwrap();
  let mut input_context = input_format.open(&input_path).unwrap();

  // Find video stream
  let video_stream_index = input_context
    .streams()
    .best(ffmpeg::media::Type::Video)
    .unwrap()
    .index();

  // Get video decoder
  let mut decoder = input_context.codec(video_stream_index).unwrap().decoder().video().unwrap();

  // Retrieve video parameters
  let input_params = decoder.parameters();

  // Set cropping parameters
  let crop_x = 0;
  let crop_y = 0;
  let crop_width = 200;
  let crop_height = 500;

  // Set output video parameters with cropping
  let mut output_params = Context::new();
  output_params.set_int("width", crop_width).unwrap();
  output_params.set_int("height", crop_height).unwrap();
  output_params.set_int("x", crop_x).unwrap();
  output_params.set_int("y", crop_y).unwrap();

  // Open output video file for writing
  let output_format = output(&output_path).unwrap();
  let mut output_context = output_format.create(&output_path).unwrap();
  let mut output_stream = output_context
    .add_stream::<VideoEncoder>(ffmpeg::codec::id::Id::H264)
    .unwrap();
  output_stream.set_parameters(&output_params);

  // Open video encoder
  output_stream.codec().open(&output_params).unwrap();

  // Write output context header
  output_context.write_header().unwrap();

  // Iterate through video frames, crop, and write to output context
  while let Ok(packet) = input_context.read_packet() {
    if packet.stream_index() == video_stream_index {
      let mut decoded_frame = VideoFrame::empty();
      decoder.decode(&packet, &mut decoded_frame).unwrap();

      // Crop the frame
      let cropped_frame = decoded_frame.crop(crop_x, crop_y, crop_width, crop_height);

      // Encode the cropped frame
      let mut encoded_packet = ffmpeg::packet::Packet::empty();
      output_stream.codec().encode(&cropped_frame, &mut encoded_packet).unwrap();

      // Write the packet to the output context
      output_context.write_packet(&mut encoded_packet).unwrap();
    }
  }

  // Write output context trailer
  output_context.write_trailer().unwrap();
}