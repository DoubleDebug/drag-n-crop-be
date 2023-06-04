# Drag-n-Crop

Image and video cropping tool and web server written in Rust

## External dependencies

This project has 1 external dependency - [ffmpeg](https://ffmpeg.org).

In order for the video cropping utility to work, your machine needs to have ffmpeg installed or ffmpeg.exe placed in the root of this project.

## Limitations

| File type | Size  | Format                              |
| --------- | ----- | ----------------------------------- |
| image     | 100MB | jpg, jpeg, png, gif, bmp, webp, svg |
| video     | 1GB   | mp4, mkv, flv, avi, mov, wmv, webm  |
