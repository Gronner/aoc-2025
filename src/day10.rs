use std::{fmt::Display, str::FromStr};
use anyhow::{Result, Context};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Machine {
    target_lights: usize,
    button: Vec<usize>,
    joltages: Vec<usize>,
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"\[([.#]+)\] ((?:\((?:\d,?)+\) ?)+)\{(?<joltage>(?:\d,?)+)\}");
        let captured = re.captures(s).context("Regex not capturing")?;
        Ok(Machine {
            target_lights: captured[1].chars().fold(0, |acc, c| (acc << 1) | if c == '.' { 0 } else { 1 }),
            button: captured[2].split_whitespace().map(|buttons| buttons[1..(buttons.len()-1)].split(',').map(|n| n.parse::<usize>().unwrap()).fold(0, |acc, n| acc | (1 << n))).collect(),
            joltages: captured.name("joltage").expect("No joltages captured.").as_str().split(',').map(|n| n.parse::<usize>().expect("Joltage not a number")).collect(),
        })
    }
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Machine>> {
    input.lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input.iter()
        .for_each(|m| println!("{m:?}"));

    todo!()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> usize {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
").unwrap()), 7);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
").unwrap()), 0);
    }
}
