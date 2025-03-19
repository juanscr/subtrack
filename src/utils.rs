use std::path::Path;

use anyhow::{anyhow, Result};

use crate::video::file::{VideoFile, VideoFileBuilder};

pub fn parse_output_file<O, I>(output_file: Option<O>, input_file: I) -> Result<VideoFile>
where
    O: AsRef<str>,
    I: AsRef<str>,
{
    let input_file_path = Path::new(input_file.as_ref());
    if output_file.is_none() {
        let stem = get_file_stem(input_file_path)?;
        let extension = get_file_extension(input_file_path)?;
        return parse_output_file(format!("{}-subs.{}", stem, extension).into(), input_file);
    }

    let output_file_name = output_file.unwrap();
    let output_file_path = Path::new(output_file_name.as_ref());
    if output_file_path == input_file_path {
        return Err(anyhow!("Output file can't be the same path as input file"));
    }
    VideoFileBuilder::new()
        .with_output_file(output_file_name)?
        .build()
}

pub fn get_file_stem(file: &Path) -> Result<Box<str>> {
    let file_name = file.display();
    file.file_stem()
        .ok_or_else(|| anyhow!("The file {} doesn't have a file name.", file_name))?
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "The file {} is not valid UTF-8. Please rename the file.",
                file_name
            )
        })
        .map(|s| s.into())
}

pub fn get_file_extension(file: &Path) -> Result<Box<str>> {
    let file_name = file.display();
    file.extension()
        .ok_or_else(|| {
            anyhow!(
                "The file {} doesn't have an extension. Please rename it.",
                file_name
            )
        })?
        .to_str()
        .ok_or_else(|| {
            anyhow!(
                "The file {} is not valid UTF-8. Please rename the file.",
                file_name
            )
        })
        .map(|s| s.into())
}
