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
    match input {
        'a'..='z' => <char as Into<u32>>::into(input) - 96,
        'A'..='Z' => 27,
        _ => panic!("char_to_score({}) expected [a-zA-Z]",input),
    }
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
    use crate::*;

    #[test]
    fn score_char() {
        //let test:char = 'a';
        assert_eq!(char_to_score('a'), 1);
        assert_eq!(char_to_score('b'), 2);
        assert_eq!(char_to_score('A'), 27);
    }
}