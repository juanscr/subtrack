use std::{fmt, path::Path};

use anyhow::{anyhow, Result};
use clap::ValueEnum;

use crate::utils::get_file_stem;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum SubtitleMode {
    /// Keep all generated subtitle files when fixing the encoding.
    Keep,

    /// Replace the original subtitle files with the fixed ones.
    Replace,

    #[default]
    /// Remove the generated subtitle files while keeping the original ones.
    Remove,
}

impl fmt::Display for SubtitleMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SubtitleMode::Keep => write!(f, "keep"),
            SubtitleMode::Replace => write!(f, "replace"),
            SubtitleMode::Remove => write!(f, "remove"),
        }
    }
}

impl SubtitleMode {
    pub fn get_file_name(&self, file: &Path, extension: Box<str>) -> Result<Box<str>> {
        let file_stem = get_file_stem(file)?;
        match self {
            SubtitleMode::Replace => Ok(file
                .to_str()
                .ok_or_else(|| anyhow!("The file name is ill-formed. Please select a valid file."))?
                .into()),
            _ => Ok(format!("{}-fixed.{}", file_stem, extension).into()),
        }
    }

    pub fn should_remove_file(&self) -> bool {
        matches!(self, SubtitleMode::Remove)
    }
}
