use itertools::Itertools;

fn main() {
    let input = include_str!("input");
    let total_calories_per_person = input
        .split("\n\n")
        .map(|x| {
            x.lines()
                .map(|x| x.parse::<usize>().unwrap())
                .sum::<usize>()
        })
        .collect::<Vec<_>>();

    let most_calories = part1(total_calories_per_person.iter());
    println!("Part 1: {}", most_calories);

    let sum_of_top_three_calories = part2(total_calories_per_person.iter());
    println!("Part 2: {}", sum_of_top_three_calories);
}

fn part1<'a, I>(total_calories_per_person: I) -> &'a usize
where
    I: Iterator<Item = &'a usize>,
{
    // find the max
    total_calories_per_person.max().unwrap()
}

fn part2<'a, I>(total_calories_per_person: I) -> usize
where
    I: Iterator<Item = &'a usize>,
{
    // sort the vector and return the sum of the top 3
    total_calories_per_person.sorted().rev().take(3).sum()
}
