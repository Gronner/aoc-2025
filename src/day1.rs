use std::str::FromStr;

use anyhow::{Context, bail};
use aoc_runner_derive::{aoc, aoc_generator};

type Input = Vec<Direction>;

#[derive(Debug)]
enum Direction {
    Right(i64),
    Left(i64),
}

impl FromStr for Direction {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.chars().collect::<Vec<char>>();
        let clicks = s[1..]
            .iter()
            .collect::<String>()
            .parse()
            .context("Failed to parse number")?;
        match s[0] {
            'L' => Ok(Direction::Left(clicks)),
            'R' => Ok(Direction::Right(clicks)),
            _ => bail!("Unknown prefix"),
        }
    }
}

#[aoc_generator(day1)]
fn parse(input: &str) -> anyhow::Result<Input> {
    input.lines().map(Direction::from_str).collect()
}

fn wrapping_add_range(lhs: i64, rhs: i64, max: i64) -> i64 {
    (lhs + rhs).rem_euclid(max + 1)
}

#[aoc(day1, part1)]
fn part1(input: &Input) -> i64 {
    let (password, _) = input.iter().fold((0, 50), |(mut password, dial), dir| {
        let new_dial = match dir {
            Direction::Right(n) => wrapping_add_range(dial, *n, 99),
            Direction::Left(n) => wrapping_add_range(dial, -(*n), 99),
        };
        if new_dial == 0 {
            password += 1;
        }
        (password, new_dial)
    });
    password
}

#[aoc(day1, part2)]
fn part2(input: &Input) -> i64 {
    let (password, _) = input
        .iter()
        .fold((0, 50), |(mut password, dial), dir| match dir {
            Direction::Right(n) => {
                password += n / 100;
                let leftover = n % 100;
                let new_dial = wrapping_add_range(dial, leftover, 99);
                if new_dial < dial || new_dial == 0 {
                    password += 1;
                }
                (password, new_dial)
            }
            Direction::Left(n) => {
                password += n / 100;
                let leftover = n % 100;
                let new_dial = wrapping_add_range(dial, -leftover, 99);
                if dial != 0 && (new_dial > dial || new_dial == 0) {
                    password += 1;
                }
                (password, new_dial)
            }
        });
    password
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                &parse(
                    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
                )
                .unwrap()
            ),
            3
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(
                &parse(
                    "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82"
                )
                .unwrap()
            ),
            6
        );
    }
}
