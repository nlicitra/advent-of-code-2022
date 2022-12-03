use std::collections::HashSet;
use std::fs;

fn score_char(c: &char) -> usize {
    let adjustment = match c.is_lowercase() {
        true => 96,
        false => 38,
    };
    *c as usize - adjustment
}

pub fn part1() -> usize {
    fs::read_to_string("./inputs/day3.txt")
        .unwrap()
        .split("\n")
        .map(|l| {
            let mid = l.len() / 2 as usize;
            let (left, right) = l.split_at(mid);
            let left: HashSet<char> = left.chars().collect();
            let right: HashSet<char> = right.chars().collect();
            let inter = left.intersection(&right);
            let num = match inter.into_iter().nth(0) {
                Some(c) => score_char(c),
                None => 0,
            };
            num
        })
        .sum()
}

pub fn part2() -> usize {
    fs::read_to_string("./inputs/day3.txt")
        .unwrap()
        .split("\n")
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|batch| {
            if batch.len() < 3 {
                return 0;
            }
            let set_a: HashSet<char> = batch[0].chars().collect();
            let set_b: HashSet<char> = batch[1].chars().collect();
            let set_c: HashSet<char> = batch[2].chars().collect();
            let inter: HashSet<char> = set_a.intersection(&set_b).map(|c| *c).collect();
            let inter: Vec<&char> = inter.intersection(&set_c).collect();
            match inter.get(0) {
                Some(c) => score_char(c),
                None => 0,
            }
        })
        .sum()
}
