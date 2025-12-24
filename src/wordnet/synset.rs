// https://wordnet.princeton.edu/documentation/wndb5wn

use std::{convert::Infallible, error::Error, str::{FromStr, SplitAsciiWhitespace}};
use super::{Word, SynsetType, ParseSynsetError};

pub struct Synset {
  pub synset_offset: u32,
  pub lex_filenum: u8,
  pub ss_type: SynsetType,
  pub words: Vec<Word>,
  pub gloss: String,
}

impl FromStr for Synset {
  type Err = ParseSynsetError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    fn read<T, E: Error + 'static>(fields: &mut SplitAsciiWhitespace, name: &'static str, parse: fn(&str) -> Result<T, E>) -> Result<T, ParseSynsetError> {
      let field = fields
        .next()
        .ok_or(ParseSynsetError::MissingField { name })?;

      parse(field)
        .map_err(|e| ParseSynsetError::InvalidFormat { name, source: Box::new(e) })
    }

    let (fields, gloss) = s
      .split_once(" | ")
      .ok_or(ParseSynsetError::MissingDelimiter {})?;

    let mut fields = fields.split_ascii_whitespace();
    let gloss = gloss.trim_end_matches("  ").to_owned();

    let synset_offset = read(&mut fields, "synset_offset", FromStr::from_str)?;

    let lex_filenum = read(&mut fields, "lex_filenum", FromStr::from_str)?;

    let ss_type = read(&mut fields, "ss_type", FromStr::from_str)?;

    let w_cnt = read(&mut fields, "w_cnt", |s| u8::from_str_radix(s, 16))?;

    let mut words = Vec::with_capacity(w_cnt as usize);
    for _ in 0..w_cnt {
      let word = read(&mut fields, "word", |s| Ok::<_, Infallible>(s.replace("_", " ")))?;
      let lex_id = read(&mut fields, "lex_id", |s| u8::from_str_radix(s, 16))?;
      words.push(Word { word, lex_id });
    }

    Ok(Synset {
      synset_offset,
      lex_filenum,
      ss_type,
      words,
      gloss,
    })
  }
}