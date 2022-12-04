use std::fs::File;
use std::io::prelude::*;

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

fn part_1() -> u32 {
  let mut res = 0;
  for line in file_to_string("inputs/input").split('\n') {
    let sections: Vec<&str> = line.split(',').collect();
    let s1 = Section::from(sections[0]);
    let s2 = Section::from(sections[1]);
    if s1.does_fully_overlap(s2) {
      res += 1;
    }
  }
  res
}

fn part_2() -> i32 {
  let mut res = 0;
  for line in file_to_string("inputs/input").split('\n') {
    let sections: Vec<&str> = line.split(',').collect();
    let s1 = Section::from(sections[0]);
    let s2 = Section::from(sections[1]);
    if s1.does_partial_overlap(s2) {
      res += 1;
    }
  }
  res
}

#[derive(Clone, Copy)]
struct Section {
  start: u32,
  end: u32,
}

impl Section {
  fn from(v: &str) -> Section {
    let v_vec: Vec<&str> = v.split('-').collect();
    let start: u32 = v_vec[0].parse().unwrap();
    let end: u32 = v_vec[1].parse().unwrap();
    Section {
      start,
      end
    }
  }
  
  fn does_fully_overlap(self, other: Section) -> bool {
    self.start >= other.start && self.end <= other.end ||
    self.start <= other.start && self.end >= other.end
  }

  fn does_partial_overlap(self, other: Section) -> bool {
    self.start > other.start && other.end >= self.start ||
    self.start < other.start &&  self.end >= other.start
  }
}


#[cfg(test)]
mod test {
  use crate::*;
  
  #[test]
  fn test_parse_section() {
    let expected = vec![2,4,6,8];
    for (i, section) in String::from("2-4,6-8").split(',').enumerate() {
      let result = Section::from(section);
      assert_eq!(result.start,expected[2* i]);
      assert_eq!(result.end,expected[2* i + 1]);
    }
  }
  
  #[test]
  fn test_full_overlap() {
    assert!( Section{start:2, end:8}.does_fully_overlap(Section{start:3, end:7}));
    assert!(!Section{start:2, end:8}.does_fully_overlap(Section{start:3, end:9}));
    assert!( Section{start:6, end:6}.does_fully_overlap(Section{start:4, end:6}));
    assert!(!Section{start:2, end:3}.does_fully_overlap(Section{start:4, end:5}));
    assert!(Section{start:2,end:99}.does_fully_overlap(Section{start: 2, end:98}));
  }
  
  #[test]
  fn test_part_1() {
    let mut res = 0;
    for line in file_to_string("inputs/demo").split('\n') {
      let sections: Vec<&str> = line.split(',').collect();
      if Section::from(sections[0]).does_fully_overlap(Section::from(sections[1])) {
        res += 1;
      }
    }
    assert_eq!(res,2);
  }
  #[test]
  fn test_no_partial_overlap() {
    assert!(!Section{start:2, end:4}.does_partial_overlap(Section{start:6, end:8}));
    assert!(!Section{start:2, end:3}.does_partial_overlap(Section{start:4, end:5}));

  }
  #[test]
  fn test_partial_overlap() {
    assert!(Section{start:5, end:7}.does_partial_overlap(Section{start:7, end:9}));
    assert!(Section{start:2, end:8}.does_partial_overlap(Section{start:3, end:7}));
    assert!(Section{start:6, end:6}.does_partial_overlap(Section{start: 4, end:6}));
    assert!(Section{start:2, end:6}.does_partial_overlap(Section{start: 4, end:8}));
  }
}