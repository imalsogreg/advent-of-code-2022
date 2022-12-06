use std::collections::HashSet;
use std::fs::read_to_string;
use std::char;
use std::io::{self, BufRead};
use std::str::FromStr;
use std::collections::hash_map::RandomState;

const input_file_stacks : &'static str = "inputs/puzzle_5_stacks.txt";
const input_file_moves : &'static str = "inputs/puzzle_5_moves.txt";


#[derive(Debug)]
struct State {
    stacks: Vec<Vec<char>>
}

impl State {
    pub fn new() -> State {
        let mut stacks = Vec::new();
        for _ in 1..10 {
            let mut stack = Vec::new();
            // Reserve 100 slots: the maximum height a stack could have
            // given our starting state.
            stack.reserve(100);
            stacks.push(stack);
        }
        State { stacks }
    }

    pub fn apply_1(&mut self, instruction: &Instruction) {
        for _ in 0..instruction.n_boxes {
            println!("{instruction:?}");
            println!("{self:?}");
            let cargo = self.stacks[instruction.source - 1].pop().unwrap();
            self.stacks[instruction.destination - 1].push(cargo);
        }
    }

    pub fn apply_2(&mut self, instruction: &Instruction) {
        println!("{self:?}");
        println!("{instruction:?}");
        let source_i = instruction.source - 1;
        let destination_i = instruction.destination - 1;
        let source_size = self.stacks[source_i].len();
        let cargo_size = instruction.n_boxes as usize;
        println!("source_size: {source_size}, cargo_size: {cargo_size}");
        let mut cargo = self.stacks[source_i].split_off(source_size - cargo_size);
        println!("cargo: {cargo:?}");
        self.stacks[destination_i].append(&mut cargo);
    }
}

impl FromStr for State {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(State {
            stacks:
              s
                .lines()
                .into_iter()
                .map(|l| l.chars().into_iter().collect())
                .collect()
        })
    }
}

impl FromStr for Instruction {
    type Err = &'static str;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words : Vec<&str> = s.split(' ').collect();
        Ok( Instruction {
            n_boxes: u8::from_str(words[1]).map_err(|_| "Nope")?,
            source: usize::from_str(words[3]).map_err(|_| "Nope")?,
            destination: usize::from_str(words[5]).map_err(|_| "Nope")?,
        } )
    }
}

#[derive(Clone,Debug)]
struct Instruction {
    n_boxes: u8,
    source: usize,
    destination: usize,
}

#[derive(Debug)]
struct Problem {
    initial_state: State,
    instructions: Vec<Instruction>,
}



impl Problem {
    pub fn load(state_path: &str, moves_path: &str) -> Problem {
        let initial_state =
            State::from_str( &read_to_string(state_path).unwrap() ).unwrap();
        let instructions =
            read_to_string(moves_path)
            .unwrap()
            .lines()
            .into_iter()
            .map(|l| Instruction::from_str(l))
            .collect::<Result<Vec<Instruction>, &'static str>>().unwrap();
        Problem {initial_state, instructions}
    }

    pub fn solve_1(mut self) -> Vec<char> {
        for i in self.instructions {
            self.initial_state.apply_1(&i);
        }
        self.initial_state.stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
    }

    pub fn solve_2(mut self) -> Vec<char> {
        for i in self.instructions {
            self.initial_state.apply_2(&i);
        }
        self.initial_state.stacks.into_iter().map(|mut s| s.pop().unwrap()).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_state() {
        let s = State::new();
        assert_eq!(s.stacks.len(), 9);
    }

    #[test]
    fn push_pop_check() {
        let mut t = vec![1,2];
        t.pop();
        assert_eq!(t, vec![1]);
        t.push(3);
        assert_eq!(t, vec![1,3]);
    }

    #[test]
    fn example_1() {
        let problem = Problem::load("inputs/puzzle_5_stacks_example.txt", "inputs/puzzle_5_moves_example.txt");
        assert_eq!(problem.solve_1(), vec!['C', 'M', 'Z']);
    }

    #[test]
    fn part_1() {
        let problem = Problem::load("inputs/puzzle_5_stacks.txt", "inputs/puzzle_5_moves.txt");
        assert_eq!(problem.solve_1(), vec!['Q', 'G', 'T', 'H', 'F', 'Z', 'B', 'H', 'V']);
    }

    #[test]
    fn example_2() {
        let problem = Problem::load("inputs/puzzle_5_stacks_example.txt", "inputs/puzzle_5_moves_example.txt");
        assert_eq!(problem.solve_2(), vec!['M', 'C', 'D']);
    }

    #[test]
    fn part_2() {
        let problem = Problem::load("inputs/puzzle_5_stacks.txt", "inputs/puzzle_5_moves.txt");
        assert_eq!(problem.solve_2(), vec!['M', 'G', 'D', 'M', 'P', 'S', 'Z', 'T', 'M']);
    }
}
