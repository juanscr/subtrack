mod ffmpeg;
mod mode;
mod subtitle;
mod utils;
mod video;

use anyhow::{anyhow, Result};
use clap::{command, Parser};
use ffmpeg::add_subtitles_to_video;
use mode::Mode;
use subtitle::{file::SubtitleFileBuilder, mode::SubtitleMode};
use utils::parse_output_file;
use video::file::VideoFileBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// The video file to add the subtitles
    input_file: Box<str>,

    /// The name of the output video file
    #[arg(short, long)]
    output_file: Option<Box<str>>,

    /// The name of the output video file
    #[arg(short, long, default_value_t)]
    mode: Mode,

    /// The subtitle file and language separated by a comma.
    #[arg(short, long = "subtitle", value_name = "SUBTITLE,LANGUAGE")]
    subtitles: Vec<Box<str>>,

    /// Changes the way how the created subtitle files are handled
    #[arg(short = 'u', long, default_value_t)]
    subtitle_mode: SubtitleMode,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.subtitles.is_empty() {
        return Err(anyhow!("At least one subtitle file must be specified."));
    }

    // Obtain current subtitle stream of video
    let video_file = VideoFileBuilder::new()
        .with_input_file(args.input_file)?
        .build()?;

    // Get output file
    let output_file = parse_output_file(args.output_file, &video_file.file_name)?;
    if args.subtitles.len() > 1 && !output_file.supports_multiple_subtitle_streams() {
        return Err(anyhow!(
            "Video file with format {:?} does not support multiple subtitle streams.",
            video_file.format
        ));
    }

    // Parse subtitles and languages
    let mut subtitles = Vec::with_capacity(args.subtitles.len());
    for subtitle_option in args.subtitles {
        subtitles.push(
            SubtitleFileBuilder::new()
                .with_subtitle_option(subtitle_option, args.subtitle_mode)?
                .build()?,
        );
    }

    // Run ffmpeg command to add subtitles
    add_subtitles_to_video(&video_file, subtitles, &output_file, &args.mode)?;
    Ok(())
}
