use std::io::prelude::*;
use std::io::BufReader;
use std::io::Read;
use std::{fs::File, path::Path};

use anyhow::{anyhow, Result};
use encoding_rs::Encoding;
use encoding_rs_io::DecodeReaderBytesBuilder;

use super::format::SubtitleFormat;
use super::handling::SubtitleHandling;

fn get_file_buffer(
    path: &Path,
    preferred_encoders: Option<Box<[&'static Encoding]>>,
) -> Result<(String, bool)> {
    let utf8_base_read = read_file(path, None);
    if let Ok(utf8_buffer) = utf8_base_read {
        return Ok((utf8_buffer, false));
    }

    // Try reading the file with other encodings
    if preferred_encoders.is_none() {
        return Err(anyhow!(
            "Failed to read the file {} with UTF-8 encoding.",
            path.display()
        ));
    }
    for encoder in preferred_encoders.unwrap().iter() {
        let encoded_read = read_file(path, Some(encoder));
        if let Ok(buffer) = encoded_read {
            return Ok((buffer, true));
        }
    }
    Err(anyhow!(
        "Failed to read the file {} with any of the preferred encoders.",
        path.display()
    ))
}

fn read_file(path: &Path, encoding: Option<&'static Encoding>) -> Result<String> {
    let file = File::open(path)?;
    let decoder = DecodeReaderBytesBuilder::new()
        .encoding(encoding)
        .build(file);
    let mut reader = BufReader::new(decoder);
    let mut utf8_buffer = String::new();
    let utf8_read = reader.read_to_string(&mut utf8_buffer);
    if utf8_read.is_ok() {
        return Ok(utf8_buffer);
    }
    return Err(anyhow!(
        "Failed to encode the file {} to UTF-8 encoding.",
        path.display()
    ));
}

fn has_dos_line_endings(buffer: &String) -> bool {
    buffer.contains("\r\n")
}

fn dos_to_unix_line_endings(buffer: &String) -> String {
    buffer.replace('\r', "")
}

pub fn get_file_with_utf8_encoding(
    file: &Path,
    format: &SubtitleFormat,
    preferred_encoders: Option<Box<[&'static Encoding]>>,
    handling: &SubtitleHandling,
) -> Result<(Box<str>, bool)> {
    let (file_buffer, is_transformed) = get_file_buffer(file, preferred_encoders)?;
    let has_dos_line_endings = has_dos_line_endings(&file_buffer);

    // If the file is already UTF-8 encoded and does not have DOS line endings, return it as is
    if !is_transformed && !has_dos_line_endings {
        let file_name = file.to_str().ok_or_else(|| {
            anyhow!(
                "The file {} is not valid UTF-8. Please rename the file.",
                file.display()
            )
        })?;
        return Ok((file_name.into(), false));
    }

    let mut decoded_buffer = file_buffer;
    if has_dos_line_endings {
        decoded_buffer = dos_to_unix_line_endings(&decoded_buffer);
    }

    let new_file_name = handling.get_file_name(file, format.to_extension())?;
    let mut file_buffer = File::create(new_file_name.as_ref())?;
    file_buffer.write_all(decoded_buffer.as_bytes())?;
    Ok((new_file_name.into(), true))
}
