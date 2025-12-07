use std::collections::HashSet;

use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;

#[aoc_generator(day7)]
fn parse(input: &str) -> (usize, usize, HashSet<(usize, usize)>) {
    (
        input
            .chars()
            .position(|c| c == 'S')
            .expect("No start found"),
        input.lines().count(),
        input
            .lines()
            .enumerate()
            .flat_map(|(r, line)| {
                line.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '^')
                    .map(move |(p, _)| (r, p))
            })
            .collect(),
    )
}

#[aoc(day7, part1)]
fn part1(input: &(usize, usize, HashSet<(usize, usize)>)) -> usize {
    let (start, max_depth, map) = input;

    let mut tachyons = HashSet::new();
    tachyons.insert(*start);

    let mut splits = 0;

    for depth in 1..*max_depth {
        let new_tachyons = tachyons
            .iter()
            .flat_map(|tachyon| {
                if map.contains(&(depth, *tachyon)) {
                    splits += 1;
                    vec![*tachyon - 1, *tachyon + 1]
                } else {
                    vec![*tachyon]
                }
            })
            .collect::<HashSet<_>>();
        tachyons = new_tachyons;
    }

    splits
}

#[memoize(Ignore: map)]
fn split(pos: usize, depth: usize, max_depth: usize, map: &HashSet<(usize, usize)>) -> usize {
    if depth == max_depth {
        return 1;
    }
    if map.contains(&(depth, pos)) {
        split(pos - 1, depth + 1, max_depth, map) + split(pos + 1, depth + 1, max_depth, map)
    } else {
        split(pos, depth + 1, max_depth, map)
    }
}

#[aoc(day7, part2)]
fn part2(input: &(usize, usize, HashSet<(usize, usize)>)) -> usize {
    let (start, max_depth, map) = input;
    split(*start, 1, *max_depth, map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
.............."
            )),
            21
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
.............."
            )),
            40
        );
    }
}
