use std::process::Command;

use anyhow::Result;

use crate::{
    behavior::Behavior,
    logger::CLILogger,
    subtitle::file::SubtitleFile,
    video::{file::VideoFile, format::VideoFormat},
};

fn get_args_for_adding_subtitles<'a, S>(
    video_file: &'a VideoFile,
    subtitles: S,
    output_file: &'a VideoFile,
    behavior: &Behavior,
) -> Vec<String>
where
    S: AsRef<[SubtitleFile]>,
{
    let mut args = Vec::from([
        "-hide_banner".to_owned(),
        "-loglevel".to_owned(),
        "error".to_owned(),
        "-i".to_owned(),
        video_file.get_file_name().into(),
    ]);

    for sub in subtitles.as_ref().iter() {
        args.extend([
            "-f".to_owned(),
            sub.format.to_extension().into(),
            "-i".to_owned(),
            sub.file_name.clone().into(),
        ]);
    }

    // Map all input streams to not lose metadata but remove subtitles
    args.extend(["-map".into(), "0".into(), "-map".into(), "-0:s".into()]);

    // Avoid mapping data stream for conversion between different formats
    if video_file.format != output_file.format {
        args.extend(["-map".into(), "-0:d".into()]);
    }

    // Map all provided subtitles
    for i in 0..subtitles.as_ref().len() {
        args.extend(["-map".to_owned(), format!("{}", i + 1).into()]);
    }

    // Add subtitles based on behavior selected by user
    args.extend(behavior.get_args_for_adding_subtitles(&subtitles));

    if video_file.format == VideoFormat::MP4 {
        args.extend(["-c:s".into(), "mov_text".into()]);
    }
    args.extend(["-c".into(), "copy".into(), output_name.as_ref().into()]);

    return args;
}

pub fn add_subtitles_to_video<S>(
    video_file: &VideoFile,
    subtitles: S,
    output_file: &VideoFile,
    behavior: &Behavior,
    logger: &CLILogger,
) -> Result<()>
where
    S: AsRef<[SubtitleFile]>,
{
    let bar = logger.report_ffmpeg_started()?;
    Command::new("ffmpeg")
        .args(get_args_for_adding_subtitles(
            video_file,
            &subtitles,
            &output_file,
            behavior,
        ))
        .spawn()?
        .wait()?;
    logger.finish_ffmpeg(&bar)?;
    Ok(())
}
