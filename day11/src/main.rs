const INPUT: &str = include_str!("../inputs/input");
use std::collections::VecDeque;

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> u32 {
    let mut monkeys = vec![];
        for monkey in INPUT.split("\n\n") {
            monkeys.push(Monkey::from(monkey));
        }

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                while !monkeys[i].items.is_empty() {
                    let next = monkeys[i].inspect_and_throw(true);
                    monkeys[next.0].items.push_back(next.1);
                }
            }
        }
        monkeys.sort_by(|a, b| b.inspected.partial_cmp(&a.inspected).unwrap());
        monkeys[0].inspected * monkeys[1].inspected
}

fn part_2() -> u32 {
    let mut monkeys = vec![];
    for monkey in INPUT.split("\n\n") {
        monkeys.push(Monkey::from(monkey));
    }

    let mut destress = 1;
    for monkey in &monkeys {
        destress *= monkey.test;
    }

    for _round in 0..10000  {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let next = monkeys[i].inspect_and_throw(false);
                monkeys[next.0].items.push_back(next.1 % destress);
            }
        }
    }
    monkeys.sort_by(|a, b| b.inspected.partial_cmp(&a.inspected).unwrap());
    monkeys[0].inspected * monkeys[1].inspected
}

#[derive(Debug, PartialEq, Eq)]
enum Operation {
    Add(u32),
    Mult(u32),
    MultSelf,
}

struct Monkey {
    items: VecDeque<u32>,
    op: Operation,
    test: u32,
    throw_if_true: usize,
    throw_if_false: usize,
    inspected: u32,
}

impl Monkey {
    fn from(input: &str) -> Monkey {
        let mut lines = input.split('\n');
        lines.next(); // skip Monkey n: line
        let items_str = lines.next().unwrap()
            .strip_prefix("  Starting items: ").unwrap().split(", ");

        let mut items: VecDeque<u32> = VecDeque::new();
        for item in items_str{
            items.push_back(item.parse().unwrap());
        }
        let items_str = lines.next().unwrap()
            .strip_prefix("  Operation: new = old ").unwrap();
        let op = match items_str.chars().next().unwrap() {
            '*' => {
                if items_str == "* old" {
                    Operation::MultSelf
                } else {
                    Operation::Mult(items_str.strip_prefix("* ").unwrap().parse().unwrap())
                }                
            },
            '+' => {
                Operation::Add(items_str.strip_prefix("+ ").unwrap().parse().unwrap())
            }
            _ => unreachable!(),
        };

        let test:u32 = lines.next().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse().unwrap();
        
        let throw_if_true:usize = lines.next().unwrap()
            .strip_prefix("    If true: throw to monkey ").unwrap()
            .parse().unwrap();
        let throw_if_false:usize = lines.next().unwrap()
            .strip_prefix("    If false: throw to monkey ").unwrap()
            .parse().unwrap();

        Monkey {
            items,
            op,
            test,
            throw_if_true,
            throw_if_false,
            inspected: 0,
        }
    }

    fn inspect_and_throw(&mut self, divide_by_3: bool) -> (usize, u32) {
        // returns (target,worry_level)
        self.inspected += 1;
        let mut item = self.items.pop_front().unwrap();
        match self.op {
            Operation::Add(n) => item += n,
            Operation::Mult(n) => item *= n,
            Operation::MultSelf => item *= item,
        }
        if divide_by_3 {
            item /= 3;
        }        

        if item % self.test == 0 {
            (self.throw_if_true, item)
        } else {
            (self.throw_if_false, item)
        }

    }
}


#[cfg(test)]
mod test {
    use crate::*;
    #[test]
    fn test_monkey() {
        let test = Monkey::from(include_str!("../inputs/demo")
                                .split_once("\n\n").unwrap().0);
        assert_eq!(test.items[0],79);
        assert_eq!(test.items[1],98);
        assert_eq!(test.op,Operation::Mult(19));
        assert_eq!(test.test,23);
        assert_eq!(test.throw_if_true,2);
        assert_eq!(test.throw_if_false,3);
    }

    #[test]
    fn test_monkey_loop() {
        let mut test = Monkey::from(include_str!("../inputs/demo")
                                .split_once("\n\n").unwrap().0);
        if !test.items.is_empty() {
            assert_eq!(test.inspect_and_throw(true),(3,500));
            assert_eq!(test.inspect_and_throw(true),(3,620));
        }
    }

    #[test]
    fn test_part_1() {
        let mut monkeys = vec![];
        for monkey in include_str!("../inputs/demo").split("\n\n") {
            monkeys.push(Monkey::from(monkey));
        }

        for _round in 0..20 {
            for i in 0..monkeys.len() {
                while !monkeys[i].items.is_empty() {
                    let next = monkeys[i].inspect_and_throw(true);
                    monkeys[next.0].items.push_back(next.1);
                }
            }
        }

        assert_eq!(monkeys[0].inspected,101);
        assert_eq!(monkeys[1].inspected,95);
        assert_eq!(monkeys[2].inspected,7);
        assert_eq!(monkeys[3].inspected,105);

        monkeys.sort_by(|a, b| b.inspected.partial_cmp(&a.inspected).unwrap());
        assert_eq!(monkeys[0].inspected * monkeys[1].inspected,10605);
        
    }

    #[test]
    fn test_part_2() {
        let mut monkeys = vec![];
        for monkey in include_str!("../inputs/demo").split("\n\n") {
            monkeys.push(Monkey::from(monkey));
        }

        let mut destress = 1;
        for monkey in &monkeys {
            destress *= monkey.test;
        }

        for _round in 0..10000  {
            for i in 0..monkeys.len() {
                while !monkeys[i].items.is_empty() {
                    let next = monkeys[i].inspect_and_throw(false);
                    monkeys[next.0].items.push_back(next.1 % destress);
                }
            }
        }

        assert_eq!(monkeys[0].inspected,52166);
        assert_eq!(monkeys[1].inspected,47830);
        assert_eq!(monkeys[2].inspected,1938);
        assert_eq!(monkeys[3].inspected,52013);

        monkeys.sort_by(|a, b| b.inspected.partial_cmp(&a.inspected).unwrap());
        assert_eq!(monkeys[0].inspected * monkeys[1].inspected,2_713_310_158);
    }
}