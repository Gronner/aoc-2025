use anyhow::Result;
use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::{
    cmp::{max, min},
    ops::RangeInclusive,
};

#[aoc_generator(day5)]
fn parse(input: &str) -> (Vec<RangeInclusive<i64>>, Vec<i64>) {
    let (fresh_ingredients, available_ingredients) = input
        .split_once("\n\n")
        .expect("Empty line missing in input.");
    let fresh_ingredients = fresh_ingredients
        .lines()
        .map(|line| {
            let (start, end) = line
                .split_once("-")
                .expect("Fresh ingredient line without -.");
            (start.parse::<i64>().expect("Range start is not a number."))
                ..=(end.parse::<i64>().expect("Range end is not a number."))
        })
        .collect::<Vec<_>>();
    let available_ingredients = available_ingredients
        .lines()
        .map(|line| {
            line.parse::<i64>()
                .expect("Not a ingredient is not a number.")
        })
        .collect::<Vec<_>>();
    (fresh_ingredients, available_ingredients)
}

#[aoc(day5, part1)]
fn part1(input: &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    let (fresh_ingredients, available_ingredients) = input;
    available_ingredients
        .iter()
        .filter(|ingredient| {
            fresh_ingredients
                .iter()
                .any(|fresh_range| fresh_range.contains(ingredient))
        })
        .count()
}

#[derive(Clone, Copy, Debug, Eq)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn overlap(self, rhs: Self) -> bool {
        // TODO: Simplify the logic
        (self.start <= rhs.start && self.end <= rhs.end && self.end >= rhs.start)        // self overlaps beginning of rhs
            || (rhs.start <= self.start && rhs.end <= self.end && rhs.end >= self.start) // self overlaps ending of rhs
            || (self.start <= rhs.start && self.end >= rhs.end) // self wholy contains rhs
            || (rhs.start <= self.start && rhs.end >= self.end) // self is wohly contained in rhs {
    }

    fn merge(self, rhs: Self) -> Self {
        Self {
            start: min(self.start, self.start),
            end: max(self.end, rhs.end),
        }
    }

    fn len(&self) -> usize {
        (self.end - self.start + 1) as usize
    }
}

impl PartialEq for Range {
    fn eq(&self, other: &Self) -> bool {
        self.start == other.start && self.end == other.end
    }
}

impl PartialOrd for Range {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Range {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.start.cmp(&other.start) {
            core::cmp::Ordering::Equal => self.end.cmp(&other.end),
            ord => ord,
        }
    }
}

#[aoc(day5, part2)]
fn part2(input: &(Vec<RangeInclusive<i64>>, Vec<i64>)) -> usize {
    let (fresh_ingredients, _) = input;
    let fresh_ingredients = fresh_ingredients
        .iter()
        .map(|range| Range {
            start: *range.start(),
            end: *range.end(),
        })
        .sorted()
        .collect::<Vec<_>>();

    let mut unique_ingredients = Vec::new();

    let mut current_range = fresh_ingredients[0];
    for ingredient in fresh_ingredients.iter().skip(1) {
        if current_range.overlap(*ingredient) {
            current_range = current_range.merge(*ingredient);
        } else {
            unique_ingredients.push(current_range);
            current_range = *ingredient;
        }
    }
    unique_ingredients.push(current_range);

    unique_ingredients.iter().map(|range| range.len()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            )),
            3
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "3-5
10-14
16-20
12-18

1
5
8
11
17
32"
            )),
            14
        );
    }
}
