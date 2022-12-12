use pathfinding::{prelude::bfs, prelude::Matrix};

fn main() {
    let mut matrix = Matrix::from_rows(include_str!("input").lines().map(str::bytes)).unwrap();

    let start = matrix.indices().find(|&i| matrix[i] == b'S').unwrap();
    let destination = matrix.indices().find(|&i| matrix[i] == b'E').unwrap();

    matrix[start] = b'a';
    matrix[destination] = b'z';

    println!("part1 {}", part1(&matrix, &start, &destination));
    println!("part2 {}", part2(&matrix, &destination));
}

fn part1(matrix: &Matrix<u8>, start: &(usize, usize), destination: &(usize, usize)) -> usize {
    bfs(
        start,
        |&p| {
            matrix
                .neighbours(p, false) // dont allow diagonals
                .filter(move |&q| matrix[q] <= matrix[p] + 1) // only allow steps of 1
        },
        |&p| p == *destination, // stop when we reach the destination
    )
    .unwrap()
    .len()
        - 1
}

fn part2(matrix: &Matrix<u8>, destination: &(usize, usize)) -> usize {
    bfs(
        destination, // work back from the destination
        |&p| {
            matrix
                .neighbours(p, false) // no diagonals
                .filter(move |&q| matrix[p] <= matrix[q] + 1) // only allow steps of 1 (backwards)
        },
        |&p| matrix[p] == b'a', // go to any sqare at height 'a'
    )
    .unwrap()
    .len()
        - 1
}
