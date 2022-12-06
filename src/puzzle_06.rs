use std::fs;
use std::collections::HashSet;
use std::collections::hash_map::RandomState;



const input_file : &'static str = "inputs/puzzle_6.txt";

pub struct CircBuf4 {
    pub elems: Vec<char>,
    pub insert_cursor: usize,
}

impl CircBuf4 {
    pub fn new() -> Self {
        CircBuf4 {
            elems: vec![],
            insert_cursor: 3,
        }
    }

    pub fn push(&mut self, c: char) {
        if self.elems.len() == 4 {
            self.insert_cursor = (self.insert_cursor + 1) % 4;
            self.elems[self.insert_cursor] = c;
        } else {
            self.elems.push(c);
        }
    }

    pub fn all_unique(&self) -> bool {
        let hs : HashSet<char, RandomState> = self.elems.iter().cloned().collect();
        hs.len() == 4
    }
}

pub struct CircBuf14 {
    pub elems: Vec<char>,
    pub insert_cursor: usize,
}

impl CircBuf14 {
    pub fn new() -> Self {
        CircBuf14 {
            elems: vec![],
            insert_cursor: 13,
        }
    }

    pub fn push(&mut self, c: char) {
        if self.elems.len() == 14 {
            self.insert_cursor = (self.insert_cursor + 1) % 14;
            self.elems[self.insert_cursor] = c;
        } else {
            self.elems.push(c);
        }
    }

    pub fn all_unique(&self) -> bool {
        let hs : HashSet<char, RandomState> = self.elems.iter().cloned().collect();
        hs.len() == 14
    }
}

pub fn find_start_sequence(s: &str) -> u32 {
    let mut ind = 0;
    let mut cb = CircBuf4::new();
    for c in s.chars().into_iter() {
        cb.push(c);
        ind += 1;
        if cb.all_unique() {
            break;
        }
    }
    ind
}

pub fn find_start_sequence_2(s: &str) -> u32 {
    let mut ind = 0;
    let mut cb = CircBuf14::new();
    for c in s.chars().into_iter() {
        cb.push(c);
        ind += 1;
        if cb.all_unique() {
            break;
        }
    }
    ind
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn circ_buf() {
        let mut cb = CircBuf4::new();
        cb.push('a');
        assert_eq!(cb.elems, vec!['a']);
        cb.push('a');
        assert_eq!(cb.elems, vec!['a','a']);
        cb.push('c');
        assert_eq!(cb.elems, vec!['a','a','c']);
        cb.push('d');
        assert_eq!(cb.elems, vec!['a','a','c','d']);
        assert!(!cb.all_unique());
        cb.push('e');
        assert_eq!(cb.elems, vec!['e','a','c','d']);
        cb.push('f');
        cb.push('g');
        cb.push('h');
        assert_eq!(cb.elems, vec!['e','f','g','h']);
        assert!(cb.all_unique());
    }

    #[test]
    fn example_1() {
        assert_eq!(find_start_sequence("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn part_1() {
        use std::fs::read_to_string;
        let input = read_to_string(input_file).unwrap();
        assert_eq!(find_start_sequence(&input), 1480);
    }

    #[test]
    fn examples_2() {
        assert_eq!(find_start_sequence_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(find_start_sequence_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(find_start_sequence_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(find_start_sequence_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(find_start_sequence_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn part_2() {
        use std::fs::read_to_string;
        let input = read_to_string(input_file).unwrap();
        assert_eq!(find_start_sequence_2(&input), 2746);
    }

}
