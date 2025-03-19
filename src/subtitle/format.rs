use std::path::Path;

use anyhow::{anyhow, Result};

use crate::utils::get_file_extension;

pub enum SubtitleFormat {
    Srt,
}

impl SubtitleFormat {
    pub fn new(file: &Path) -> Result<Self> {
        let file_extension = get_file_extension(file)?;
        match file_extension.as_ref() {
            "srt" => Ok(SubtitleFormat::Srt),
            extension => Err(anyhow!("File extension {} not supported.", extension)),
        }
    }

    pub fn to_extension(&self) -> Box<str> {
        match self {
            SubtitleFormat::Srt => "srt".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_srt_file() {
        assert_eq!(SubtitleFormat::Srt.to_extension().as_ref(), "srt")
    }
}
