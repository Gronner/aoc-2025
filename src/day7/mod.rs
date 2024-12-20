use anyhow::{Context, Result};
use itertools::repeat_n;
use rayon::prelude::*;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
#[allow(unused)]
use itertools::Itertools;

type Output = u64;
type Input = Vec<Equation>;

#[derive(Debug)]
pub struct Equation {
    result: u64,
    operands: Vec<u64>,
}

fn add(lhs: u64, rhs: u64) -> u64 {
    lhs + rhs
}

fn mul(lhs: u64, rhs: u64) -> u64 {
    lhs * rhs
}

fn concat(lhs: u64, rhs: u64) -> u64 {
    lhs * (10_u64.pow(lhs.ilog10())) + rhs
}

impl Equation {
    fn is_valid(&self, operations: &[fn(u64, u64) -> u64]) -> bool {
        let missing = self.operands.len() - 1;
        // This creates all possible sequences with repetition
        repeat_n(operations, missing)
            .multi_cartesian_product()
            .any(|ops| {
                ops.iter()
                    .zip(self.operands[1..].iter())
                    .fold(self.operands[0], |acc, (op, operand)| op(acc, *operand))
                    == self.result
            })
    }
}

impl FromStr for Equation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (result, operands) = s.split_once(':').context("No split found")?;
        Ok(Self {
            result: result.parse().context("No valid integer")?,
            operands: operands
                .split_whitespace()
                .map(|n| n.parse::<u64>().context("No valid integer"))
                .collect::<Result<Vec<u64>>>()?,
        })
    }
}

#[aoc_generator(day7)]
pub fn input_generator(input: &str) -> Result<Input> {
    input
        .lines()
        .map(Equation::from_str)
        .collect::<Result<Vec<Equation>>>()
}

#[aoc(day7, part1)]
pub fn solve_part1(input: &Input) -> Output {
    const OPERATIONS: [fn(u64, u64) -> u64; 2] = [add, mul];
    input
        .par_iter()
        .filter(|eq| eq.is_valid(&OPERATIONS))
        .map(|eq| eq.result)
        .sum()
}

#[aoc(day7, part2)]
pub fn solve_part2(input: &Input) -> Output {
    const OPERATIONS: [fn(u64, u64) -> u64; 3] = [add, mul, concat];
    input
        .par_iter()
        .filter(|eq| eq.is_valid(&OPERATIONS))
        .map(|eq| eq.result)
        .sum()
}

pub fn part1(input: &str) -> impl std::fmt::Display {
    solve_part1(&input_generator(input).unwrap())
}

pub fn part2(input: &str) -> impl std::fmt::Display {
    solve_part2(&input_generator(input).unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample() -> &'static str {
        "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"
    }

    #[test]
    fn samples_part1() {
        assert_eq!(3749, solve_part1(&input_generator(sample()).unwrap()));
    }

    #[test]
    fn samples_part2() {
        assert_eq!(11387, solve_part2(&input_generator(sample()).unwrap()));
    }
}
