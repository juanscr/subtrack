use std::path::Path;

use anyhow::{anyhow, Result};

use super::{
    encoding::get_file_with_utf8_encoding, format::SubtitleFormat, language::Language,
    mode::SubtitleMode,
};

#[derive(Default)]
pub struct SubtitleFileBuilder {
    file_name: Option<Box<str>>,
    language: Option<Language>,
    format: Option<SubtitleFormat>,
    mode: Option<SubtitleMode>,
    is_original_subtitle_file: bool,
}

impl SubtitleFileBuilder {
    pub fn new() -> Self {
        SubtitleFileBuilder {
            ..Default::default()
        }
    }

    pub fn with_file<S>(self, file_name: S, subtitle_mode: SubtitleMode) -> Result<Self>
    where
        S: AsRef<str>,
    {
        let file = Path::new(file_name.as_ref());
        if !file.exists() {
            return Err(anyhow!(
                "Subtitle file {} does not exist. Please select an existing file.",
                file_name.as_ref()
            ));
        }
        if !file.is_file() {
            return Err(anyhow!(
                "Subtitle {} is not a file. Please select a valid file path.",
                file_name.as_ref()
            ));
        }
        let format = SubtitleFormat::new(&file)?;
        let encoders = self
            .language
            .as_ref()
            .map_or(None, |v| v.preferred_encoders());

        let (subtitle_file_name, is_transformed) =
            get_file_with_utf8_encoding(file, &format, encoders, &subtitle_mode)?;
        Ok(SubtitleFileBuilder {
            file_name: Some(subtitle_file_name),
            format: Some(format),
            mode: Some(subtitle_mode),
            is_original_subtitle_file: !is_transformed,
            ..self
        })
    }

    pub fn with_language(self, language: Language) -> Self {
        SubtitleFileBuilder {
            language: Some(language),
            ..self
        }
    }

    pub fn with_subtitle_option<S>(self, subtitle_option: S, mode: SubtitleMode) -> Result<Self>
    where
        S: AsRef<str>,
    {
        if !subtitle_option.as_ref().contains(",") {
            self.with_file(subtitle_option, mode)
        } else {
            let (subtitle_file, language) = subtitle_option.as_ref().rsplit_once(',').unwrap();
            self.with_language(Language::new(language)?)
                .with_file(subtitle_file, mode)
        }
    }

    pub fn build(self) -> Result<SubtitleFile> {
        let file_name = self
            .file_name
            .ok_or_else(|| anyhow!("A file name is required."))?;
        let format = self
            .format
            .ok_or_else(|| anyhow!("The file format is not define."))?;
        let mode = self.mode.ok_or_else(|| {
            anyhow!("The subtitle mode is not defined. Please select a valid mode.")
        })?;

        return Ok(SubtitleFile {
            file_name,
            format,
            language: self.language,
            mode,
            is_original_subtitle_file: self.is_original_subtitle_file,
        });
    }
}

pub struct SubtitleFile {
    pub language: Option<Language>,
    pub format: SubtitleFormat,
    pub file_name: Box<str>,
    pub mode: SubtitleMode,
    pub is_original_subtitle_file: bool,
}

impl Drop for SubtitleFile {
    fn drop(&mut self) {
        if self.is_original_subtitle_file || !self.mode.should_remove_file() {
            return;
        }
        std::fs::remove_file(self.file_name.as_ref()).unwrap();
    }
}
