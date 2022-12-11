use std::collections::vec_deque::VecDeque;
use std::str::FromStr;

use itertools::Itertools;

fn main() {
    let monkeys = include_str!("input")
        .split("\n\n")
        .map(|i| i.parse::<Monkey>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1: {}", part1(&monkeys));
    println!("Part 2: {}", part2(&monkeys));
}

fn part1(monkeys: &Vec<Monkey>) -> usize {
    let mut cloned = monkeys.clone();
    for _ in 0..20 {
        Monkey::round(&mut cloned, None)
    }
    cloned.sort_unstable_by_key(|m| std::cmp::Reverse(m.examined));
    cloned[0].examined * cloned[1].examined
}

fn part2(monkeys: &Vec<Monkey>) -> usize {
    let mut cloned = monkeys.clone();
    let divisor = cloned.iter().map(|m| m.test).product();
    for _ in 0..10000 {
        Monkey::round(&mut cloned, Some(divisor))
    }
    cloned.sort_unstable_by_key(|m| std::cmp::Reverse(m.examined));
    cloned[0].examined * cloned[1].examined
}

#[derive(Debug, Clone, Default)]
struct Monkey {
    monkey_id: u64,
    items: Vec<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,

    examined: usize,
}

impl Monkey {
    fn act(monkeys: &mut [Monkey], index: usize, divisor: Option<u64>) {
        let mut monkey = Monkey::default();
        // i admit i got this from a guy on reddit, swapping out the memory was my only choice i could think of
        std::mem::swap(&mut monkey, &mut monkeys[index]);
        monkey.examined += monkey.items.len();
        for item in monkey.items.drain(..) {
            let worry_level = match divisor {
                Some(d) => monkey.operation.apply(item) % d,
                None => monkey.operation.apply(item) / 3,
            };

            monkeys[match worry_level % monkey.test == 0 {
                true => monkey.if_true,
                false => monkey.if_false,
            }]
            .items
            .push(worry_level);
        }
        std::mem::swap(&mut monkey, &mut monkeys[index]);
    }

    fn round(monkeys: &mut [Monkey], divisor: Option<u64>) {
        (0..monkeys.len()).for_each(|index| Self::act(monkeys, index, divisor));
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let monkey_id = lines
            .next()
            .unwrap()
            .split_whitespace()
            .nth(1)
            .unwrap()
            .replace(":", "")
            .parse()
            .unwrap();
        let starting_items = lines
            .next()
            .unwrap()
            .split(":")
            .nth(1)
            .unwrap()
            .split(",")
            .map(|x| x.trim().parse().unwrap())
            .collect();
        let operation = lines.next().unwrap().parse().unwrap();
        let divisible_test: u64 = lines
            .next()
            .unwrap()
            .split("by ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        let if_true = lines
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
        let if_false = lines
            .next()
            .unwrap()
            .split("to monkey ")
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();

        Ok(Monkey {
            monkey_id,
            items: starting_items,
            operation,

            test: divisible_test,
            if_true,
            if_false,
            examined: 0,
        })
    }
}

#[derive(Debug, Clone, Default)]
enum Operation {
    Plus(u64),
    Multiply(u64),
    #[default]
    Square,
}

impl Operation {
    fn apply(&self, item: u64) -> u64 {
        match self {
            Operation::Plus(x) => item + x,
            Operation::Multiply(x) => item * x,
            Operation::Square => item * item,
        }
    }
}

impl FromStr for Operation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let operation = s.split("new = old ").collect::<Vec<&str>>()[1];

        let parts = operation.split(" ").collect::<Vec<&str>>();
        if parts[1] == "old" {
            return Ok(Operation::Square);
        } else {
            match parts[0] {
                "+" => Ok(Operation::Plus(parts[1].parse::<u64>().unwrap())),
                "*" => Ok(Operation::Multiply(parts[1].parse::<u64>().unwrap())),
                _ => Err("Unknown operation".to_string()),
            }
        }
    }
}
