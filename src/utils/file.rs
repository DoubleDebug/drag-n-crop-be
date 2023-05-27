pub mod file {
    use std::path::{Path, PathBuf};

    /**
    Changes file name (without extension) and returns the full path.
    Example:

    - `let new_file_name = change_file_name("./my/path/name.rs", "new_name");`
    - `assert_eq!(new_file_name, Path::new("./my/path/new_name.rs"));`
    */
    pub fn change_file_name(path: impl AsRef<Path>, name: &str) -> PathBuf {
        let path = path.as_ref();
        let mut result = path.to_owned();
        result.set_file_name(name);
        if let Some(ext) = path.extension() {
            result.set_extension(ext);
        }
        result
    }

    /**
       Returns the file name (without extension).
       Example:

       - `let path = Path::new("./my/path/test.rs");`
       - `let name = get_file_name(&path);`
       - `assert_eq!(name, "test");`
    */
    pub fn get_file_name(path: &impl AsRef<Path>) -> &str {
        Path::file_stem(Path::new(path.as_ref().as_os_str()))
            .unwrap()
            .to_str()
            .unwrap()
    }

    /**
     Renames file at the specified path and appends string to the end of the file name.
     Example:

     - `let old_path = Path::new(".\\my\\path\\test.rs");`
     - `let new_path = append_to_file_name(&old_path, "123");`
     - `assert_eq!(".\\my\\path\\test123.rs", new_path.to_str().unwrap());`
    */
    pub fn append_to_file_name(path: &impl AsRef<Path>, text: &str) -> PathBuf {
        let old_file_name = get_file_name(&path);
        let mut new_file_name = String::from(old_file_name);
        new_file_name.push_str(text);
        let new_path = change_file_name(path, new_file_name.as_str());

        new_path
    }

    /**
     * Generates a new file name that doesn't already exist.
     * It appends "-1" at the end of input file path.
     * If a file already exists for that path,
     * it keeps increasing the count until it finds a non-existing file path.
     */
    pub fn get_output_path(input_path: &String) -> PathBuf {
        let mut output_file_path;
        let mut i = 1;
        loop {
            output_file_path = append_to_file_name(
                &Path::new(&input_path),
                format!("-{}", i.to_string().as_str()).as_str(),
            );
            if !Path::new(&output_file_path).exists() {
                break;
            }
            i += 1;
        }
        return output_file_path;
    }

    /**
     * Returns `true` if the file is an image.
     */
    pub fn is_image_file(file_path: &str) -> bool {
        let path = Path::new(file_path);
        match path.extension().and_then(std::ffi::OsStr::to_str) {
            Some(ext) => {
                let ext_lower = ext.to_lowercase();
                match ext_lower.as_str() {
                    "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "tiff" | "svg" => true,
                    _ => false,
                }
            }
            None => false,
        }
    }

    /**
     * Returns `true` if the file is a video.
     */
    pub fn is_video_file(file_path: &str) -> bool {
        let path = Path::new(file_path);
        
        if !&path.exists() {
            return false;
        }
        
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
