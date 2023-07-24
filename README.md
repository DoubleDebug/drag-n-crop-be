# Drag-n-Crop server

[Drag-n-crop](https://drag-n-crop.web.app) is an image and video cropping utility and a web server written in Rust.
<br />
The web server currently has 2 endpoints - one for cropping images and one for cropping videos.

Check out the Swagger documentation hosted [here](https://dh0418oxqna64.cloudfront.net).
<br />
Learn more about this project [here](https://github.com/DoubleDebug/drag-n-crop).

## Peer dependencies

This project has 1 peer dependency - [ffmpeg](https://ffmpeg.org).
<br />
In order for the video cropping utility to work, your machine needs to have ffmpeg installed or ffmpeg.exe placed in the root of this project.

## Limitations

| File type | Size  | Format                              |
| --------- | ----- | ----------------------------------- |
| image     | 100MB | jpg, jpeg, png, gif, bmp, webp, svg |
| video     | 1GB   | mp4, mkv, flv, avi, mov, wmv, webm  |

## How to run locally
0. Install [rust](https://www.rust-lang.org/tools/install).
1. Install [ffmpeg](https://ffmpeg.org) or copy ffmpeg.exe to the root of this project.
2. Clone this repository.
3. Run `cargo run` in the root directory.
4. Navigate to http://127.0.0.1:8000 to see the Swagger documentation.
