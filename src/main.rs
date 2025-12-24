use std::{error::Error, fs::File, io::{self, BufRead, BufReader, Write}, process::Command, str::FromStr};

pub mod wordnet;
use wordnet::Synset;

mod game;
use game::Game;

mod rand;
mod wrap;

fn main() -> Result<(), Box<dyn Error>> {
  let data_file = File::open("data.noun")?;
  let data_reader = BufReader::new(data_file);
  let data: Vec<Synset> = data_reader
    .lines()
    .filter(|line| !line.as_ref().is_ok_and(|line| line.starts_with("  ")))
    .map(|line| {
      let line = line?;
      Ok(Synset::from_str(&line)?)
    })
    .collect::<Result<_, Box<dyn Error>>>()?;

  let dictionary: Vec<(String, String)> = data
    .iter()
    .filter_map(|synset| {
      let word = &synset.words.get(0)?.word;
      if !word.chars().all(|c| c.is_alphabetic() && c.is_lowercase()) {
        return None;
      }

      let mut gloss = synset
        .gloss
        .split(';')
        .next()
        .unwrap()
        .to_owned();

      if !gloss.ends_with('.') {
        gloss.push('.');
      }

      Some((word.to_owned(), gloss))
    })
    .collect();

  let mut game = Game::new(dictionary);
  let mut rick_rolled = false;

  loop {
    clear();

    print!("{}", game);
    io::stdout().flush()?;

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    for char in input.trim_end().chars() {
      match char {
        '1' => game.next_word(),
        '2' => game.toggle_definition(),
        '3' => game.hint(),
        '4' => {
          game.give_up();

          if !rick_rolled {
            rick_rolled = true;
            open_url("https://youtu.be/dQw4w9WgXcQ?t=43")?;
          }
        },
        '5' => return Ok(()),
        c => {
          let letter = c
            .to_lowercase()
            .next()
            .expect("c should not be empty");
          if !letter.is_alphabetic() { continue; }

          game.guess(letter);
        },
      }
    }
  }
}

fn open_url(url: &str) -> io::Result<()> {
  Command
    ::new(
      #[cfg(target_os = "windows")] "rundll32",
      #[cfg(target_os = "linux")]   "xdg-open",
      #[cfg(target_os = "macos")]   "open",
    )
    .args(
      #[cfg(target_os = "windows")] ["url.dll,FileProtocolHandler", url],
      #[cfg(target_os = "linux")]   [url],
      #[cfg(target_os = "macos")]   [url],
    )
    .spawn()?;

  Ok(())
}

fn clear() {
  #[cfg(windows)]
  Command::new("cmd")
    .args(["/c", "cls"])
    .status()
    .unwrap();

  #[cfg(unix)]
  print!("\x1Bc");
}