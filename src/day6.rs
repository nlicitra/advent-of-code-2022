use std::collections::HashSet;
use std::fs;

pub fn part1() -> usize {
    let chars: Vec<char> = fs::read_to_string("./inputs/day6.txt")
        .unwrap()
        .chars()
        .collect();
    let mut result = 0;
    for (index, window) in chars.as_slice().windows(4).enumerate() {
        let set: HashSet<&char> = window.into_iter().collect();
        if window.len() == set.len() {
            result = index + 4;
            break;
        }
    }
    result
}

pub fn part2() -> usize {
    let chars: Vec<char> = fs::read_to_string("./inputs/day6.txt")
        .unwrap()
        .chars()
        .collect();
    let mut result = 0;
    for (index, window) in chars.as_slice().windows(14).enumerate() {
        let set: HashSet<&char> = window.into_iter().collect();
        if window.len() == set.len() {
            result = index + 14;
            break;
        }
    }
    result
}
