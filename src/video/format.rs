use std::path::Path;

use anyhow::{anyhow, Result};

#[derive(Debug, PartialEq, Eq)]
pub enum VideoFormat {
    MP4,
    MKV,
}

impl VideoFormat {
    // TODO: Rewrite this to not relay in the extension but internal metadata
    pub fn new(file: &Path) -> Result<Self> {
        let file_extension = file
            .extension()
            .ok_or_else(|| anyhow!("File doesn't have an extension set."))?
            .to_str()
            .ok_or_else(|| anyhow!("File extension is ill-formed."))?;

        match file_extension {
            "mp4" => Ok(VideoFormat::MP4),
            "mkv" => Ok(VideoFormat::MKV),
            extension => Err(anyhow!("File extension {} not supported.", extension)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_video_format_for_mp4() -> Result<()> {
        let file = Path::new("my_file.mp4");
        assert_eq!(VideoFormat::new(&file)?, VideoFormat::MP4);
        Ok(())
    }

    #[test]
    fn test_build_video_format_for_mkv() -> Result<()> {
        let file = Path::new("my_file.mkv");
        assert_eq!(VideoFormat::new(file)?, VideoFormat::MKV);
        Ok(())
    }

    #[test]
    #[should_panic(expected = "File extension txt not supported.")]
    fn test_build_video_format_for_other_extensions() {
        VideoFormat::new(Path::new("my_file.txt")).unwrap();
    }

    #[test]
    #[should_panic(expected = "File doesn't have an extension set.")]
    fn test_build_video_format_for_no_extensions() {
        VideoFormat::new(Path::new("my_file")).unwrap();
    }
}
