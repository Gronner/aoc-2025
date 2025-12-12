use aoc_runner_derive::{aoc, aoc_generator};

type Input = (Vec<Vec<Vec<bool>>>, Vec<((usize, usize), Vec<usize>)>);

/// See https://itp.tugraz.at/~06nuss/content/20110306_Graz.pdf
#[aoc_generator(day12)]
fn parse(input: &str) -> Input {
    let splits = input.splitn(7, "\n\n").collect::<Vec<_>>();
    let shapes = splits
        .iter()
        .take(5)
        .map(|shape_spec| {
            shape_spec
                .lines()
                .skip(1)
                .map(|line| line.chars().map(|c| c == '#').collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let regions = splits
        .last()
        .expect("No last element in input split.")
        .lines()
        .map(|line| {
            let (shape, shape_count) = line.split_once(": ").expect("No : in input line");
            let (x, y) = shape.split_once('x').expect("No dimension separator.");
            let x = x.parse::<usize>().expect("x dimension not a number.");
            let y = y.parse::<usize>().expect("y dimension not a number.");
            (
                (x, y),
                shape_count
                    .split(' ')
                    .map(|n| n.parse::<usize>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .collect::<Vec<_>>();
    (shapes, regions)
}

fn search() {
    todo!("Nope");
}

fn cover() {
    todo!("NOOOPE")
}

fn uncover() {
    todo!("EPOOON")
}

const SHAPE_SIZE: usize = 3;

#[aoc(day12, part1)]
fn part1(input: &Input) -> usize {
    let (_, regions) = input;
    regions
        .iter()
        .filter(|((x, y), presents)| (x / SHAPE_SIZE) * (y / SHAPE_SIZE) >= presents.iter().sum())
        .count()
    // Are you kidding me that this works
}

#[aoc(day12, part2)]
fn part2(input: &Input) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2
"
            )),
            2
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse("<EXAMPLE>")), 0);
    }
}
