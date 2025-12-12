use std::{collections::HashMap, hash::{DefaultHasher, Hash, Hasher}};

use aoc_runner_derive::{aoc, aoc_generator};
use memoize::memoize;
use pathfinding::prelude::yen;

fn compute_hash(inp: &str) -> u64 {
    let mut state = DefaultHasher::new();
    inp.hash(&mut state);
    state.finish()
}

#[aoc_generator(day11)]
fn parse(input: &str) -> HashMap<u64, Vec<u64>> {
    input
        .lines()
        .map(|line| {
            let (device, connections) = line.split_once(": ").unwrap();
            let connections = connections
                .split_whitespace()
                .map(compute_hash)
                .collect::<Vec<_>>();
            (compute_hash(device), connections)
        })
        .collect::<HashMap<_, _>>()
}

#[aoc(day11, part1)]
fn part1(input: &HashMap<u64, Vec<u64>>) -> usize {
    yen(
        &compute_hash("you"),
        |cur| {
            input
                .get(cur)
                .unwrap()
                .iter()
                .map(|next| (*next, 1))
                .collect::<Vec<_>>()
        },
        |cur| {
            *cur == compute_hash("out")
        },
        1000,
    ).len()
}

#[memoize(Ignore: connections)]
fn dfs(start: u64, end: u64, connections: &HashMap<u64, Vec<u64>>) -> usize {
    if start == end {
        return 1;
    }
    connections.get(&start).unwrap_or(&vec![])
        .iter()
        .map(|&next| dfs(next, end, connections))
        .sum()

}

#[aoc(day11, part2)]
fn part2(input: &HashMap<u64, Vec<u64>>) -> usize {
    let svr = compute_hash("svr");
    let dac = compute_hash("dac");
    let fft = compute_hash("fft");
    let out = compute_hash("out");

    // Faster version thanks to https://www.reddit.com/r/adventofcode/comments/1pjp1rm/2025_day_11_solutions/ntf4e0t/
    dfs(svr, dac, input) * 
    dfs(dac, fft, input) * 
    dfs(fft, out, input) +
    dfs(svr, fft, input) * 
    dfs(fft, dac, input) * 
    dfs(dac, out, input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
"
            )),
            5
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse("svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
")),
            2
        );
    }
}
