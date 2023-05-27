use image::imageops;
use std::path::PathBuf;
use std::process::Command;

pub mod utils {
    pub mod file;
    pub mod validate;
}
use utils::file::file::get_output_path;
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
pub struct CropOptions {
    pub input_file_path: String,
    pub output_file_path: Option<String>,
    pub top_left_point: Point,
    pub size: ImageSize,
}

/**
 * Crops image with the given options and returns the file path of the newly created cropped image.
 */
pub fn crop_image(options: &CropOptions) -> Result<String, String> {
    validate_options(&options)?;

    let mut img = image::open(&options.input_file_path).unwrap();
    let cropped_img = imageops::crop(
        &mut img,
        options.top_left_point.x,
        options.top_left_point.y,
        options.size.width,
        options.size.height,
    );

    let output_file_path;
    if let Some(output_path) = &options.output_file_path {
        output_file_path = PathBuf::from(&output_path);
    } else {
        output_file_path = get_output_path(&options.input_file_path);
    }
    cropped_img.to_image().save(&output_file_path).unwrap();

    Ok(output_file_path.to_str().unwrap().to_string())
}

/**
 * Crops video with the given options and returns the file path of the newly created cropped video file.
 */
pub fn crop_video(options: &CropOptions) -> Result<String, String> {
    // validate_options(&options)?;

    let crop_dimensions = format!("crop={}:{}:{}:{}",
    &options.size.width, &options.size.height, &options.top_left_point.x, &options.top_left_point.y); // width:height:x:y
    let output_file_path;
    if let Some(output_path) = &options.output_file_path {
        output_file_path = PathBuf::from(&output_path);
    } else {
        output_file_path = get_output_path(&options.input_file_path);
    }

    let output = Command::new("./ffmpeg.exe")
        .arg("-i")
        .arg(&options.input_file_path)
        .arg("-filter:v")
        .arg(&crop_dimensions)
        .arg(&output_file_path)
        .arg("-loglevel")
        .arg("error")
        .output();
        

    if output.is_err() {
        return Err(output.unwrap_err().to_string());
    }

    let status = output.as_ref().unwrap().status;
    if !status.success() {
        let error_message = format!("{}\n{}", status.to_string(), std::str::from_utf8(&output.unwrap().stderr).unwrap());
        return Err(error_message);
    }
    
    return Ok(output_file_path.to_string_lossy().to_string());
    }

