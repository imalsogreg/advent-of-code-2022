use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::path::Path;

const input_file : &'static str = "inputs/puzzle_1.txt";
const fake_input_file : &'static str = "inputs/puzzle_1_fake.txt";

/// Every line in the input must parse into one of these forms.
#[derive(PartialEq, Clone, Eq, Debug)]
enum Entry {
  Blank,
  Calories(i32),
}

impl FromStr for Entry {
  type Err = &'static str;
  fn from_str(s: &str) -> Result<Self, &'static str> {
    match s {
      "" => Ok(Entry::Blank),
      _ => {
        let n = i32::from_str(s).map_err(|_| "Could not parse to number")?;
        Ok(Entry::Calories(n))
      }
    }
  }
}

/// As we traverse the inputs, we will update this state.
#[derive(Debug)]
struct State {
  leader_id: i32,
  leader_total: i32,
  current_id: i32,
  current_total: i32,
}

impl State {

  pub fn new() -> State {
    State { leader_id: 0, leader_total: 0, current_id: 0, current_total: 0 }
  }

  /// Update the state according to an entry.
  pub fn step(&mut self, entry: &Entry) {
    match entry {
      Entry::Blank => {
        self.current_id = self.current_id + 1;
        self.current_total = 0;
      },
      Entry::Calories(c) => {
        self.current_total = self.current_total + c;
        self.update_leader();
      }
    }
  }

  /// Look at the current elf entry and, if that elf is
  /// the leader, update the leader stats.
  pub fn update_leader(&mut self) {
    if self.current_total > self.leader_total {
      self.leader_id = self.current_id;
      self.leader_total = self.current_total;
    }
  }

}

pub fn run<'a, I>(lines: I) -> Result<(i32, i32), &'static str>
  where I: Iterator< Item = String >
{
  let mut state = State::new();
  let mut state2 = State2::new();
  for line in lines {
    match Entry::from_str(&line) {
      Ok(entry) => {
        state.step(&entry);
        state2.step(&entry);
        println!("{:?}", state2);
      },
      Err(e) => {
        return Err(e);
      }
    }
  }
  Ok((state.leader_total, state2.top_3_sum()))
}

pub fn file_lines(path: &str) -> Box<dyn Iterator<Item = String>> {
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines().into_iter().map(|l| l.unwrap());
  Box::new(lines)
}

#[derive(Debug)]
struct State2 {
  previous_totals: Vec<i32>,
  current_total: i32,
}

impl State2 {
  pub fn new() -> State2 {
    State2 { previous_totals: vec![], current_total: 0 }
  }

  pub fn step(&mut self, entry: &Entry) {
    match entry {
      Entry::Blank => {
        self.previous_totals.push(self.current_total);
        self.current_total = 0;
      },
      Entry::Calories(n) => {
        self.current_total = self.current_total + n;
      }
    }
  }

  pub fn top_3_sum(self) -> i32 {
    let mut totals = self.previous_totals;
    totals.push(self.current_total);
    totals.sort();
    totals.reverse();
    totals.get(0).unwrap_or(&0) + totals.get(1).unwrap_or(&0) + totals.get(2).unwrap_or(&0)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn parse() {
    assert_eq!( Entry::from_str(""), Ok(Entry::Blank));
    assert_eq!( Entry::from_str("123"), Ok(Entry::Calories(123)));
    assert!( Entry::from_str("123 ").is_err());
  }

  #[test]
  fn trivial() {
    let input = vec![];
    assert_eq!(run(input.into_iter()), Ok((0,0)));
  }

  #[test]
  fn middle_parse_error() {
    let input : Vec<String> = vec!["1", "", "blah", "2"].into_iter().map(|s| s.to_string()).collect();
    assert!(run(input.into_iter()).is_err());
  }

  #[test]
  fn small() {
    let input : Vec<String> = vec!["1", "2", "3", "", "2"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(run(input.into_iter()), Ok((6,8)));
  }

  #[test]
  fn small_2() {
    let input : Vec<String> = vec!["1", "2", "3", "", "8"].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(run(input.into_iter()), Ok((8,14)));
  }

  #[test]
  fn small_3() {
    let input : Vec<String> = vec!["", "1", "2", "3", "", "8", ""].into_iter().map(|s| s.to_string()).collect();
    assert_eq!(run(input.into_iter()), Ok((8, 14)));
  }

  #[test]
  fn from_fake_file() {
    let input = file_lines(fake_input_file);
    assert_eq!(run(input.into_iter()), Ok((100,113)));
  }

  #[test]
  fn real() {
    let input = file_lines(input_file);
    assert_eq!(run(input.into_iter()), Ok((67027, 197291)));
  }

}
