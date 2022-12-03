use std::fs::File;
use std::io::prelude::*;

fn file_to_string(path: &str) -> String {
  let mut file = File::open("inputs/input").unwrap();
  let mut contents = String::new();
  file.read_to_string(&mut contents).unwrap();
  contents
}

fn main() -> std::io::Result<()> {
  print!("part 1: {}\n", part_1());
  print!("part 2: {}\n", part_2());
  Ok(())
}

fn part_1() -> i32 {
  let mut res = 0;
  let input = file_to_string("inputs/input");
  for e in input.split("\n\n") {
    let candidate = get_calories(e.to_string());
    if candidate > res { 
      res = candidate;
    }
  }
  
  res
}

fn part_2() -> i32 {
  let mut res = vec![];
  let input = file_to_string("inputs/input");
  for e in input.split("\n\n") {
    res.push(get_calories(e.to_string()));
  }

  res.sort_by(|a, b| b.cmp(a));
  res[0] + res[1] + res[2]

}

fn get_calories(input: String) -> i32 {
  let mut res = 0;
  for e in input.split("\n") {
    res += e.parse::<i32>().unwrap_or(0);
  }
  res
  
}

#[cfg(test)]
mod tests {
  use crate::get_calories;
  use std::fs::File;
  use std::io::prelude::*;
  
  #[test]
  fn test_get_calories_no_input() {
    assert_eq!(get_calories(String::from("")), 0)
  }
  
  #[test]
  fn test_get_calories_single_number() {
    assert_eq!(get_calories(String::from("1000")), 1000)
  }
  
  #[test]
  fn test_get_calories_multiple_number() {
    assert_eq!(get_calories(String::from(
      "1000\n1000")), 2000)
    }
    
    #[test]
    fn test_get_calories_demo() {
      let mut file = File::open("inputs/demo").unwrap();
      let mut contents = String::new();
      file.read_to_string(&mut contents).unwrap();
      let mut i = 0;
      let expected = vec![6000, 4000, 11000, 24000, 10000];
      for e in contents.split("\n\n") {
        assert_eq!(get_calories(e.to_string()),expected[i]);
        i += 1;
      }
      
    }
  }
  