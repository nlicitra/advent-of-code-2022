use core::num;
use std::fs;

type Stacks = Vec<Vec<char>>;

fn generate_empty_state(num_stacks: usize) -> Stacks {
    let mut stacks: Stacks = Vec::default();
    for n in 0..num_stacks {
        stacks.push(Vec::default());
    }
    stacks
}

fn parse_state_input(input: &str) -> Stacks {
    let mut lines: Vec<&str> = input.split("\n").collect();
    let stack_numbers = lines.pop().unwrap();
    let num_stacks = stack_numbers.chars().filter(|c| *c != ' ').last().unwrap();
    let num_stacks: usize = String::from(num_stacks).parse().unwrap();
    lines.reverse();
    let mut stacks = generate_empty_state(num_stacks);
    for l in lines {
        let line: Vec<char> = l.chars().collect();
        let parts: Vec<char> = line.chunks(4).map(|c| c[1]).collect();
        for (i, c) in parts.iter().enumerate() {
            if *c != ' ' {
                stacks[i].push(*c);
            }
        }
    }
    stacks
}

fn parse_instruction(input: &str) -> (usize, usize, usize) {
    let parts: Vec<&str> = input.split_whitespace().collect();
    (
        parts[1].parse().unwrap(),
        parts[3].parse().unwrap(),
        parts[5].parse().unwrap(),
    )
}

pub fn part1() -> String {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    let (init_state, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_state_input(init_state);

    for instruction in instructions.split("\n") {
        let (amount, from, to) = parse_instruction(instruction);
        for _ in 0..amount {
            if let Some(thing) = stacks[from - 1].pop() {
                stacks[to - 1].push(thing);
            }
        }
    }
    let mut result = String::default();
    for s in stacks {
        if let Some(thing) = s.last() {
            result.push(*thing);
        }
    }
    result
}

pub fn part2() -> String {
    let input = fs::read_to_string("./inputs/day5.txt").unwrap();
    let (init_state, instructions) = input.split_once("\n\n").unwrap();
    let mut stacks = parse_state_input(init_state);

    for instruction in instructions.split("\n") {
        let (amount, from, to) = parse_instruction(instruction);
        let mut batch: Vec<char> = Vec::new();
        for _ in 0..amount {
            if let Some(thing) = stacks[from - 1].pop() {
                batch.push(thing);
            }
        }
        batch.reverse();
        stacks[to - 1].append(&mut batch);
    }
    let mut result = String::default();
    for s in stacks {
        if let Some(thing) = s.last() {
            result.push(*thing);
        }
    }
    result
}
