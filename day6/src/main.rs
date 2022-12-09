use std::collections::HashSet;

fn main() {
    let input = include_str!("input");

    println!(
        "Part 1: {:?}",
        get_start_of_packet_sequence_index(input, 4).unwrap()
    );

    println!(
        "Part 2: {:?}",
        get_start_of_packet_sequence_index(input, 14).unwrap()
    );
}

fn get_start_of_packet_sequence_index(s: &str, window_size: usize) -> Result<usize, ()> {
    let chars = s.chars().collect::<Vec<char>>();
    for (i, window) in chars.windows(window_size).enumerate() {
        let unique_chars = window.iter().map(|&c| c).collect::<HashSet<char>>().len();
        if unique_chars == window_size {
            return Ok(i + window_size);
        }
    }
    Err(())
}
