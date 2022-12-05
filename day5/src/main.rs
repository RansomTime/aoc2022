use std::fs::File;
use std::io::prelude::*;
use std::collections::VecDeque;
use regex::Regex;
#[macro_use]
extern crate lazy_static;


fn parse_input_stacks() -> VecDeque<VecDeque<char>>  {
  let input = file_to_string("inputs/input_stacks");
  let mut res = VecDeque::from([VecDeque::from([]), VecDeque::from([]),VecDeque::from([]),VecDeque::from([]),VecDeque::from([]),VecDeque::from([]),VecDeque::from([]),VecDeque::from([]),VecDeque::from([])]);
  
  for line in input.split('\n').rev() {
    let mut i = 0;
    let mut chars = line.chars();
    while i <= 8 {
      if chars.next().unwrap() == '[' {
        let next = chars.next().unwrap();
        res[i].push_back(next);
        for _ in 0..2 { chars.next(); } // skip to next
      } else {
        for _ in 0..3 { chars.next(); } 
      }
      i += 1;
    }
  }
  res
}

#[allow(dead_code)]
fn file_to_string(path: &str) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn main() {
  println!("part 1: {}", part_1());
  println!("part 2: {}", part_2());
}

fn part_1() -> String {
  let mut stacks = parse_input_stacks();
  for line in file_to_string("inputs/input_inst").split('\n') {
    let ins = Instruction::from_str(line);
    for _ in 0..ins.num {
      let to_push = stacks[ins.src].pop_back().unwrap();
      stacks[ins.dst].push_back(to_push);
    }
  }
  stacks.into_iter().map(|mut stack| stack.pop_back().unwrap()).collect()
  
}

fn part_2() -> String {
  let mut stacks = parse_input_stacks();
  for line in file_to_string("inputs/input_inst").split('\n') {
    let ins = Instruction::from_str(line);
    let mut to_push = VecDeque::from([]);
    for _ in 0..ins.num {
      to_push.push_back(stacks[ins.src].pop_back().unwrap());
    }
    for _ in 0..ins.num {
      stacks[ins.dst].push_back(to_push.pop_back().unwrap());
    }
  }
  stacks.into_iter().map(|mut stack| stack.pop_back().unwrap()).collect()
}

#[derive(Debug, PartialEq)]
struct Instruction {
  num: u8,
  src: usize,
  dst: usize,
}

impl Instruction {
  fn from_str(v: &str) -> Instruction {
    lazy_static! {
      static ref INST: Regex = Regex::new(r"move (\d{1,}) from (\d{1,}) to (\d{1,})").unwrap();
    }
    let caps = INST.captures(v).unwrap();
    
    // caps.get(0) is the whole match so
    let num:u8    = caps.get(1).unwrap().as_str().parse().unwrap(); 
    let src:usize = caps.get(2).unwrap().as_str().parse().unwrap();
    let dst:usize = caps.get(3).unwrap().as_str().parse().unwrap();
    Instruction {
      num,
      src: src - 1, // decrement here for ez indexing
      dst: dst - 1,
    }
  }
}

#[cfg(test)]
mod test {
  use crate::*;
  
  #[test]
  fn dummy_test() {
    assert_eq!("", "");
  }
  
  #[test]
  fn test_parse() {
    let res = Instruction::from_str("move 17 from 1 to 9");
    assert_eq!(Instruction{
      num: 17,
      src: 0,
      dst: 8,
    }, res);
  }
  
  fn get_demo() -> VecDeque<VecDeque<char>>  {
    VecDeque::from([VecDeque::from(['Z','N']),
    VecDeque::from(['M','C','D']),
    VecDeque::from(['P']),])
  }
  
  #[test]
  fn test_demo() {
    let mut stacks = get_demo();
    for line in file_to_string("inputs/demo").split('\n') {
      let ins = Instruction::from_str(line);
      for _ in 0..ins.num {
        let to_push = stacks[ins.src].pop_back().unwrap();
        stacks[ins.dst].push_back(to_push);
      }
    }
    let res:String = stacks.into_iter().map(|mut stack| stack.pop_back().unwrap()).collect();
    assert_eq!(res, String::from("CMZ"));
  }
  
  #[test]
  fn test_parse_input() {
    let stacks = parse_input_stacks();
    let res:String = stacks.into_iter().map(|mut stack| stack.pop_back().unwrap()).collect();
    assert_eq!(res, String::from("SRSJBTQRT"));
  }
  
  #[test]
  fn test_demo_part_2() {
    let mut stacks = get_demo();
    for line in file_to_string("inputs/demo").split('\n') {
      let ins = Instruction::from_str(line);
      let mut to_push = VecDeque::from([]);
      for _ in 0..ins.num {
        to_push.push_back(stacks[ins.src].pop_back().unwrap());
      }
      for _ in 0..ins.num {
        stacks[ins.dst].push_back(to_push.pop_back().unwrap());
      }
    }
    let res:String = stacks.into_iter().map(|mut stack| stack.pop_back().unwrap()).collect();
    assert_eq!(res, String::from("MCD"));
  }
}