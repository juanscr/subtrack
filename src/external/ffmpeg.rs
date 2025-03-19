use std::process::Command;

use anyhow::Result;

use crate::{
    behavior::Behavior,
    subtitle::file::SubtitleFile,
    video::{file::VideoFile, format::VideoFormat},
};

fn get_args_for_adding_subtitles<'a, S, O>(
    video_file: &'a VideoFile,
    subtitles: S,
    output_name: O,
    behavior: &Behavior,
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
    args.extend(["-map".into(), "0".into()]);
    for i in 0..subtitles.as_ref().len() {
        args.extend(["-map".to_owned(), format!("{}", i + 1).into()]);
    }

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
) -> Result<()>
where
    S: AsRef<[SubtitleFile]>,
{
    Command::new("ffmpeg")
        .args(get_args_for_adding_subtitles(
            video_file,
            &subtitles,
            &output_file.file_name,
            behavior,
        ))
        .spawn()?
        .wait()?;
    Ok(())
}
