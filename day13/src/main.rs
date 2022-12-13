use itertools::Itertools;
use serde::Deserialize;

use std::cmp::Ordering;

fn main() {
    let input = include_str!("input");

    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut lists = input.lines().filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<ListItem>(l).unwrap());

    let mut sum = 0;
    for i in 1.. {
        match is_next_pair_in_order(&mut lists) {
            Some(true) => sum += i,
            Some(false) => (),
            None => break,
        }
    }
    sum
}

fn is_next_pair_in_order(iter : &mut impl Iterator<Item = ListItem>) -> Option<bool> {
    let Some(left) = iter.next() else { return None; };
    let right = iter.next().unwrap();

    Some(left < right)
}

fn part2(input: &str) -> usize {
    let dividers : (ListItem, ListItem) = (serde_json::from_str::<ListItem>("[[2]]").unwrap(), serde_json::from_str::<ListItem>("[[6]]").unwrap());

    let mut lists = input.lines().filter(|l| !l.is_empty())
        .map(|l| serde_json::from_str::<ListItem>(l).unwrap()).collect::<Vec<_>>();

    lists.sort();

    lists.iter().enumerate()
        .filter(|(_, l)| **l == dividers.0 || **l == dividers.1)
        .map(|(i, _)| i +1).product()

}


#[derive(Debug, Clone, Deserialize)]
#[serde(untagged)]
enum ListItem {
    Single(u8),
    Nested(Vec<ListItem>)
}

impl PartialEq<Self> for ListItem {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}
impl Eq for ListItem {}

impl PartialOrd<Self> for ListItem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for ListItem {
    fn cmp(&self, other: &Self) -> Ordering {
        use ListItem::*;
        match (self, other) {
            (Single(a), Single(b)) => a.cmp(b),
            (Single(a), Nested(b)) => [Single(*a)][..].cmp(b),
            (Nested(a), Single(b)) => a.as_slice().cmp(&[Single(*b)]),
            (Nested(a), Nested(b)) => a.cmp(b)
        }
    }
}