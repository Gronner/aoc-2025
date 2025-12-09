use itertools::Itertools;
use std::isize;

use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day9)]
fn parse(input: &str) -> Vec<(isize, isize)> {
    input
        .lines()
        .map(|line| line.split_once(',').expect("Coordinate without a comma."))
        .map(|(x, y)| {
            (
                x.parse::<isize>().expect("X coordinate is not a number."),
                y.parse::<isize>().expect("Y coordinate is not a number."),
            )
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Debug)]
struct Rectangle {
    x_max: isize,
    x_min: isize,
    y_max: isize,
    y_min: isize,
}

impl Rectangle {
    fn new(corner1: &(isize, isize), corner2: &(isize, isize)) -> Self {
        Self {
            x_max: corner1.0.max(corner2.0),
            x_min: corner1.0.min(corner2.0),
            y_max: corner1.1.max(corner2.1),
            y_min: corner1.1.min(corner2.1),
        }
    }

    fn area(&self) -> isize {
        (self.x_max - self.x_min + 1) * (self.y_max - self.y_min + 1)
    }

    fn within_corners(&self, corner1: &(isize, isize), corner2: &(isize, isize)) -> bool {
        return (corner1.0.max(corner2.0) <= self.x_min || corner1.0.min(corner2.0) >= self.x_max)
            || (corner1.1.max(corner2.1) <= self.y_min || corner1.1.min(corner2.1) >= self.y_max);
    }
}

#[aoc(day9, part1)]
fn part1(input: &[(isize, isize)]) -> isize {
    input
        .iter()
        .combinations(2)
        .map(|tile_pair| {
            Rectangle::new(tile_pair[0], tile_pair[1]).area()
        })
        .max()
        .unwrap()
}

#[aoc(day9, part2)]
fn part2(input: &[(isize, isize)]) -> isize {
    input
        .iter()
        .combinations(2)
        //  A---B
        //  |   |
        //  C---D
        .map(|tile_pair| {
            Rectangle::new(tile_pair[0], tile_pair[1])
        })
        .filter(|rectangle| {
            input
                .iter()
                .chain(&[input[0]])
                .tuple_windows()
                .all(|(corner1, corner2)| rectangle.within_corners(corner1, corner2))
        })
        .map(|rectangle| {
            rectangle.area()
        })
        .max()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
            )),
            50
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
"
            )),
            24
        );
    }
}
