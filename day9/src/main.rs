use std::fmt::{Display, Error, Formatter};
use std::str::FromStr;

fn main() {
    let input: Vec<Instruction> = include_str!("input")
        .lines()
        .map(|l| l.parse().unwrap())
        .collect::<Vec<_>>();

    part1(&input);
    part2(&input);
}

fn part1(input: &Vec<Instruction>) {
    let mut headPosition = Vector { x: 0, y: 0 };
    let mut tailPosition = Vector { x: 0, y: 0 };

    let mut tail_positions: Vec<Vector> = vec![tailPosition];

    for inst in input {
        for _ in 0..inst.distance {
            headPosition = headPosition.step_direction(&inst.direction);
            // now figure out if the tail needs to move too
            while tailPosition.distance_from(&headPosition) > 1 {
                // we want to move in the direction of the head
                let direction = tailPosition.direction_to(&headPosition);
                tailPosition = tailPosition.add(&direction);
                tail_positions.push(tailPosition);
            }
        }
    }
    // only select unique positions
    tail_positions.sort();
    tail_positions.dedup();

    println!("tail positions: {:?}", tail_positions.len());
}

fn part2(input: &Vec<Instruction>) {
    let mut knotPositions: Vec<Vector> = vec![];
    for _ in 0..10 {
        knotPositions.push(Vector { x: 0, y: 0 });
    }

    let mut tail_positions: Vec<Vector> = vec![knotPositions[knotPositions.len() - 1]];

    // apply the move to the first knot
    for inst in input {
        for _ in 0..inst.distance {
            knotPositions[0] = knotPositions[0].step_direction(&inst.direction);
            // now figure out if the tail needs to move too
            for i in 1..knotPositions.len() {
                while knotPositions[i].distance_from(&knotPositions[i - 1]) > 1 {
                    // we want to move in the direction of the head
                    let direction = knotPositions[i].direction_to(&knotPositions[i - 1]);
                    knotPositions[i] = knotPositions[i].add(&direction);
                }
            }
            // get the position of the last knot and store it
            tail_positions.push(knotPositions[knotPositions.len() - 1]);
        }
    }
    // only select unique positions
    tail_positions.sort();
    tail_positions.dedup();

    println!("tail positions: {:?}", tail_positions.len());
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
struct Vector {
    x: i32,
    y: i32,
}

impl Vector {
    fn step_direction(&self, direction: &Direction) -> Vector {
        match direction {
            Direction::Up => Vector {
                x: self.x,
                y: self.y + 1,
            },
            Direction::Down => Vector {
                x: self.x,
                y: self.y - 1,
            },
            Direction::Left => Vector {
                x: self.x - 1,
                y: self.y,
            },
            Direction::Right => Vector {
                x: self.x + 1,
                y: self.y,
            },
        }
    }

    fn add(&self, other: &Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }

    fn direction_to(&self, other: &Vector) -> Vector {
        let x = match (self.x.cmp(&other.x)) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        let y = match (self.y.cmp(&other.y)) {
            std::cmp::Ordering::Less => 1,
            std::cmp::Ordering::Equal => 0,
            std::cmp::Ordering::Greater => -1,
        };
        Vector { x, y }
    }

    fn distance_from(&self, other: &Vector) -> i32 {
        // d=√((x2 – x1)² + (y2 – y1)²)
        let x = (other.x - self.x).pow(2);
        let y = (other.y - self.y).pow(2);
        ((x + y) as f32).sqrt() as i32
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: i32,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (direction, distance) = s.split_at(1);
        let direction = match direction {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            _ => return Err(()),
        };
        let distance = distance.trim().parse().map_err(|_| ())?;
        Ok(Instruction {
            direction,
            distance,
        })
    }
}
