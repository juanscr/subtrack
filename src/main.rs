mod ffmpeg;
mod subtitle;
mod utils;
mod video;

use anyhow::{anyhow, Result};
use clap::{command, Parser};
use ffmpeg::add_subtitles_to_video;
use subtitle::file::{SubtitleFile, SubtitleFileBuilder};
use utils::{fill_subtitle_builder, parse_output_file};
use video::file::VideoFileBuilder;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    input_file: Box<str>,

    #[arg(short, long)]
    output_file: Option<Box<str>>,

    #[arg(short, long)]
    subtitles: Vec<Box<str>>,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    if args.subtitles.is_empty() {
        return Err(anyhow!("At least one subtitle file must be specified."));
    }

    // Obtain current subtitle stream of video
    let video_file = VideoFileBuilder::new()
        .with_file(args.input_file)?
        .build()?;

    // Parse subtitles and languages
    let mut subtitles = Vec::<SubtitleFile>::new();
    for subtitle_option in args.subtitles {
        let mut builder = SubtitleFileBuilder::new();
        fill_subtitle_builder(&mut builder, subtitle_option)?;
        subtitles.push(builder.build()?);
    }

    // Get output file
    let output_file = parse_output_file(args.output_file, &video_file.file_name)?;

    // Run ffmpeg command to add subtitles
    add_subtitles_to_video(&video_file, subtitles, &output_file)?;
    Ok(())
}
