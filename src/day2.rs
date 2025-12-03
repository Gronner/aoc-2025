use anyhow::Result;
use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Id {
    start: i64,
    end: i64,
}

impl Id {
    fn sum_invalids(&self) -> i64 {
        (self.start..=self.end)
            .map(|n| n.to_string())
            .filter(|n| {
                let mid = n.len() / 2;
                n[..mid] == n[mid..]
            })
            .map(|n| n.parse::<i64>().expect("Was already a number can not fail"))
            .sum()
    }

    fn sum_invalids2(&self) -> i64 {
        (self.start..=self.end)
            .map(|n| n.to_string())
            .filter(|n| is_repeated(n))
            .map(|n| n.parse::<i64>().expect("Was already a number can not fail"))
            .sum()
    }
}

/// A string consists only of the same substring if rotating it yields the same string:
/// aa => aa
/// abab => baba => abab
fn is_repeated(n: &str) -> bool {
    let mid = n.len() / 2;
    let n = n.as_bytes();
    (1..=mid).any(|rot| n == [&n[rot..], &n[..rot]].concat())
}

impl FromStr for Id {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once("-").unwrap();
        Ok(Id {
            start: start.parse()?,
            end: end.parse()?,
        })
    }
}

#[aoc_generator(day2)]
fn parse(input: &str) -> Result<Vec<Id>> {
    input.split(",").map(Id::from_str).collect()
}

#[aoc(day2, part1)]
fn part1(input: &[Id]) -> i64 {
    input.iter().map(|n| n.sum_invalids()).sum()
}

#[aoc(day2, part2)]
fn part2(input: &[Id]) -> i64 {
    input.iter().map(|n| n.sum_invalids2()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").unwrap()), 1227775554);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124").unwrap()), 4174379265);
    }
}
