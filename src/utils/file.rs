pub mod file {
  use std::path::{ Path, PathBuf };

  /// Changes file name (without extension) and returns the full path.
  ///
  /// ## Usage:
  /// ```
  /// use std::path::Path;
  /// use drag_and_crop::utils::file::file::change_file_name;
  ///
  /// let new_file_name = change_file_name("./my/path/name.rs", "new_name");
  /// assert_eq!(new_file_name, Path::new("./my/path/new_name.rs"));
  /// ```
  pub fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
    let path = path.as_ref();
    let mut result = path.to_owned();
    result.set_file_name(name);
    if let Some(ext) = path.extension() {
      result.set_extension(ext);
    }
    result
  }

  /// Returns the file name (without extension).
  ///
  /// ## Usage:
  /// ```
  /// use std::path::Path;
  /// use drag_and_crop::utils::file::file::get_file_name;
  ///
  /// let path = Path::new("./my/path/test.rs");
  /// let name = get_file_name(&path);
  /// assert_eq!(name, "test");
  /// ```
  pub fn get_file_name(path: &impl AsRef<Path>) -> &str {
    Path::file_stem(Path::new(path.as_ref().as_os_str())).unwrap().to_str().unwrap()
  }

  /// Formats file name for Firebase storage.
  /// Gets file name from relative path and removes the "cropped-" prefix.
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::utils::file::file::format_file_name_for_storage;
  ///
  /// let file_name = "./tmp/cropped-image-1.jpg";
  /// let formatted = format_file_name_for_storage(file_name);
  /// assert_eq!(formatted, "image-1.jpg");
  /// ```
  pub fn format_file_name_for_storage(file_name: &str) -> String {
    let prefix_length = "cropped-".len();
    Path::new(file_name)
      .file_name()
      .unwrap()
      .to_string_lossy()
      .to_string()
      .chars()
      .skip(prefix_length)
      .collect()
  }

  /// Renames file at the specified path and appends string to the end of the file name.
  ///
  /// ## Usage:
  /// ```
  /// use std::path::Path;
  /// use drag_and_crop::utils::file::file::append_to_file_name;
  ///
  /// let old_path = Path::new(".\\my\\path\\test.rs");
  /// let new_path = append_to_file_name(&old_path, "123");
  /// assert_eq!(".\\my\\path\\test123.rs", new_path.to_str().unwrap());
  /// ```
  pub fn append_to_file_name(path: &impl AsRef<Path>, text: &str) -> PathBuf {
    let old_file_name = get_file_name(&path);
    let mut new_file_name = String::from(old_file_name);
    new_file_name.push_str(text);
    let new_path = change_file_name(path, new_file_name.as_str());

    new_path
  }

  /// Generates a new file name that doesn't already exist.
  /// It appends "-1" at the end of input file path.
  /// If a file already exists for that path,
  /// it keeps increasing the count until it finds a non-existing file path.
  pub fn get_output_path(input_path: &String) -> PathBuf {
    let mut output_file_path;
    let mut i = 1;
    loop {
      output_file_path = append_to_file_name(
        &Path::new(&input_path),
        format!("-{}", i.to_string().as_str()).as_str()
      );
      if !Path::new(&output_file_path).exists() {
        break;
      }
      i += 1;
    }
    return output_file_path;
  }

  /// Returns `true` if the file is an image.
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::utils::file::file::is_image_file;
  ///
  /// let file_name_1 = "./images/family-photo.jpg";
  /// let is_image_1 = is_image_file(file_name_1);
  /// assert_eq!(is_image_1, true);
  ///
  /// let file_name_2 = "./files/abc.txt";
  /// let is_image_2 = is_image_file(file_name_2);
  /// assert_eq!(is_image_2, false);
  /// ```
  ///
  /// ## Supported video formats:
  /// "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg"
  pub fn is_image_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    match path.extension().and_then(std::ffi::OsStr::to_str) {
      Some(ext) => {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
          "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" => true,
          _ => false,
        }
      }
      None => false,
    }
  }

  /// Returns `true` if the file is an video.
  ///
  /// ## Usage:
  /// ```
  /// use drag_and_crop::utils::file::file::is_video_file;
  ///
  /// let file_name_1 = "./videos/family-video.mkv";
  /// let is_video_1 = is_video_file(file_name_1);
  /// assert_eq!(is_video_1, true);
  ///
  /// let file_name_2 = "./files/abc.txt";
  /// let is_video_2 = is_video_file(file_name_2);
  /// assert_eq!(is_video_2, false);
  /// ```
  ///
  /// ## Supported video formats:
  /// "mp4" | "mkv" | "flv" | "avi" | "mov" | "wmv" | "webm"
  pub fn is_video_file(file_path: &str) -> bool {
    let path = Path::new(file_path);
    match path.extension().and_then(std::ffi::OsStr::to_str) {
      Some(ext) => {
        let ext_lower = ext.to_lowercase();
        match ext_lower.as_str() {
          "mp4" | "mkv" | "flv" | "avi" | "mov" | "wmv" | "webm" => true,
          _ => false,
        }
      }
      None => false,
    }
  }
}
