fn main() {
    let rucksacks = include_str!("input").lines().collect::<Vec<_>>();

    println!("Part 1: {}", part1(&rucksacks));
    println!("Part 2: {}", part2(&rucksacks));
}

fn part1(rucksacks: &Vec<&str>) -> usize {
    rucksacks
        .into_iter()
        .filter_map(|&rucksack| {
            let (l, r) = rucksack.split_at(rucksack.len() / 2);
            char_in_all_string(vec![l, r])
        })
        .map(|f| get_prio(f as u8))
        .sum()
}

fn part2(rucksacks: &Vec<&str>) -> usize {
    // grab every three lines
    rucksacks
        .chunks(3)
        .map(|r| r.to_vec())
        .filter_map(char_in_all_string)
        .map(|c| get_prio(c as u8))
        .sum()
}

fn char_in_all_string(strings: Vec<&str>) -> Option<char> {
    let mut chars = strings[0].chars();
    let mut char = chars.next();
    while char.is_some() {
        let c = char.unwrap();
        if strings.iter().all(|s| s.contains(c)) {
            return Some(c);
        }
        char = chars.next();
    }
    return None;
}

fn get_prio(item: u8) -> usize {
    if item >= b'a' {
        return (item - b'a') as usize + 1;
    }
    return (item - b'A') as usize + 27;
}
