use aoc_runner_derive::{aoc, aoc_generator};
use pathfinding::grid::Grid;

#[aoc_generator(day4)]
fn parse(input: &str) -> Grid {
    let mut grid = Grid::from_coordinates(
        &input
            .lines()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '@')
                    .map(move |(x, _)| (x, y))
            })
            .collect::<Vec<_>>(),
    )
    .unwrap();

    grid.enable_diagonal_mode();
    grid
}

#[aoc(day4, part1)]
fn part1(input: &Grid) -> usize {
    input
        .iter()
        .filter(|roll| input.neighbours(*roll).len() < 4)
        .count()
}

#[aoc(day4, part2)]
fn part2(input: &Grid) -> usize {
    let mut grid = input.clone();
    let mut removed = 0;
    while grid.iter().any(|roll| grid.neighbours(roll).len() < 4) {
        let old_grid = grid.clone();
        old_grid
            .iter()
            .filter(|roll| old_grid.neighbours(*roll).len() < 4)
            .for_each(|roll| {
                grid.remove_vertex(roll);
            });
        removed += old_grid.vertices_len() - grid.vertices_len();
    }
    removed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )),
            13
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
"
            )),
            43
        );
    }
}
