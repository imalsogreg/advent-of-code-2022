use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::hash_map::RandomState;

const input_file : &'static str = "inputs/puzzle_3.txt";

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
struct Item { item_code: char }

#[derive(PartialEq, Eq, Debug)]
struct Rucksack {
    left_items: Vec<Item>,
    right_items: Vec<Item>,
}

impl FromStr for Rucksack {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| s.is_ascii()) {
            let mut left_items : Vec<Item> = s.chars().into_iter().map(|item_code| Item{ item_code }).collect();
            let right_items = left_items.split_off(s.chars().into_iter().collect::<Vec<_>>().len() / 2);
            Ok(Rucksack {
                left_items, right_items
            })
        } else {
            Err("Encountered non-ascii chars")
        }
    }
}

impl Item {
    pub fn priority(&self) -> i32 {
        let c = self.item_code;
        if c.is_ascii() {
            if self.item_code.is_ascii_lowercase() {
                (c as i32 - 'a' as i32 + 1)
            } else if self.item_code.is_ascii_uppercase() {
                (c as i32 - 'A' as i32 + 1 + 26)
            } else {
                0
            }
        } else {
            0
        }
    }
}

impl Rucksack {
    pub fn misplaced_item(&self) -> Result<Item, &'static str> {
        let left_set : HashSet<&Item,RandomState> = self.left_items.iter().collect();
        let right_set : HashSet<&Item,_> = self.right_items.iter().collect();
        let misplaced_items : Vec<&Item> = left_set.intersection(&right_set).into_iter().cloned().collect();
        match misplaced_items.len() {
            1 => Ok(misplaced_items[0].clone()),
            0 => Err("No misplaced items"),
            _ => Err("Multiple misplaced items"),
        }
    }


}


pub fn file_lines(path: &str) -> Box<dyn Iterator<Item = String>> {
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines().into_iter().map(|l| l.unwrap());
  Box::new(lines)
}

pub fn run<'a, I>(lines: I) -> Result<i32, &'static str>
  where I: Iterator< Item = String >
{
    let mut total_value = 0;
    for line in lines {
      let priority =  Rucksack::from_str(&line)?.misplaced_item()?.priority();
      total_value += priority;
    }
    Ok(total_value)
}

pub fn run_2<'a, I>(mut lines: I) -> Result<i32, &'static str>
  where I: Iterator< Item = String >
{
    let mut total_group_priority = 0;
    loop {
        match lines.next() {
            None => { break; },
            Some(elf_1) => {
                let elf_2 = lines.next().expect("elf should exist");
                let elf_3 = lines.next().expect("elf should exist");
                let items_1 = elf_1.chars().into_iter().collect::<HashSet::<char, RandomState>>();
                let items_2 = elf_2.chars().into_iter().collect::<HashSet::<char, RandomState>>();
                let items_3 = elf_3.chars().into_iter().collect::<HashSet::<char, RandomState>>();
                let commons = items_1.intersection( &items_2 ).into_iter().cloned().collect::<HashSet::<char, RandomState>>().intersection( &items_3 ).into_iter().cloned().collect::<Vec<char>>();
                assert!(commons.len() == 1);
                total_group_priority += Item{item_code: commons[0]}.priority()
            }
        }
    }
    Ok(total_group_priority)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_priorities() {
        assert_eq!( Item{item_code: 'a'}.priority(), 1);
        assert_eq!( Item{item_code: 'z'}.priority(), 26);
        assert_eq!( Item{item_code: 'A'}.priority(), 27);
        assert_eq!( Item{item_code: 'Z'}.priority(), 52);
    }

    #[test]
    fn d3_parse() {
        assert_eq!( Rucksack::from_str("abcdEF").unwrap(), Rucksack {
            left_items: "abc".chars().into_iter().map(|item_code| Item{item_code}).collect(),
            right_items: "dEF".chars().into_iter().map(|item_code| Item{item_code}).collect(),
        } )
    }

    #[test]
    fn d3_oddballs() {
        assert_eq!( Rucksack::from_str("aabcda").unwrap().misplaced_item().unwrap(), Item{item_code: 'a'} );
        assert_eq!( Rucksack::from_str("vJrwpWtwJgWrhcsFMMfFFhFp").unwrap().misplaced_item().unwrap(), Item{item_code: 'p'} );
        assert_eq!( Rucksack::from_str("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL").unwrap().misplaced_item().unwrap(), Item{item_code: 'L'} );
        assert_eq!( Rucksack::from_str("PmmdzqPrVvPwwTWBwg").unwrap().misplaced_item().unwrap(), Item{item_code: 'P'} );
        assert_eq!( Rucksack::from_str("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn").unwrap().misplaced_item().unwrap(), Item{item_code: 'v'} );
        assert_eq!( Rucksack::from_str("ttgJtRGJQctTZtZT").unwrap().misplaced_item().unwrap(), Item{item_code: 't'} );
        assert_eq!( Rucksack::from_str("CrZsJsPPZsGzwwsLwLmpwMDw").unwrap().misplaced_item().unwrap(), Item{item_code: 's'} );
    }

    #[test]
    fn d3_example() {
        let input : Vec<String> = vec![
            "vJrwpWtwJgWrhcsFMMfFFhFp" ,
            "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL",
            "PmmdzqPrVvPwwTWBwg",
            "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn",
            "ttgJtRGJQctTZtZT",
            "CrZsJsPPZsGzwwsLwLmpwMDw",
        ].into_iter().map(|s| s.to_string()).collect();
        assert_eq!(run(input.into_iter()).unwrap(), 157);
    }

    #[test]
    fn d3_real_1() {
        let input = file_lines(input_file);
        assert_eq!(run(input.into_iter()).unwrap(), 7701);
    }

    #[test]
    fn d3_real_2() {
        let input = file_lines(input_file);
        assert_eq!(run_2(input.into_iter()).unwrap(), 2644);
    }
}
