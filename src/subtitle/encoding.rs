use std::io::prelude::*;
use std::io::Read;
use std::{fs::File, path::Path};

use anyhow::{anyhow, Result};
use encoding_rs::{Encoding, ISO_8859_15, UTF_8, WINDOWS_1252};

use crate::utils::get_file_stem;

use super::format::SubtitleFormat;

fn get_file_buffer<S>(file_path: S) -> Result<Vec<u8>>
where
    S: AsRef<str>,
{
    let mut file = File::open(file_path.as_ref())?;
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer)?;
    Ok(buffer)
}

/// Check if a file is UTF-8 encoded
fn is_file_utf8_encoded<B>(buffer: B) -> bool
where
    B: AsRef<[u8]>,
{
    !UTF_8.decode(buffer.as_ref()).2
}

/// Pass for each of the encoders to find one without errors
const ENCODERS: [&Encoding; 2] = [ISO_8859_15, WINDOWS_1252];

fn change_encoding_to_utf8<'a, B>(buffer: &'a B) -> Result<Box<[u8]>>
where
    B: AsRef<[u8]>,
{
    for encoder in ENCODERS {
        let (result, _, failed_encoding) = encoder.decode(buffer.as_ref());

        // Check if the decoding was successful
        if !failed_encoding {
            return Ok(result.as_bytes().into());
        }
    }

    Err(anyhow!(
        "Failed to decode the file with any of the encoders."
    ))
}

pub fn get_file_with_utf8_encoding(file: &Path, format: &SubtitleFormat) -> Result<Box<str>> {
    let file_name = file
        .to_str()
        .ok_or_else(|| anyhow!("The file name is ill-formed. Please select a valid file."))?;

    // If it's a UTF-8 encoded file, do not try to detect the encoding
    if is_file_utf8_encoded(file_name) {
        return Ok(file_name.into());
    }

    let file_buffer = get_file_buffer(file_name)?;
    let decoded_buffer = change_encoding_to_utf8(&file_buffer)?;

    // Create a new file with the same name but with a -utf8 suffix and encoded content
    let file_stem = get_file_stem(file)?;
    let new_file_name = format!("{}-utf8.{}", file_stem, format.to_extension());
    let mut file_buffer = File::create(&new_file_name)?;
    file_buffer.write_all(&decoded_buffer)?;
    Ok(new_file_name.into())
}
