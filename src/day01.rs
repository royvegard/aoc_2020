use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;

#[aoc_generator(day1)]
fn parse_input(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1, loops)]
fn solve_part1_loops(input: &[u32]) -> u32 {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() - 1 {
            if input[i] + input[j] == 2020 && i != j {
                return input[i] * input[j];
            }
        }
    }
    0
}

#[aoc(day1, part2, loops)]
fn solve_part2_loops(input: &[u32]) -> u32 {
    for i in 0..input.len() - 1 {
        for j in i + 1..input.len() - 1 {
            for k in j + 1..input.len() - 1 {
                if input[i] + input[j] + input[k] == 2020 {
                    return input[i] * input[j] * input[k];
                }
            }
        }
    }
    0
}

#[aoc(day1, part1, seen)]
fn solve_part1_seen(input: &[u32]) -> u32 {
    use std::collections::HashSet;

    let mut seen: HashSet<u32> = HashSet::new();

    for i in input {
        seen.insert(*i);
        if seen.contains(&(2020 - i)) {
            return i * (2020 - i);
        }
    }

    0
}

#[aoc(day1, part1, tuple_combinations)]
fn solve_part1_tuple_combinations(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_combinations::<(&u32, &u32)>()
        .find(|&(v1, v2)| v1 + v2 == 2020)
        .map(|(v1, v2)| v1 * v2)
        .unwrap()
}

#[aoc(day1, part2)]
fn solve_part2(input: &[u32]) -> u32 {
    input
        .iter()
        .tuple_combinations::<(&u32, &u32, &u32)>()
        .find(|(&v1, &v2, &v3)| v1 + v2 + v3 == 2020)
        .map(|(v1, v2, v3)| v1 * v2 * v3)
        .unwrap()
}
