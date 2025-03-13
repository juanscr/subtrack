use clap::ValueEnum;
use std::{fmt, vec};

use crate::subtitle::file::SubtitleFile;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Mode {
    /// Keep all pre-existing tracks and add the new ones.
    Append,

    /// Replace subtitle tracks with the same language, otherwise add them.
    Replace,

    /// Remove all pre-existing tracks and add the new ones.
    #[default]
    Overwrite,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Mode::Append => write!(f, "append"),
            Mode::Replace => write!(f, "replace"),
            Mode::Overwrite => write!(f, "overwrite"),
        }
    }
}

impl Mode {
    pub fn to_ffmpeg_flags<S>(&self, subtitles: S) -> Option<Vec<String>>
    where
        S: AsRef<[SubtitleFile]>,
    {
        match self {
            // Don't negatively map any subtitles
            Mode::Append => None,

            // Negative map all subtitles that have the languages present in the subtitles list
            Mode::Replace => {
                let mut flags = Vec::<String>::new();
                for sub in subtitles.as_ref().iter() {
                    if let Some(language) = &sub.language {
                        flags.extend([
                            "-map".into(),
                            format!("-0:s:m:language:{}", language.to_metadata_tag()),
                        ]);
                    }
                }
                if flags.is_empty() {
                    return None;
                }
                return Some(flags);
            }

            // Negative map all subtitles tracks from the original
            Mode::Overwrite => Some(vec!["-map".into(), "-0:s".into()]),
        }
    }
}
