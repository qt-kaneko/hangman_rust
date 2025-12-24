use std::{str::FromStr};
use super::{ParseSynsetTypeError};

pub enum SynsetType {
  Noun,
  Verb,
  Adjective,
  AdjectiveSatellite,
  Adverb,
}

impl FromStr for SynsetType {
  type Err = ParseSynsetTypeError;

  fn from_str(input: &str) -> Result<Self, Self::Err> {
    match input {
      "n" => Ok(SynsetType::Noun),
      "v" => Ok(SynsetType::Verb),
      "a" => Ok(SynsetType::Adjective),
      "s" => Ok(SynsetType::AdjectiveSatellite),
      "r" => Ok(SynsetType::Adverb),
      _ => Err(ParseSynsetTypeError),
    }
  }
}