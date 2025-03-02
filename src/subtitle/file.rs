use std::path::Path;

use anyhow::{anyhow, Result};

use super::{format::SubtitleFormat, language::Language};

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

    pub fn with_file<S>(&mut self, file_name: S) -> Result<&mut Self>
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
        self.file_name = Some(file_name.as_ref().into());
        self.format = Some(format);
        Ok(self)
    }

    pub fn with_language(&mut self, language: Language) -> &mut Self {
        self.language = Some(language);
        self
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
