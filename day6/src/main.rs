use std::collections::VecDeque;
use itertools::Itertools;


const INPUT: &str = include_str!("../inputs/input");


fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    get_first_packet(INPUT, 4).unwrap()
}

fn part_2() -> usize {
    get_first_packet(INPUT, 14).unwrap()
}

fn get_first_packet(input: &str, len: usize) -> Option<usize> {
    let mut seen = VecDeque::from([]);
    for (i, e) in input.chars().enumerate() {
        seen.push_back(e);
        
        // expensive check but not worth optimising
        if seen.clone().into_iter().unique().count() == len { 
            return Some(i+1);
        }
        while seen.len() >= len { //shrink to 14
            seen.pop_front();
        }
    }
    None
}



#[cfg(test)]
mod test {
    use crate::*;

    #[test]
    fn test_demo_1() {
        assert_eq!(get_first_packet("bvwbjplbgvbhsrlpgdmjqwftvncz",4).unwrap(),5);
        
    }
    #[test]
    fn test_demo_2() {
        assert_eq!(get_first_packet("nppdvjthqldpwncqszvftbrmjlhg",4).unwrap(),6);
    }
    #[test]
    fn test_demo_3() {
        assert_eq!(get_first_packet("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",4).unwrap(),10);
    }
    #[test]
    fn test_demo_4() {
        assert_eq!(get_first_packet("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",4).unwrap(),11);
    }
    #[test]
    fn test_part_2() {
        assert_eq!(get_first_packet("mjqjpqmgbljsphdztnvjfqwrcgsmlb",14).unwrap(),19);
    }
}