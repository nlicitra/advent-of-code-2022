use std::fs;

fn parse_range(range: &str) -> (usize, usize) {
    let (left, right) = range.split_once('-').unwrap();
    (left.parse().unwrap(), right.parse().unwrap())
}

pub fn part1() -> usize {
    fs::read_to_string("./inputs/day4.txt")
        .unwrap()
        .split("\n")
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a_from, a_to) = parse_range(a);
            let (b_from, b_to) = parse_range(b);
            let a_range = a_to - a_from;
            let b_range = b_to - b_from;
            if (a_range >= b_range && b_from >= a_from && b_to <= a_to) {
                return 1;
            } else if (a_range < b_range && a_from >= b_from && b_to >= a_to) {
                return 1;
            }
            0
        })
        .sum()
}

pub fn part2() -> usize {
    fs::read_to_string("./inputs/day4.txt")
        .unwrap()
        .split("\n")
        .map(|l| {
            let (a, b) = l.split_once(',').unwrap();
            let (a_from, a_to) = parse_range(a);
            let (b_from, b_to) = parse_range(b);
            let a_range = a_from..a_to + 1;
            let b_range = b_from..b_to + 1;
            if a_range.contains(&b_from)
                || a_range.contains(&b_to)
                || b_range.contains(&a_from)
                || b_range.contains(&a_to)
            {
                return 1;
            }
            0
        })
        .sum()
}
