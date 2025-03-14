use std::path::Path;

use anyhow::{anyhow, Result};

use super::{encoding::get_file_with_utf8_encoding, format::SubtitleFormat, language::Language};

#[derive(Default)]
pub struct SubtitleFileBuilder {
    file_name: Option<Box<str>>,
    language: Option<Language>,
    format: Option<SubtitleFormat>,
}

impl SubtitleFileBuilder {
    pub fn new() -> Self {
        SubtitleFileBuilder {
            ..Default::default()
        }
    }

    pub fn with_file<S>(self, file_name: S) -> Result<Self>
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

        Ok(SubtitleFileBuilder {
            file_name: Some(get_file_with_utf8_encoding(file, &format)?),
            format: Some(format),
            ..self
        })
    }

    pub fn with_language(self, language: Language) -> Self {
        SubtitleFileBuilder {
            language: Some(language),
            ..self
        }
    }

    pub fn with_subtitle_option<S>(self, subtitle_option: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        if !subtitle_option.as_ref().contains(",") {
            self.with_file(subtitle_option)
        } else {
            let (subtitle_file, language) = subtitle_option.as_ref().rsplit_once(',').unwrap();
            self.with_language(Language::new(language)?)
                .with_file(subtitle_file)
        }
    }

    pub fn build(self) -> Result<SubtitleFile> {
        let file_name = self
            .file_name
            .ok_or_else(|| anyhow!("A file name is required."))?;
        let format = self
            .format
            .ok_or_else(|| anyhow!("The file format is not define."))?;
        return Ok(SubtitleFile {
            file_name,
            format,
            language: self.language,
        });
    }
}

pub struct SubtitleFile {
    pub language: Option<Language>,
    pub format: SubtitleFormat,
    pub file_name: Box<str>,
}
