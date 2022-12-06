use std::collections::HashSet;
use std::fs::File;
use std::char;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::hash_map::RandomState;

const input_file : &'static str = "inputs/puzzle_4.txt";

#[derive(Clone,Debug, PartialEq)]
struct Assignment {
    first: i32,
    last: i32,
}

#[derive(Clone,Debug, PartialEq)]
struct Pair {
   elf_1: Assignment,
   elf_2: Assignment,
}

impl FromStr for Pair {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        println!("{}",s);
        let (assignment_1, assignment_2) = s.rsplit_once(',').ok_or("Missing comma in first split")?;
        let (first, last) = assignment_1.rsplit_once('-').ok_or("Nope")?;
        let elf_1 = Assignment {
            first: i32::from_str(first).map_err(|_| "Nope")?,
            last: i32::from_str(last).map_err(|_| "Nope")?,
        };
        let (first, last) = assignment_2.rsplit_once('-').ok_or("Nope")?;
        let elf_2 = Assignment {
            first: i32::from_str(first).map_err(|_| "Nope")?,
            last: i32::from_str(last).map_err(|_| "Nope")?,
        };
        Ok(Pair {elf_1, elf_2})

    }
}

impl Assignment {
    pub fn contains(&self, other: &Assignment) -> bool {
        self.first <= other.first && self.last >= other.last
    }
}

impl Pair {
    pub fn has_full_containment(&self) -> bool {
        self.elf_1.contains(&self.elf_2) || self.elf_2.contains(&self.elf_1)
    }

    pub fn has_some_overlap(&self) -> bool {

        !(self.elf_1.last < self.elf_2.first || self.elf_2.last < self.elf_1.first)

    }

}


pub fn file_lines(path: &str) -> Box<dyn Iterator<Item = String>> {
  let file = File::open(path).unwrap();
  let lines = io::BufReader::new(file).lines().into_iter().map(|l| l.unwrap());
  Box::new(lines)
}

/*
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn d3_parse() {
        assert_eq!( Pair::from_str("1-2,3-4").unwrap(), Pair {elf_1: Assignment{first: 1, last: 2}, elf_2: Assignment {first: 3, last: 4}} );
    }

    #[test]
    fn d3_containment() {
        assert!( Pair::from_str("2-4,3-4").unwrap().has_full_containment() );
        assert!( !Pair::from_str("2-4,3-5").unwrap().has_full_containment() );
        assert!( Pair::from_str("3-4,2-4").unwrap().has_full_containment() );
        assert!( !Pair::from_str("3-5,2-4").unwrap().has_full_containment() );
    }

    #[test]
    fn d3_has_some_overlap() {
        assert!( Pair::from_str("2-4,3-4").unwrap().has_some_overlap() );
        assert!( Pair::from_str("3-4,2-4").unwrap().has_some_overlap() );
        assert!( Pair::from_str("1-2,2-3").unwrap().has_some_overlap() );
        assert!( Pair::from_str("2-3,1-2").unwrap().has_some_overlap() );
        assert!( Pair::from_str("1-4,2-3").unwrap().has_some_overlap() );
        assert!( Pair::from_str("2-3,1-4").unwrap().has_some_overlap() );

        assert!( !Pair::from_str("5-6,1-2").unwrap().has_some_overlap() );
        assert!( !Pair::from_str("3-4,1-2").unwrap().has_some_overlap() );
        assert!( !Pair::from_str("1-2,3-4").unwrap().has_some_overlap() );
    }

    #[test]
    fn d3_example_1() {
        let input = vec![
            "2-4,6-8",
            "2-3,4-5",
            "5-7,7-9",
            "2-8,3-7",
            "6-6,4-6",
            "2-6,4-8",
        ].into_iter().map(|s| s.to_string()).collect::<Vec<String>>();
        assert_eq!(input.into_iter().map(|p| Pair::from_str(&p).expect("parse failed")).filter(|p| p.has_full_containment()).count(), 2);
    }

    #[test]
    fn d3_real_1() {
        let input = file_lines(input_file);
        assert_eq!(input.into_iter().map(|p| Pair::from_str(&p).expect("parse failed")).filter(|p| p.has_full_containment()).count(), 424);
    }

    #[test]
    fn d3_real_2() {
        let input = file_lines(input_file);
        assert_eq!(input.into_iter().map(|p| Pair::from_str(&p).expect("parse failed")).filter(|p| p.has_some_overlap()).count(), 804);
    }
}
*/
