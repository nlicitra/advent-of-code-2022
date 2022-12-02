use std::fs;

pub fn part1() -> usize {
    return fs::read_to_string("./inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .sum()
        })
        .max()
        .unwrap();
}

pub fn part2() -> usize {
    let mut calories = fs::read_to_string("./inputs/day1.txt")
        .unwrap()
        .split("\n\n")
        .map(|s| {
            s.split_whitespace()
                .map(|n| n.parse::<usize>().unwrap())
                .sum()
        })
        .collect::<Vec<usize>>();
    calories.sort_by(|a, b| b.cmp(a));
    return calories.iter().take(3).sum();
}
