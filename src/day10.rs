use anyhow::{Context, Result};
use std::{
    collections::{HashSet, VecDeque},
    str::FromStr,
};

use aoc_runner_derive::{aoc, aoc_generator};
use microlp::{ComparisonOp, OptimizationDirection, Problem};

#[derive(Debug)]
struct Machine {
    target_lights: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl FromStr for Machine {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex!(r"\[([.#]+)\] ((?:\((?:\d,?)+\) ?)+)\{(?<joltage>(?:\d,?)+)\}");
        let captured = re.captures(s).context("Regex not capturing")?;
        Ok(Machine {
            target_lights: captured[1].chars().enumerate().fold(0, |acc, (pos, c)| {
                acc | ((if c == '.' { 0 } else { 1 }) << pos)
            }),
            buttons: captured[2]
                .split_whitespace()
                .map(|buttons| {
                    buttons[1..(buttons.len() - 1)]
                        .split(',')
                        .map(|n| n.parse::<usize>().unwrap())
                        .fold(0, |acc, n| acc | (1 << n))
                })
                .collect(),
            joltages: captured
                .name("joltage")
                .expect("No joltages captured.")
                .as_str()
                .split(',')
                .map(|n| n.parse::<usize>().expect("Joltage not a number"))
                .collect(),
        })
    }
}

fn bfs(machine: &Machine) -> usize {
    let mut queue = VecDeque::new();
    queue.push_back((0, 0));
    let mut cache = HashSet::new();
    while let Some((lights, count)) = queue.pop_front() {
        if cache.contains(&(lights, count)) {
            continue;
        }
        if lights == machine.target_lights {
            return count;
        }
        cache.insert((lights, count));
        for button in machine.buttons.iter() {
            let next_state = lights ^ button;
            queue.push_back((next_state, count + 1));
        }
    }
    unreachable!("No solution found for machine");
}

#[aoc_generator(day10)]
fn parse(input: &str) -> Result<Vec<Machine>> {
    input
        .lines()
        .map(Machine::from_str)
        .collect::<Result<Vec<_>>>()
}

#[aoc(day10, part1)]
fn part1(input: &[Machine]) -> usize {
    input.iter().map(bfs).sum()
}

fn solve(machine: &Machine) -> f64 {
    let mut problem = Problem::new(OptimizationDirection::Minimize);
    let variables = (0..machine.buttons.len())
        .map(|_| problem.add_integer_var(1.0, (0, i32::MAX)))
        .collect::<Vec<_>>();

    machine
        .joltages
        .iter()
        .enumerate()
        .for_each(|(idx, joltage)| {
            // j_n == joltage
            problem.add_constraint(
                variables
                    .iter()
                    .enumerate()
                    .map(|(btn_num, &variable)| {
                        // j_n = sum(button_m * (if applies to idx))
                        (
                            variable,
                            ((machine.buttons[btn_num] & (1 << idx)) as i64).signum() as f64,
                        )
                    })
                    .collect::<Vec<_>>(),
                ComparisonOp::Eq,
                *joltage as f64,
            )
        });
    problem.solve().expect("No solution found").objective()
}

#[aoc(day10, part2)]
fn part2(input: &[Machine]) -> f64 {
    input.iter().map(solve).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                &parse(
                    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
                )
                .unwrap()
            ),
            7
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                &parse(
                    "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
"
                )
                .unwrap()
            ),
            33.0
        );
    }
}
