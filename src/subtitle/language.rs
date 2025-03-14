use anyhow::{anyhow, Result};
use encoding_rs::Encoding;

pub enum Language {
    English,
    French,
    Spanish,
}

impl Language {
    pub fn new<S>(language: S) -> Result<Self>
    where
        S: AsRef<str>,
    {
        match language.as_ref().to_lowercase().as_str() {
            "spanish" => Ok(Self::Spanish),
            "english" => Ok(Self::English),
            "french" => Ok(Self::French),
            _ => Err(anyhow!("Language not supported")),
        }
    }

    pub fn to_metadata_tag(&self) -> Box<str> {
        match self {
            Language::Spanish => "spa".into(),
            Language::English => "eng".into(),
            Language::French => "fre".into(),
        }
    }

    pub fn preferred_encoders(&self) -> Option<Box<[&'static Encoding]>> {
        match self {
            Language::Spanish => Some(Box::new([
                encoding_rs::WINDOWS_1252,
                encoding_rs::ISO_8859_15,
            ])),
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spanish_tag() {
        assert_eq!(Language::Spanish.to_metadata_tag().as_ref(), "spa")
    }

    #[test]
    fn test_english_tag() {
        assert_eq!(Language::English.to_metadata_tag().as_ref(), "eng")
    }

    #[test]
    fn test_french_tag() {
        assert_eq!(Language::French.to_metadata_tag().as_ref(), "fre")
    }
}
