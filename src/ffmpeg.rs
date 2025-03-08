use std::process::Command;

use anyhow::{anyhow, Result};

use crate::{
    subtitle::file::SubtitleFile,
    video::{file::VideoFile, format::VideoFormat},
};

fn get_args_for_adding_subtitles<'a, S, O>(
    video_file: &'a VideoFile,
    subtitles: S,
    output_name: O,
) -> Vec<String>
where
    S: AsRef<[SubtitleFile]>,
    O: AsRef<str>,
{
    let mut args = Vec::from([
        "-hide_banner".to_owned(),
        "-loglevel".to_owned(),
        "error".to_owned(),
        "-i".to_owned(),
        video_file.file_name.clone().into(),
    ]);

    for sub in subtitles.as_ref().iter() {
        args.extend([
            "-f".to_owned(),
            sub.format.to_ffmpeg_file_type().into(),
            "-i".to_owned(),
            sub.file_name.clone().into(),
        ]);
    }

    args.extend(["-map".into(), "0".into(), "-map".into(), "-0:s".into()]);
    for (i, sub) in subtitles.as_ref().iter().enumerate() {
        args.push("-map".into());
        if let Some(language) = &sub.language {
            args.extend([
                format!("{}", i + 1).into(),
                format!("-metadata:s:s:{}", i).into(),
                format!("language={}", language.to_metadata_tag()).into(),
            ]);
        }
    }

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
) -> Result<()>
where
    S: AsRef<[SubtitleFile]>,
{
    if subtitles.as_ref().len() > 1 && !output_file.supports_multiple_subtitle_streams() {
        return Err(anyhow!(
            "Video file with format {:?} does not support multiple subtitle streams.",
            video_file.format
        ));
    }
    Command::new("ffmpeg")
        .args(get_args_for_adding_subtitles(
            video_file,
            &subtitles,
            &output_file.file_name,
        ))
        .spawn()?
        .wait()?;
    Ok(())
}
