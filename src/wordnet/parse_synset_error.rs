use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ParseSynsetError {
  MissingField {
    name: &'static str,
  },
  MissingDelimiter,
  InvalidFormat {
    name: &'static str,
    source: Box<dyn Error>,
  },
}

impl Error for ParseSynsetError {
  fn source(&self) -> Option<&(dyn Error + 'static)> {
    match self {
      ParseSynsetError::InvalidFormat { source, .. } => Some(source.as_ref()),
      _ => None,
    }
  }
}

impl Display for ParseSynsetError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      ParseSynsetError::MissingField { name } => {
        write!(f, "Missing '{name} field'")
      }
      ParseSynsetError::MissingDelimiter => {
        write!(f, "Missing '|' delimiter")
      }
      ParseSynsetError::InvalidFormat { name, .. } => {
        write!(f, "Invalid format for '{name}'")
      }
    }
  }
}