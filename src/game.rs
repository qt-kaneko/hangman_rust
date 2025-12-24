use std::{collections::BTreeSet, fmt::Display};
use super::{rand::random, wrap::{wrap, wrap_once}};

#[derive(PartialEq, Eq)]
pub enum State {
  Won,
  Lost,
  Playing,
}

pub struct Game {
  dictionary: Vec<(String, String)>,
  current: usize,
  guessed: BTreeSet<char>,
  definition_visible: bool,
  state: State,
}

impl Game {
  pub fn new(dictionary: Vec<(String, String)>) -> Self {
    let mut game = Self {
      dictionary,
      current: 0,
      guessed: BTreeSet::new(),
      definition_visible: true,
      state: State::Playing,
    };
    game.next_word();
    game
  }

  fn word(&self) -> &str {
    &self.dictionary[self.current].0
  }

  fn gloss(&self) -> &str {
    &self.dictionary[self.current].1
  }

  pub fn mistakes(&self) -> usize {
    self
      .guessed
      .iter()
      .filter(|&&c| !self.word().contains(c))
      .count()
  }

  pub fn won(&self) -> bool {
    self
      .word()
      .chars()
      .all(|c| self.guessed.contains(&c))
  }

  pub fn next_word(&mut self) {
    self.current = random(self.dictionary.len());

    self.guessed.clear();

    self.state = State::Playing;
  }

  pub fn toggle_definition(&mut self) {
    self.definition_visible = !self.definition_visible;
  }

  pub fn hint(&mut self) {
    let available: Vec<char> = self
      .word()
      .chars()
      .filter(|c| !self.guessed.contains(c))
      .collect();

    if let Some(letter) = available.get(random(available.len())) {
      self.guess(*letter);
    };
  }

  pub fn give_up(&mut self) {
    if self.state != State::Playing {
      return;
    }

    self.state = State::Lost;
  }

  pub fn guess(&mut self, letter: char) {
    if self.state != State::Playing {
      return;
    }

    self.guessed.insert(letter);

    if self.mistakes() > 5 {
      self.state = State::Lost;
    }
    else if self.won() {
      self.state = State::Won;
    }
  }
}

impl Display for Game {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    let word = self
      .word()
      .chars()
      .map(|c| if self.guessed.contains(&c) || self.state == State::Lost { c } else { '_' })
      .fold(String::new(), |mut acc, c| {
        if !acc.is_empty() { acc.push(' '); }
        acc.push(c);
        acc
      });

    let used_letters = self
      .guessed
      .iter()
      .fold(String::new(), |mut acc, c| {
        if !acc.is_empty() { acc.push(' '); }
        acc.push(*c);
        acc
      });
    let (used_letters_first, used_letters_rest) = wrap_once(&used_letters, 44);
    let used_letters_rest: Vec<&str> = wrap(used_letters_rest, 58).collect();

    let mistakes = self.mistakes();
    let head      = if mistakes > 0 || self.state == State::Lost { "o" } else { "" };
    let left_arm  = if mistakes > 1 || self.state == State::Lost { "/" } else { "" };
    let torso     = if mistakes > 2 || self.state == State::Lost { "|" } else { "" };
    let right_arm = if mistakes > 3 || self.state == State::Lost { "\\" } else { "" };
    let left_leg  = if mistakes > 4 || self.state == State::Lost { "/" } else { "" };
    let right_leg = if mistakes > 5 || self.state == State::Lost { "\\" } else { "" };

    let definition_visible = self.definition_visible || self.state == State::Playing;

    let definition = if definition_visible { self.gloss() } else { "" };
    let (definition_first, definition_rest) = wrap_once(definition, 46);
    let definition_rest: Vec<&str> = wrap(&definition_rest, 58).collect();

    let status = match self.state {
      State::Won => "You win",
      State::Lost => "R.I.P",
      State::Playing => "",
    };

    let show_definition = if definition_visible { "*" } else { "" };

    write!(
      f,
      include_str!("ui.txt"),
      word,
      used_letters_first,
      used_letters_rest.get(0).unwrap_or(&""),
      used_letters_rest.get(1).unwrap_or(&""),
      head, left_arm, torso, right_arm, left_leg, right_leg,
      definition_first,
      definition_rest.get(0).unwrap_or(&""),
      definition_rest.get(1).unwrap_or(&""),
      definition_rest.get(2).unwrap_or(&""),
      definition_rest.get(3).unwrap_or(&""),
      status,
      show_definition,
    )
  }
}