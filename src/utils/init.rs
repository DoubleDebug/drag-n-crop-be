use std::fs;
use std::path::Path;

pub fn init() {
  // create tmp folder for storing downloaded media
  let temp_folder = "./tmp";
  let folder_name = Path::new(temp_folder);
  if folder_name.exists() {
    // delete all media from previous session
    let dir_result = fs::read_dir(temp_folder);
    if dir_result.is_err() {
      println!("Failed to read the contents of the temporary directory.");
      return;
    }
    let dir = dir_result.unwrap();
    for entry in dir {
      if entry.is_err() {
        println!("Failed to read the contents of the temporary directory.");
        return;
      }
      let entry = entry.unwrap();
      let path = entry.path();
      if path.is_file() {
        let deletion_result = fs::remove_file(path);
        if deletion_result.is_err() {
          println!("Failed to clear the contents of the temporary directory.");
          continue;
        }
        deletion_result.unwrap();
      }
    }
  } else {
    let _ = fs::create_dir(folder_name);
  }
}
