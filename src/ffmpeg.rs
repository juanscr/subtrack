use std::process::Command;

use anyhow::Result;

use crate::{
    mode::Mode,
    subtitle::file::SubtitleFile,
    video::{file::VideoFile, format::VideoFormat},
};

fn get_args_for_adding_subtitles<'a, S, O>(
    video_file: &'a VideoFile,
    subtitles: S,
    output_name: O,
    mode: &Mode,
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
            sub.format.to_extension().into(),
            "-i".to_owned(),
            sub.file_name.clone().into(),
        ]);
    }

    args.extend(["-map".into(), "0".into()]);
    if let Some(mode_ffmpeg_flags) = mode.to_ffmpeg_flags(&subtitles) {
        args.extend(mode_ffmpeg_flags);
    }
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
    mode: &Mode,
) -> Result<()>
where
    S: AsRef<[SubtitleFile]>,
{
    Command::new("ffmpeg")
        .args(get_args_for_adding_subtitles(
            video_file,
            &subtitles,
            &output_file.file_name,
            mode,
        ))
        .spawn()?
        .wait()?;
    Ok(())
}
