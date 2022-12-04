use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::path::Path;

const input_file : &'static str = "inputs/puzzle_2.txt";
const fake_input_file : &'static str = "inputs/puzzle_2_fake.txt";

#[derive(PartialEq, Clone, Eq, Debug)]
enum Play {
  Rock,
  Paper,
  Scissors,
}

/// Every line in the input must parse into one of these forms.
#[derive(PartialEq, Clone, Eq, Debug)]
struct Entry {
  them: Play,
  me: Play,
}

impl Entry {
  pub fn score(&self) -> u32 {
    let Entry{ them, me } = self;
    let win_portion = match (me, them) {
      (Play::Rock,     Play::Scissors) => 6,
      (Play::Scissors, Play::Paper) => 6,
      (Play::Paper,    Play::Rock) => 6,
      _ if them == me => 3,
      _ => 0,
    };
    let me_portion = match me {
      Play::Rock => 1,
      Play::Paper => 2,
      Play::Scissors => 3,
    };
    me_portion + win_portion
  }
}

impl FromStr for Entry {
  type Err = &'static str;
  fn from_str(s: &str) -> Result<Self, &'static str> {
    let mut chars = s.chars();
    let them_char = chars.next();
    let space = chars.next();
    let me_char = chars.next();
    if space != Some(' ') {
      return Err("Bar parse for space");
    }
    let them = match them_char {
      Some('A') => Play::Rock,
      Some('B') => Play::Paper,
      Some('C') => Play::Scissors,
      _ => {
        return Err("Bad parse for them");
      }
    };
    let me = match (me_char, them.clone()) {
      (Some('X'),Play::Rock) => Play::Scissors,
      (Some('X'),Play::Paper) => Play::Rock,
      (Some('X'),Play::Scissors) => Play::Paper,
      (Some('Y'),Play::Rock) => Play::Rock,
      (Some('Y'),Play::Paper) => Play::Paper,
      (Some('Y'),Play::Scissors) => Play::Scissors,
      (Some('Z'),Play::Rock) => Play::Paper,
      (Some('Z'),Play::Paper) => Play::Scissors,
      (Some('Z'),Play::Scissors) => Play::Rock,
      _ => {
        return Err("Bad parse for me");
      }
    };
    Ok(Entry {them, me})
  }
}

/// As we traverse the inputs, we will update this state.
#[derive(Debug)]
struct State {
  score: u32,
}

impl State {

  pub fn new() -> State {
    State { score: 0 }
  }

  /// Update the state according to an entry.
  pub fn step(&mut self, entry: &Entry) {
    self.score = self.score + entry.score();
  }

}

pub fn run<'a, I>(lines: I) -> Result<(u32, u32), &'static str>
  where I: Iterator< Item = String >
{
  let mut state = State::new();
  let mut state2 = State2::new();
  for line in lines {
    match Entry::from_str(&line) {
      Ok(entry) => {
        state.step(&entry);
        state2.step(&entry);
      },
      Err(e) => {
        return Err(e);
      }
    }
  }
  Ok((state.score, state2.score))
}

pub fn file_lines(path: &str) -> Box<dyn Iterator<Item = String>> {
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines().into_iter().map(|l| l.unwrap());
  Box::new(lines)
}

#[derive(Debug)]
struct State2 {
  score: u32,
}

impl State2 {
  pub fn new() -> State2 {
    State2 { score: 0}
  }

  pub fn step(&mut self, entry: &Entry) {
  }

}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn d2_real() {
    let input = file_lines(input_file);
    assert_eq!(run(input.into_iter()), Ok((11258, 0)));
  }

}
