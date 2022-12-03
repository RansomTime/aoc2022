use std::fs::File;
use std::io::prelude::*;

#[allow(dead_code)]
fn file_to_string(path: &str) -> String {
  let mut file = File::open(path).unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn char_to_score(input: char) -> u32 {
  // convert using ordinal value, panic if not given [A-Za-z]
  match input {
    'a'..='z' => <char as Into<u32>>::into(input) - 96,
    'A'..='Z' => <char as Into<u32>>::into(input) - 38,
    _ => panic!("char_to_score({}) expected [a-zA-Z]",input),
  }
}

fn main() {
  println!("part 1: {}", part_1());
  println!("part 2: {}", part_2());
}

fn part_1() -> u32 {
  let mut res = 0;
  for line in file_to_string("inputs/input").split("\n") {
    match Rucksack::from(line).get_shared() {
      Some (shared_item) => res += char_to_score(shared_item),
      None => panic!("no shared item in rucksack: {}",line),
    }
  }
  res
}

fn part_2() -> u32 {
  let mut res = 0;
  let lines = file_to_string("inputs/input");
  let mut lines_itr = lines.split("\n").peekable();
  while lines_itr.peek().is_some() { // we know that input's length is 300
    match Rucksack::get_group_badge(
      Rucksack::from(lines_itr.next().unwrap()),
      Rucksack::from(lines_itr.next().unwrap()),
      Rucksack::from(lines_itr.next().unwrap()),
    ) {
      Some (shared_item) => res += char_to_score(shared_item),
      None => panic!("no shared item in rucksacks"),
    }
  }
  res
}

struct Rucksack {
  contents: String,
}

impl Rucksack {
  fn from(input: &str) -> Rucksack {
    Rucksack {
      contents: String::from(input),
    }
  }

  fn get_left(&self) -> String {
    let midpoint = self.contents.len()/2;
    String::from(self.contents.get(0..midpoint).unwrap())
  }

  fn get_right(&self) -> String {
    let midpoint = self.contents.len()/2;
    String::from(self.contents.get(midpoint..).unwrap())
  }

  fn get_shared(&self) -> Option<char> {
    for e in self.get_left().chars() {
      if self.get_right().contains(e) {
        return Some(e)
      }
    }
    None
  }
  
  fn get_group_badge(first: Rucksack, second: Rucksack, third: Rucksack) -> Option <char> {
    for e in first.contents.chars() {
      if second.contents.contains(e) && third.contents.contains(e) {
        return Some(e)
      }
    }
    None
  }
}



#[cfg(test)]
mod test {
  use crate::*;
  
  #[test]
  fn score_char() {
    //let test:char = 'a';
    assert_eq!(char_to_score('a'), 1);
    assert_eq!(char_to_score('b'), 2);
    assert_eq!(char_to_score('A'), 27);
    assert_eq!(char_to_score('Z'), 52);
    assert_eq!(char_to_score('P'), 42);
    assert_eq!(char_to_score('t'), 20);
  }
  
  #[test]
  fn split_rucksack() {
    let test = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
    
    assert_eq!(test.get_left(),  String::from("vJrwpWtwJgWr"));
    assert_eq!(test.get_right(), String::from("hcsFMMfFFhFp"));
    
    let test2 = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(test2.get_left(),  String::from("jqHRNqRjqzjGDLGL"));
    assert_eq!(test2.get_right(), String::from("rsFMfFZSrLrFZsSL"));
  }
  
  #[test]
  fn diff_compartments() {
    let test = Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp");
    let test2 = Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
    assert_eq!(test.get_shared(),Some('p'));
    assert_eq!(test2.get_shared(),Some('L'));
  }
  
  #[test]
  fn test_part_1() {
    let mut res = 0;
    for line in file_to_string("inputs/demo").split("\n") {
      match Rucksack::from(line).get_shared() {
        Some (shared_item) => res += char_to_score(shared_item),
        None => panic!("no shared item in rucksack: {}",line),
      }
    }
    assert_eq!(res,157);
  }
  
  #[test]
  fn test_group_badge() {
    assert_eq!(Rucksack::get_group_badge(
      Rucksack::from("vJrwpWtwJgWrhcsFMMfFFhFp"),
      Rucksack::from("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"),
      Rucksack::from("PmmdzqPrVvPwwTWBwg")),
      Some('r')
    );

    assert_eq!(Rucksack::get_group_badge(
      Rucksack::from("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"),
      Rucksack::from("ttgJtRGJQctTZtZT"),
      Rucksack::from("CrZsJsPPZsGzwwsLwLmpwMDw")),
      Some('Z')
    );
  }

  #[test]
  fn test_part_2() {
    let mut res = 0;
    let lines = file_to_string("inputs/demo");
    let mut lines_itr = lines.split("\n").peekable();
    while lines_itr.peek().is_some() {
      match Rucksack::get_group_badge(
        Rucksack::from(lines_itr.next().unwrap()),
        Rucksack::from(lines_itr.next().unwrap()),
        Rucksack::from(lines_itr.next().unwrap()),
      ) {
        Some (shared_item) => res += char_to_score(shared_item),
        None => panic!("no shared item in rucksacks"),
      }
    }
    assert_eq!(res,70);
  }
  
}