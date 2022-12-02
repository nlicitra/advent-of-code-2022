use std::{cmp::Ordering, fs};

#[derive(Debug, Clone, Copy)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

enum BattleResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl BattleResult {
    fn derive_other_hand(&self, hand: &Hand) -> Hand {
        match self {
            Self::Draw => *hand,
            Self::Loss => match hand {
                Hand::Rock => Hand::Scissors,
                Hand::Scissors => Hand::Paper,
                Hand::Paper => Hand::Rock,
            },
            Self::Win => match hand {
                Hand::Rock => Hand::Paper,
                Hand::Scissors => Hand::Rock,
                Hand::Paper => Hand::Scissors,
            },
        }
    }
}

impl Hand {
    fn value(&self) -> usize {
        match *self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }

    fn battle(&self, other: &Self) -> BattleResult {
        match self {
            Self::Rock => match other {
                Self::Rock => BattleResult::Draw,
                Self::Paper => BattleResult::Loss,
                Self::Scissors => BattleResult::Win,
            },
            Self::Paper => match other {
                Self::Rock => BattleResult::Win,
                Self::Paper => BattleResult::Draw,
                Self::Scissors => BattleResult::Loss,
            },
            Self::Scissors => match other {
                Self::Rock => BattleResult::Loss,
                Self::Paper => BattleResult::Win,
                Self::Scissors => BattleResult::Draw,
            },
        }
    }
}

impl From<usize> for Hand {
    fn from(val: usize) -> Self {
        match val {
            1 => Self::Rock,
            2 => Self::Paper,
            3 => Self::Scissors,
            _ => panic!("unexpected value, {val}"),
        }
    }
}

impl From<&str> for Hand {
    fn from(val: &str) -> Self {
        match val {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => panic!("unexpected value, {val}"),
        }
    }
}

impl From<&str> for BattleResult {
    fn from(val: &str) -> Self {
        match val {
            "X" => Self::Loss,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => panic!("unexpected value, {val}"),
        }
    }
}

pub fn part1() -> usize {
    fs::read_to_string("./inputs/day2.txt")
        .unwrap()
        .split("\n")
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let theirs: Hand = left.into();
            let mine: Hand = right.into();
            let result = mine.value() + mine.battle(&theirs) as usize;
            result
        })
        .sum()
}

pub fn part2() -> usize {
    fs::read_to_string("./inputs/day2.txt")
        .unwrap()
        .split("\n")
        .map(|l| {
            let (left, right) = l.split_once(' ').unwrap();
            let theirs: Hand = left.into();
            let expected_result: BattleResult = right.into();
            let mine = expected_result.derive_other_hand(&theirs);
            let result = mine.value() + mine.battle(&theirs) as usize;
            result
        })
        .sum()
}
