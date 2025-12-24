use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub struct ParseSynsetTypeError;

impl Error for ParseSynsetTypeError {
}

impl Display for ParseSynsetTypeError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    write!(f, "Failed to parse synset type")
  }
}