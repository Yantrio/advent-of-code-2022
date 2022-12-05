use regex::Regex;
use std::str::FromStr;

fn main() {
    let mut split = include_str!("input").split("\n\n");

    let grid: Grid = split.next().unwrap().parse().unwrap();
    let moves: Vec<Move> = split
        .next()
        .unwrap()
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();

    part1(grid.clone(), &moves);
    part2(grid.clone(), &moves);
}

fn part1(grid: Grid, moves: &Vec<Move>) {
    let mut pt1_grid = grid.clone();
    for m in moves {
        pt1_grid.apply_move_pt1(&m);
    }
    println!("result {}", pt1_grid.get_top_of_stacks())
}

fn part2(grid: Grid, moves: &Vec<Move>) {
    let mut pt2_grid = grid.clone();
    for m in moves {
        pt2_grid.apply_move_pt2(&m);
    }
    println!("result {}", pt2_grid.get_top_of_stacks())
}

#[derive(Debug, Clone)]
struct Grid(Vec<Vec<char>>);

impl Grid {
    fn apply_move_pt1(&mut self, m: &Move) {
        for _ in 0..m.amount {
            // pop from the from row, push to the to row
            let c = self.0[m.from - 1].pop().unwrap();
            self.0[m.to - 1].push(c);
        }
    }

    fn apply_move_pt2(&mut self, m: &Move) {
        let mut to_move: Vec<char> = vec![];
        for _ in 0..m.amount {
            // pop from the from row, push to the to row
            let c = self.0[m.from - 1].pop().unwrap();
            to_move.push(c);
        }
        // this is lazy, but reverse it and push
        to_move.reverse();
        for c in to_move {
            self.0[m.to - 1].push(c);
        }
    }

    fn get_top_of_stacks(&self) -> String {
        let mut result = String::new();
        for row in &self.0 {
            result.push(row[row.len() - 1]);
        }
        result.to_owned()
    }
}

impl FromStr for Grid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Grid(Vec::new());
        let mut lines = s.lines().rev();
        let num_vecs = lines.next().unwrap().len() / 4;
        for _ in 0..=num_vecs {
            grid.0.push(Vec::new());
        }
        for line in lines {
            let iter = line.chars().skip(1);

            // im sure theres a better way to do this, this will do for now
            let mut i = 0;
            for c in iter.step_by(4) {
                if !c.is_whitespace() {
                    // push it
                    grid.0[i].push(c);
                }
                i += 1
            }
        }
        Ok(grid)
    }
}

#[derive(Debug)]
struct Move {
    pub from: usize,
    pub to: usize,
    pub amount: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
        let caps = re.captures(s).unwrap();

        let amount = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
        let from = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
        let to = caps.get(3).unwrap().as_str().parse::<usize>().unwrap();
        Ok(Move { from, to, amount })
    }
}
