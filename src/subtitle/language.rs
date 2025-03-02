use anyhow::{anyhow, Result};

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
