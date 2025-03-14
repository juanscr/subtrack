<h1 align="center">subtrack</h1>
<div>
    <h6 align="right">
        <img align="left" src="https://img.shields.io/badge/license-GPLv3-blue.svg" alt="License">
        <img align="left" src="https://img.shields.io/badge/rust-1.56%2B-orange.svg" alt="Rust Version">
        Yet another ffmpeg wrapper!
    </h6>
</div>

Seamlessly add subtitles to your video files with ease!

- **Automatic Subtitle Detection**: Automatically detects subtitle formats and encodes the content to UTF8.
- **Multiple Subtitle Streams**: Supports adding multiple subtitle streams to your video files, with the proper language metadata.
- **Format Support**: Works with video formats like MP4 and MKV.

## Table of contents

- [Installation](#installation)
- [Usage](#usage)
  - [Options](#options)
  - [Examples](#examples)
- [License](#license)
- [Contributing](#contributing)
- [Acknowledgements](#acknowledgements)

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

- `-o, --output-file <OUTPUT_FILE>`: The name of the output video file. If not provided, the name of your input file with the suffix `-subs` will be created.
- `-s, --subtitle <SUBTITLE_FILE,LANGUAGE>`: The subtitle file and language separated by a comma. This option can be used multiple times to add multiple subtitles.
- `-m, --mode <MODE>`: How subtitles are added to the video container. Options include `append`, `replace` and `overwrite`.
- `-u, --subtitle-mode <SUBTITLE_MODE>`: Changes the way how the created fixed subtitle files are handled. Options include `keep`, `replace` and `remove`.

### Examples

Add a single subtitle to a video:

```sh
subtrack --subtitle my_subtitle.srt,english my_video.mp4
```

Add multiple subtitles to a video:

```sh
subtrack -s my_subtitle_en.srt,english -s my_subtitle_es.srt,spanish my_video.mkv
```

Add subtitles with a custom output file name:

```sh
subtrack -o my_output_video.mp4 -s my_subtitle.srt,english my_video.mp4
```

Add new subtitles while keeping the original ones:

```sh
subtrack -m append -s my_subtitle.srt,english my_video.mp4
```

## License

This project is licensed under the GNU General Public License v3.0. See the LICENSE file for details.

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## Acknowledgements

This CLI tool wouldn't be possible without this amazing tools:

- [FFmpeg](https://ffmpeg.org/) for the powerful multimedia framework.
- [Rust](https://www.rust-lang.org/) for the safe and fast programming language.

<p align="center">
  <img src="https://img.shields.io/badge/made%20with-rust-blue.svg" alt="Made with Rust">
</p>
