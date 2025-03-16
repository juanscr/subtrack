use std::path::Path;

use anyhow::{anyhow, Result};

use crate::{external::ffprobe::get_number_of_subtitles, subtitle::language::Language};

use super::format::VideoFormat;

#[derive(Default)]
pub struct VideoFileBuilder {
    file_name: Option<Box<str>>,
    format: Option<VideoFormat>,
}

impl VideoFileBuilder {
    pub fn new() -> Self {
        VideoFileBuilder {
            ..Default::default()
        }
    }

    pub fn with_input_file<S>(self, file_name: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        self.with_file(file_name, |file| {
            if !file.exists() {
                return Err(anyhow!(
                    "Video {} does not exist. Please select an existing file.",
                    file.display()
                ));
            }
            if !file.is_file() {
                return Err(anyhow!(
                    "Video {} is not a file. Please select a valid file path.",
                    file.display()
                ));
            }
            Ok(())
        })
    }

    pub fn with_output_file<S>(self, file_name: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        self.with_file(file_name, |file| {
            if file.exists() {
                return Err(anyhow!(
                    "Output file {} path already exists.",
                    file.display()
                ));
            }
            if file.is_dir() {
                return Err(anyhow!(
                    "Output {} is a directory. Please select a valid file path.",
                    file.display()
                ));
            }
            Ok(())
        })
    }

    pub fn with_file<S, F>(self, file_name: S, file_validator: F) -> Result<Self>
    where
        S: AsRef<str>,
        F: Fn(&Path) -> Result<()>,
    {
        let file = Path::new(file_name.as_ref());

        // Check if the file exists and is a file
        file_validator(&file)?;

        let format = VideoFormat::new(file)?;
        Ok(VideoFileBuilder {
            file_name: Some(file_name.as_ref().into()),
            format: Some(format),
            ..self
        })
    }

    pub fn build(self) -> Result<VideoFile> {
        let file_name = self
            .file_name
            .ok_or_else(|| anyhow!("A file name is required."))?;
        let format = self
            .format
            .ok_or_else(|| anyhow!("The file format is not define."))?;

        Ok(VideoFile {
            file_name: file_name.into(),
            format,
        })
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct VideoFile {
    pub file_name: Box<str>,
    pub format: VideoFormat,
}

impl VideoFile {
    pub fn supports_multiple_subtitle_streams(&self) -> bool {
        self.format != VideoFormat::MP4
    }

    pub fn get_file_name(&self) -> &str {
        self.file_name.as_ref()
    }

    pub fn get_number_of_subtitles(&self, language: Option<&Language>) -> usize {
        get_number_of_subtitles(self.get_file_name(), language).unwrap_or(0)
    }
}

#[cfg(test)]
mod tests {
    use tempfile::{Builder, TempDir};

    use super::*;

    #[test]
    pub fn test_video_builder_mp4() -> Result<()> {
        let video = Builder::new().prefix("my_file").suffix(".mp4").tempfile()?;
        let video_file_name = video.path().to_str().unwrap();
        let video_file = VideoFileBuilder::new()
            .with_input_file(video_file_name)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            video_file,
            VideoFile {
                file_name: video_file_name.into(),
                format: VideoFormat::MP4
            }
        );
        Ok(())
    }

    #[test]
    pub fn test_video_builder_mkv() -> Result<()> {
        let video = Builder::new().prefix("my_file").suffix(".mkv").tempfile()?;
        let video_file_name = video.path().to_str().unwrap();
        let video_file = VideoFileBuilder::new()
            .with_input_file(video_file_name)
            .unwrap()
            .build()
            .unwrap();

        assert_eq!(
            video_file,
            VideoFile {
                file_name: video_file_name.into(),
                format: VideoFormat::MKV
            }
        );
        Ok(())
    }

    #[test]
    #[should_panic(expected = "Input file does not exist. Please select an existing file.")]
    pub fn test_video_builder_non_existent_file() {
        VideoFileBuilder::new()
            .with_input_file("my input non existent file")
            .unwrap();
    }

    #[test]
    #[should_panic(expected = "Input is not a file. Please select a valid file path.")]
    pub fn test_video_builder_directory() {
        let temp_directory = TempDir::new().unwrap();
        let temp_directory_path = temp_directory.path().to_str().unwrap();
        VideoFileBuilder::new()
            .with_input_file(temp_directory_path)
            .unwrap();
    }

    #[test]
    pub fn test_mp4_not_supports_multiple_subtitle_streams() {
        let video_file = VideoFile {
            file_name: "test".into(),
            format: VideoFormat::MP4,
        };
        assert!(!video_file.supports_multiple_subtitle_streams());
    }

    #[test]
    pub fn test_mkv_supports_multiple_subtitle_streams() {
        let video_file = VideoFile {
            file_name: "test".into(),
            format: VideoFormat::MKV,
        };
        assert!(video_file.supports_multiple_subtitle_streams());
    }
}
