use std::time::Duration;

use anyhow::{Error, Result};
use console::{style, Emoji, Term};
use indicatif::ProgressBar;

use crate::subtitle::language::Language;

pub struct CLILogger {
    term: Term,
    total_steps: u8,
}

fn get_count_step(step: u8, total_steps: u8) -> String {
    format!("[{}/{}]", step, total_steps)
}

impl CLILogger {
    pub fn new(subtitles_length: usize) -> Self {
        CLILogger {
            term: Term::stdout(),
            total_steps: (3 + subtitles_length) as u8,
        }
    }

    fn get_count_step(&self, step: u8) -> String {
        get_count_step(step, self.total_steps)
    }

    pub fn get_subtitle_logger(&self, step: u8) -> SubtitleLogger {
        SubtitleLogger::new(self.total_steps, step)
    }

    pub fn report_video_building(&self) -> Result<()> {
        let count_step = self.get_count_step(1);
        let emoji_with_count = format!("🎬 {}", count_step);
        let emoji = Emoji(&emoji_with_count, &count_step);
        Ok(self.term.write_line(&format!(
            "{} Building video file and subtitles...",
            style(emoji).green(),
        ))?)
    }

    pub fn report_output_file_parsing<O>(&self, output_file_name: O) -> Result<()>
    where
        O: AsRef<str>,
    {
        let count_step = self.get_count_step(2);
        let emoji_with_count = format!("📖 {}", count_step);
        let emoji = Emoji(&emoji_with_count, &count_step);
        Ok(self.term.write_line(&format!(
            "{} Parsing output file {}...",
            style(emoji).green(),
            output_file_name.as_ref()
        ))?)
    }

    pub fn report_ffmpeg_started(&self) -> Result<ProgressBar> {
        let bar = ProgressBar::new_spinner();
        bar.set_style(
            indicatif::ProgressStyle::default_spinner().template("{spinner:.green}  {msg}")?,
        );
        let count_step = self.get_count_step(self.total_steps);
        let message = format!("{} Running ffmpeg to add subtitles...", count_step);
        bar.set_message(message);
        bar.enable_steady_tick(Duration::from_millis(100));
        Ok(bar)
    }

    pub fn finish_ffmpeg(&self, bar: &ProgressBar) -> Result<()> {
        bar.finish();

        let count_step = self.get_count_step(self.total_steps);
        let emoji_with_count = format!("🚀 {}", count_step);
        let emoji = Emoji(&emoji_with_count, &count_step);
        self.term.write_line(&format!(
            "{} Finished processing video file!",
            style(emoji).green()
        ))?;
        Ok(())
    }

    pub fn report_error(&self, e: Error) -> Result<()> {
        let term = Term::stderr();
        term.write_line("")?;
        term.write_line(&format!(" {} Failed: {}", style(Emoji("✖️", "X")).red(), e))?;
        std::process::exit(1);
    }
}

pub struct SubtitleLogger {
    step: u8,
    total_steps: u8,
    term: Term,
}

impl SubtitleLogger {
    fn new(total_steps: u8, step: u8) -> Self {
        SubtitleLogger {
            step,
            total_steps,
            term: Term::stdout(),
        }
    }

    fn get_count_step(&self) -> String {
        get_count_step(self.step + 2, self.total_steps)
    }

    pub fn report_subtitle_parsing_done<S>(
        &self,
        language: &Option<Language>,
        file_name: S,
    ) -> Result<()>
    where
        S: AsRef<str>,
    {
        let count_step = self.get_count_step();
        let emoji_with_count = format!("✅ {}", count_step);
        let emoji = Emoji(&emoji_with_count, &count_step);
        let string = if let Some(language) = language {
            language.to_string()
        } else {
            file_name.as_ref().to_string()
        };
        Ok(self.term.write_line(&format!(
            "{} Subtitle file {} parsed successfully.",
            style(emoji).green(),
            string
        ))?)
    }
}
