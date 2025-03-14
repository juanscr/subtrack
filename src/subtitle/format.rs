use std::path::Path;

use anyhow::{anyhow, Result};

pub enum SubtitleFormat {
    Srt,
}

impl SubtitleFormat {
    pub fn new(file: &Path) -> Result<Self> {
        let file_extension = file
            .extension()
            .ok_or_else(|| anyhow!("File doesn't have an extension set."))?
            .to_str()
            .ok_or_else(|| anyhow!("File extension is ill-formed."))?;

        match file_extension {
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
