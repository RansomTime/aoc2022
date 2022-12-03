use std::fs::File;
use std::io::prelude::*;

fn file_to_string(path: &str) -> String {
  let mut file = File::open(path).unwrap();
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
    for line in file_to_string("inputs/input").split("\n") {
      res += get_score(line);
    }
    res
}

fn part_2() -> i32 {
  let mut res = 0;
    for line in file_to_string("inputs/input").split("\n") {
      res += get_score_part_2(line);
    }
    res

}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum WLD {
  Win,
  Lose,
  Draw
}

impl WLD {  
  fn from_string(input: &str)  -> WLD {
    match input {
      "X" => WLD::Lose,
      "Y" => WLD::Draw,
      "Z" => WLD::Win,
      _ => panic!("received: {}",input),
    }
  }

  fn to_score(&self) -> i32 {
    match self {
      WLD::Win  => 6,
      WLD::Draw => 3,
      WLD::Lose => 0,
    }
  }

  fn determine_throw(&self, opponent: RPS)-> RPS {
    match opponent {
      RPS::Rock => {
        match self {
          WLD::Win  => RPS::Paper,
          WLD::Lose => RPS::Scissors,
          WLD::Draw => RPS::Rock,
        }
      },
      RPS::Paper => {
        match self {
          WLD::Win  => RPS::Scissors,
          WLD::Lose => RPS::Rock,
          WLD::Draw => RPS::Paper,
        }
      },
      RPS::Scissors => {
        match self {
          WLD::Win  => RPS::Rock,
          WLD::Lose => RPS::Paper,
          WLD::Draw => RPS::Scissors,
        }
      },
    }
  }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
enum RPS {
  Rock,
  Paper,
  Scissors,
}

impl RPS {  
  fn from_string(input: &str)  -> RPS {
    match input {
      "A" | "X" => RPS::Rock,
      "B" | "Y" => RPS::Paper,
      "C" | "Z" => RPS::Scissors,
      _ => panic!("received: {}",input),
    }
  }

  fn to_score(&self) -> i32 {
    match self {
      RPS::Rock => 1,
      RPS::Paper => 2,
      RPS::Scissors => 3,
    }
  }

  fn determine_winner(player1: RPS, player2: RPS) -> i32 {
    // returns 0 if no winner, else the player number (1 or 2)
    match player1 {
      RPS::Rock => {
        match player2 {
          RPS::Rock => 0,
          RPS::Paper => 2,
          RPS::Scissors => 1,
        }
      },
      RPS::Paper => {
        match player2 {
          RPS::Rock => 1,
          RPS::Paper => 0,
          RPS::Scissors => 2,
        }
      },
      RPS::Scissors => {
        match player2 {
          RPS::Rock => 2,
          RPS::Paper => 1,
          RPS::Scissors => 0,
        }
      },
    }
  }
}

fn get_score_part_2(input: &str) -> i32 {
  let throws: Vec<&str> = input.split(" ").collect();
  let them = RPS::from_string(throws[0]);
  let me = WLD::from_string(throws[1]);
  let my_throw = me.determine_throw(them);

  my_throw.to_score() + me.to_score()
}


fn get_score(input: &str) -> i32 {
  let throws: Vec<&str> = input.split(" ").collect();
  let me = RPS::from_string(throws[1]);
  let them = RPS::from_string(throws[0]);

  match RPS::determine_winner(me, them) {
    1 => 6 + me.to_score(),
    2 => me.to_score(),
    0 => 3 + me.to_score(),
    _ => panic!("determine winnner fail")
  }

}

#[cfg(test)]
mod tests {
  use crate::get_score;
  use crate::get_score_part_2;
  //use std::fs::File;
  //use std::io::prelude::*;
  use crate::file_to_string;
  
  #[test]
  fn test_get_score() {
    assert_eq!(get_score("A Y"), 8);
    assert_eq!(get_score("B X"), 1);
    assert_eq!(get_score("C Z"), 6);
  }

  #[test]
  fn test_demo() {
    let mut res = 0;
    for line in file_to_string("inputs/demo").split("\n") {
      res += get_score(line);
    }
    assert_eq!(res, 15);
  }

  #[test]
  fn test_demo_part_2() {
    let mut res = 0;
    for line in file_to_string("inputs/demo").split("\n") {
      res += get_score_part_2(line);
    }
    assert_eq!(res, 12);
  }
  
}
  