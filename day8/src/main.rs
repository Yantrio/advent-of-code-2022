fn main() {
    let input = include_str!("input");
    let trees = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<Vec<u32>>>();

    println!("{}", part_1(&trees));
    println!("{}", part_2(&trees));
}

fn part_1(trees: &Vec<Vec<u32>>) -> usize {
    // the count starts with the perimeter of the grid
    let mut count = (trees.len() - 1) * 4;
    for x in 1..trees.len() - 1 {
        for y in 1..trees[x].len() - 1 {
            if tree_is_visible(&trees, x, y) {
                count += 1;
            }
        }
    }
    count
}

fn part_2(trees: &Vec<Vec<u32>>) -> usize {
    // find the max tree score
    let mut max_score = 0;
    for x in 1..trees.len() - 1 {
        for y in 1..trees[x].len() - 1 {
            let score = get_tree_score(&trees, x, y);
            if score > max_score {
                max_score = score;
            }
        }
    }
    max_score
}

fn get_tree_score(trees: &[Vec<u32>], x: usize, y: usize) -> usize {
    let trees_in_all_directions = trees_in_all_directions(trees, x, y);
    let tree_height = trees[y][x];

    trees_in_all_directions
        .iter()
        .map(|direction| {
            direction
                .iter()
                .position(|t| t >= &tree_height)
                .map(|t| t + 1)
                .unwrap_or_else(|| direction.len())
        })
        .product()
}

fn tree_is_visible(trees: &[Vec<u32>], x: usize, y: usize) -> bool {
    let trees_in_all_directions = trees_in_all_directions(trees, x, y);
    let tree_height = trees[y][x];

    // a tree is visible if there is no tree in any direction that is taller than it
    // if any of the directions have a tree that is taller than the current tree
    // then the current tree is not visible

    trees_in_all_directions
        .iter()
        .any(|direction| direction.iter().all(|t| t < &tree_height))
}

fn trees_in_all_directions(trees: &[Vec<u32>], x: usize, y: usize) -> [Vec<u32>; 4] {
    // for each direction (up, down, left, right)
    // collect the vector of trees in that direction, not including the current tree

    // get the current row and column of the grid position so we can split them
    let row = trees[y].clone();
    let column = trees.iter().map(|r| r[x]).collect::<Vec<u32>>();

    let (above, below) = column.split_at(y);
    let (left, right) = row.split_at(x);

    [
        above.iter().copied().rev().collect(),
        below[1..].to_vec(),
        left.iter().copied().rev().collect(),
        right[1..].to_vec(),
    ]
}
