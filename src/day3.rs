use aoc_runner_derive::{aoc, aoc_generator};
#[aoc_generator(day3)]
fn parse(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("Unexpected character in input.") as i64)
                .collect()
        })
        .collect()
}

fn find_biggest_n_digit_number(bank: &[i64], digits: usize) -> i64 {
    let mut current_pos = 0;
    (0..digits)
        .rev()
        .map(|digit| {
            let max_n = bank[current_pos..(bank.len() - digit)]
                .iter()
                .max()
                .expect("For the numbers, there will always be a biggest number.");
            current_pos += bank[current_pos..]
                .iter()
                .position(|battery| battery == max_n)
                .expect("Maximum value not in numbers.")
                + 1;
            *max_n
        })
        .fold(0, |acc, max_n| acc * 10 + max_n)
}

#[aoc(day3, part1)]
fn part1(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|bank| find_biggest_n_digit_number(bank, 2))
        .sum()
}

#[aoc(day3, part2)]
fn part2(input: &[Vec<i64>]) -> i64 {
    input
        .iter()
        .map(|bank| find_biggest_n_digit_number(bank, 12))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
            )),
            357
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "987654321111111
811111111111119
234234234234278
818181911112111
"
            )),
            3121910778619
        );
    }
}
