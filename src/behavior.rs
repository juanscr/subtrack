use clap::ValueEnum;
use std::{fmt, vec};

use crate::{subtitle::file::SubtitleFile, video::file::VideoFile};

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Behavior {
    /// Keep all pre-existing tracks and add the new ones.
    Append,

    /// Remove all pre-existing tracks and add the new ones.
    #[default]
    Overwrite,
}

impl fmt::Display for Behavior {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Behavior::Append => write!(f, "append"),
            Behavior::Overwrite => write!(f, "overwrite"),
        }
    }
}

fn add_subtitles_to_video<S, F>(args: &mut Vec<String>, subtitles: S, get_index: F) -> ()
where
    S: AsRef<[SubtitleFile]>,
    F: Fn(usize) -> usize,
{
    for (i, sub) in subtitles.as_ref().iter().enumerate() {
        if let Some(language) = &sub.language {
            args.extend([
                format!("-metadata:s:s:{}", get_index(i)).into(),
                format!("language={}", language.to_metadata_tag()).into(),
            ]);
        }
    }
}

impl Behavior {
    pub fn get_args_for_adding_subtitles<S>(
        &self,
        video_file: &VideoFile,
        subtitles: S,
    ) -> Vec<String>
    where
        S: AsRef<[SubtitleFile]>,
    {
        match self {
            // Don't negatively map any subtitles
            Behavior::Append => {
                let number_of_subtitles = video_file.get_number_of_subtitles(None);
                let mut args = Vec::new();
                add_subtitles_to_video(&mut args, subtitles, |i| i + number_of_subtitles);
                args
            }

            // Negative map all subtitles tracks from the original
            Behavior::Overwrite => {
                let mut args = vec!["-map".into(), "-0:s".into()];
                add_subtitles_to_video(&mut args, subtitles, |i| i + 1);
                args
            }
        }
    }
}
