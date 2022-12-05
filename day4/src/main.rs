use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("input")
        .lines()
        .map(|l| l.split(',').take(2).collect::<Vec<_>>())
        .map(|pair| (to_range_inclusive(pair[0]), to_range_inclusive(pair[1])))
        .collect::<Vec<_>>();

    println!(
        "Part 1: {:?}",
        input
            .iter()
            .filter(|pair| contains(&pair.0, &pair.1))
            .count()
    );
    println!(
        "Part 2: {:?}",
        input
            .iter()
            .filter(|pair| overlaps(&pair.0, &pair.1))
            .count()
    );
}

fn to_range_inclusive(input: &str) -> RangeInclusive<usize> {
    let mut split = input.split('-');
    let start = split.next().unwrap().parse::<usize>().unwrap();
    let end = split.next().unwrap().parse::<usize>().unwrap();
    start..=end
}

fn contains(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(b.start()) && a.contains(b.end()) || b.contains(a.start()) && b.contains(a.end())
}

fn overlaps(a: &RangeInclusive<usize>, b: &RangeInclusive<usize>) -> bool {
    a.contains(b.start()) || a.contains(b.end()) || b.contains(a.start()) || b.contains(a.end())
}
