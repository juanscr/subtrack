use std::process::Command;

use anyhow::Result;

use crate::subtitle::language::Language;

fn get_args_for_number_of_subtitles<S>(file_name: S, language: Option<&Language>) -> Vec<String>
where
    S: AsRef<str>,
{
    let mut args = Vec::from([
        "-loglevel".to_owned(),
        "error".to_owned(),
        "-show_entries".to_owned(),
        "stream=index".to_owned(),
        "-select_streams".to_owned(),
    ]);

    if let Some(language) = language {
        args.push(format!("s:m:{}", language.to_metadata_tag()));
    } else {
        args.push("s".to_owned());
    }
    args.push(file_name.as_ref().into());
    return args;
}

pub fn get_number_of_subtitles<S>(file_name: S, language: Option<&Language>) -> Result<usize>
where
    S: AsRef<str>,
{
    let subtitle_streams = Command::new("ffprobe")
        .args(get_args_for_number_of_subtitles(file_name, language))
        .output()?
        .stdout;
    let number_of_new_lines = subtitle_streams.iter().filter(|&&x| x == b'\n').count();
    Ok(number_of_new_lines / 3)
}
