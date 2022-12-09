use std::collections::HashMap;

const INPUT: &str = include_str!("../inputs/input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> usize {
    let mut pos = Position::init(2);
    for line in INPUT.split('\n') {
        let dir = line.chars().next().unwrap();
        let n:usize = line.split(' ').nth(1).unwrap().parse().unwrap();
        pos.head_move(dir, n);
    }
    pos.tail_visited.keys().count()
}

fn part_2() -> usize {
    let mut pos = Position::init(10);
    for line in INPUT.split('\n') {
        //println!("{}",line);
        let dir = line.chars().next().unwrap();
        let n:usize = line.split(' ').nth(1).unwrap().parse().unwrap();
        pos.head_move(dir, n);
    }
    pos.tail_visited.keys().count()
}

struct Position {
    //head: Vec<i32>,
    knots: Vec<Vec<i32>>,
    tail_visited: HashMap<(i32, i32), bool>,
}

impl Position {
    fn init(knots: usize) -> Position {
        let mut tail_visited = HashMap::new();
        tail_visited.insert((0,0), true);
        Position {knots: vec![vec![0;2];knots], tail_visited }
    }

    fn head_move(&mut self, dir: char, n: usize) {
        let dir_v = match dir {
            'L' => [-1, 0],
            'R' => [1, 0],
            'U' => [0, 1],
            'D' => [0, -1],
            _   => unreachable!(),
        };
        for _step in 0..n {
            self.knots[0][0] += dir_v[0];
            self.knots[0][1] += dir_v[1];
            self.tail_move(1);
        }
    }

    fn tail_move(&mut self, idx: usize) {
        let prev = self.knots[idx - 1].clone();
        let tail = &mut self.knots[idx];
        let mut diff = vec![
            prev[0] - tail[0],
            prev[1] - tail[1]
        ];
        if diff[0].abs() < 2 && diff[1].abs() < 2 {
            return;
        }
        for e in diff.iter_mut() { // diff -> direction vector
            if *e == 2 {
                *e = 1;
            }
            if *e == -2 {
                *e = -1;
            }
        }
        tail[0] += diff[0];
        tail[1] += diff[1];
        let max_tails = self.knots.len() - 1;
        if idx == max_tails {
            self.tail_visited.insert((self.knots[max_tails][0],self.knots[max_tails][1]),true);
        } else {
            self.tail_move(idx + 1); // recursion!
        }
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_tail_move() {
        let mut test = Position::init(2);
        test.knots[0] = vec![2,0];
        
        assert_eq!(test.knots[1],[0,0]);
        assert!(test.tail_visited.contains_key(&(0,0)));
        assert!(!test.tail_visited.contains_key(&(1,0)));

        test.tail_move(1);
        assert_eq!(test.knots[1],[1,0]);
        assert!(test.tail_visited.contains_key(&(1,0)));
    }

    #[test]
    fn test_part_1() {
        let input = include_str!("../inputs/demo");
        let mut pos = Position::init(2);
        for line in input.split('\n') {
            let dir = line.chars().next().unwrap();
            let n:usize = line.split(' ').nth(1).unwrap().parse().unwrap();
            pos.head_move(dir, n);
        }
        assert_eq!(pos.tail_visited.keys().count(),13);
    }
}