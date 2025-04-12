use clap::ValueEnum;
use std::fmt;

use crate::subtitle::file::SubtitleFile;

#[derive(Copy, Clone, Default, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum Behavior {
    /// Keep all pre-existing tracks and add the new ones.
    #[default]
    Append,

    /// Remove all pre-existing tracks and add the new ones.
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

fn add_subtitles_to_video<S>(args: &mut Vec<String>, subtitles: S) -> ()
where
    S: AsRef<[SubtitleFile]>,
{
    for (i, sub) in subtitles.as_ref().iter().enumerate() {
        if let Some(language) = &sub.language {
            args.extend([
                format!("-metadata:s:s:{}", i).into(),
                format!("language={}", language.to_metadata_tag()).into(),
            ]);
        }
    }
}

impl Behavior {
    pub fn get_args_for_adding_subtitles<S>(&self, subtitles: S) -> Vec<String>
    where
        S: AsRef<[SubtitleFile]>,
    {
        let mut args = Vec::new();
        match self {
            // Don't negatively map any subtitles
            Behavior::Append => {
                args.extend(["-map".into(), "0:s?".into()]);
                add_subtitles_to_video(&mut args, subtitles);
                args
            }

            // Negative map all subtitles tracks from the original
            Behavior::Overwrite => {
                add_subtitles_to_video(&mut args, subtitles);
                args
            }
        }
    }
}
