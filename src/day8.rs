use anyhow::Result;
use itertools::Itertools;
use std::{collections::HashSet, str::FromStr};

use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct JunctionBox {
    x: isize,
    y: isize,
    z: isize,
}

impl JunctionBox {
    fn distance_sq(&self, other: &Self) -> isize {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx * dx + dy * dy + dz * dz
    }
}

impl FromStr for JunctionBox {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut coords = s
            .splitn(3, ',')
            .map(|n| n.parse::<isize>().expect("Coordinate not a number"));
        Ok(JunctionBox {
            x: coords.next().unwrap(),
            y: coords.next().unwrap(),
            z: coords.next().unwrap(),
        })
    }
}

#[aoc_generator(day8)]
fn parse(input: &str) -> Result<Vec<JunctionBox>> {
    input
        .lines()
        .map(JunctionBox::from_str)
        .collect::<Result<Vec<_>>>()
}

struct UnionFind {
    nodes: Box<[(usize, usize)]>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            nodes: (0..n).map(|i| (i, 1)).collect(),
        }
    }

    fn find(&mut self, i: usize) -> usize {
        if self.nodes[i].0 == i {
            i
        } else {
            let root = self.find(self.nodes[i].0);
            self.nodes[i].0 = root;
            root
        }
    }

    fn union(&mut self, i: usize, j: usize) -> bool {
        let root_i = self.find(i);
        let root_j = self.find(j);

        if root_i == root_j {
            false
        } else {
            if self.nodes[root_i].1 < self.nodes[root_j].1 {
                self.nodes[root_i].0 = root_j;
                self.nodes[root_j].1 += self.nodes[root_i].1;
            } else {
                self.nodes[root_j].0 = root_i;
                self.nodes[root_i].1 += self.nodes[root_j].1;
            }
            true
        }
    }
}

#[aoc(day8, part1)]
fn part1(input: &[JunctionBox]) -> usize {
    let junction_pairs = input
        .iter()
        .enumerate()
        .permutations(2)
        .map(|jb_permutation| {
            (
                jb_permutation[0].0,
                jb_permutation[1].0,
                jb_permutation[0].1.distance_sq(jb_permutation[1].1),
            )
        })
        .sorted_by(|(_, _, distance1), (_, _, distance2)| distance1.cmp(distance2))
        .map(|(id1, id2, _)| (id1, id2))
        .collect::<Vec<_>>();

    let mut union_find = UnionFind::new(input.len());

    junction_pairs.iter()
        .take(1000)
        .for_each(|pair| {union_find.union(pair.0, pair.1);});

    let mut visited = HashSet::new();
    let sizes = (0..input.len()).
        filter_map(|idx| {
            let root = union_find.find(idx);
            if visited.insert(root) {
                Some(union_find.nodes[root].1)
            } else {
                None
            }
        })
    .collect::<Vec<_>>();
    sizes.iter().sorted().rev().for_each(|n| println!("{n}"));
    sizes.iter().sorted().rev().take(3).product()

}

#[aoc(day8, part2)]
fn part2(input: &[JunctionBox]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(
                &parse(
                    "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689"
                )
                .unwrap()
            ),
            40
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
").unwrap()), 0);
    }
}
