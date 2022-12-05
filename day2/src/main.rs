fn main() {
    println!("Hello, world!");

    let rounds = include_str!("./input")
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    println!("Part 1: {:?}", part1(&rounds));
    println!("Part 2: {:?}", part2(&rounds));
}

fn part1(rounds: &Vec<Vec<&str>>) -> usize {
    rounds.iter().map(|round| handle_round_pt1(round)).sum()
}

fn handle_round_pt1(round: &[&str]) -> usize {
    enum RoundResult {
        Win,
        Draw,
        Loss,
    }

    let result = match round[1] {
        "X" => match round[0] {
            "A" => RoundResult::Draw,
            "B" => RoundResult::Loss,
            "C" => RoundResult::Win,
            _ => panic!("Invalid round"),
        },
        "Y" => match round[0] {
            "A" => RoundResult::Win,
            "B" => RoundResult::Draw,
            "C" => RoundResult::Loss,
            _ => panic!("Invalid round"),
        },
        "Z" => match round[0] {
            "A" => RoundResult::Loss,
            "B" => RoundResult::Win,
            "C" => RoundResult::Draw,
            _ => panic!("Invalid round"),
        },
        _ => panic!("Invalid round 4"),
    };

    // get the score based on what we played
    let score = match round[1] {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    };

    match result {
        RoundResult::Win => score + 6,
        RoundResult::Draw => score + 3,
        RoundResult::Loss => score,
    }
}

fn part2(rounds: &Vec<Vec<&str>>) -> usize {
    rounds.iter().map(|round| handle_round_pt2(round)).sum()
}

fn handle_round_pt2(round: &[&str]) -> usize {
    // now, X = lose, Y = draw, Z = win
    let score = match round[0] {
        "A" => match round[1] {
            "X" => 3,
            "Y" => 1 + 3,
            "Z" => 2 + 6,
            _ => panic!("Invalid round"),
        },
        "B" => match round[1] {
            "X" => 1,
            "Y" => 2 + 3,
            "Z" => 3 + 6,
            _ => panic!("Invalid round"),
        },
        "C" => match round[1] {
            "X" => 2,
            "Y" => 3 + 3,
            "Z" => 1 + 6,
            _ => panic!("Invalid round"),
        },
        _ => panic!("Invalid round"),
    };

    score
}
