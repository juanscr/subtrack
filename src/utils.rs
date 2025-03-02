use std::path::Path;

use anyhow::{anyhow, Result};

use crate::subtitle::{file::SubtitleFileBuilder, language::Language};

pub fn fill_subtitle_builder<S>(
    builder: &mut SubtitleFileBuilder,
    subtitle_option: S,
) -> Result<&mut SubtitleFileBuilder>
where
    S: AsRef<str>,
{
    Ok(if !subtitle_option.as_ref().contains(",") {
        builder.with_file(subtitle_option)?
    } else {
        let (subtitle_file, language) = subtitle_option.as_ref().rsplit_once(',').unwrap();
        builder
            .with_file(subtitle_file)?
            .with_language(Language::new(language)?)
    })
}

pub fn parse_output_file<O, I>(output_file: Option<O>, input_file: I) -> Result<Box<str>>
where
    O: AsRef<str>,
    I: AsRef<str>,
{
    let input_file_path = Path::new(input_file.as_ref());
    if output_file.is_none() {
        let stem = input_file_path
            .file_stem()
            .ok_or_else(|| anyhow!("Could not extract stem from input file."))?
            .to_str()
            .ok_or_else(|| anyhow!("Could not extract stem from input file."))?;
        let extension = input_file_path
            .extension()
            .ok_or_else(|| anyhow!("Could not extract file extension."))?
            .to_str()
            .ok_or_else(|| anyhow!("Could not extract file extension."))?;
        return parse_output_file(format!("{}-subs.{}", stem, extension).into(), input_file);
    }

    let output_file_name = output_file.unwrap();
    let output_file_path = Path::new(output_file_name.as_ref());
    if output_file_path == input_file_path {
        return Err(anyhow!("Output file can't be the same path as input file"));
    }
    if output_file_path.exists() {
        return Err(anyhow!("Output file path already exists."));
    }
    Ok(output_file_name.as_ref().into())
}
