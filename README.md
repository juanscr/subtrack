<h1 align="center">subtrack</h1>
<h6 align="right">Yet another ffmpeg wrapper</h6>

<p align="center">
  <img src="https://img.shields.io/badge/license-GPLv3-blue.svg" alt="License">
  <img src="https://img.shields.io/badge/rust-1.56%2B-orange.svg" alt="Rust Version">
</p>

<p align="center">
  Seamlessly add subtitles to your video files with ease!
</p>

## Features

- **Automatic Subtitle Detection**: Automatically detects subtitle formats.
- **Multiple Subtitle Streams**: Supports adding multiple subtitle streams to your video files, with the proper language metadata.
- **Format Support**: Works with video formats like MP4 and MKV.

## Installation

To install `subtrack`, ensure you have Rust and Cargo installed, then run:

```sh
cargo install --path .
```

## Usage

```sh
subtrack [OPTIONS] <INPUT_FILE> --subtitle <SUBTITLE_FILE,LANGUAGE>...
```

### Options

- `-o, --output-file <OUTPUT_FILE>`: The name of the output video file.
- `-s, --subtitle <SUBTITLE_FILE,LANGUAGE>`: The subtitle file and language separated by a comma. This option can be used multiple times to add multiple subtitles.

### Examples

Add a single subtitle to a video:

```sh
subtrack --subtitle my_subtitle.srt,english my_video.mp4
```

Add multiple subtitles to a video:

```sh
subtrack --subtitle my_subtitle_en.srt,english --subtitle my_subtitle_es.srt,spanish my_video.mkv
```

## License

This project is licensed under the GNU General Public License v3.0. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

- [FFmpeg](https://ffmpeg.org/) for the powerful multimedia framework.
- [Rust](https://www.rust-lang.org/) for the safe and fast programming language.

<p align="center">
  <img src="https://img.shields.io/badge/made%20with-rust-blue.svg" alt="Made with Rust">
</p>
