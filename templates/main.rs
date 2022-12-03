use std::fs::File;
use std::io::prelude::*;

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

fn part_1() -> i32 {
    0
}

fn part_2() -> i32 {
    0
}



#[cfg(test)]
mod test {
    use crate::inputs;

    #[test]
    fn dummy_test() {
        assert_eq!(inputs::test(), "");
    }
}