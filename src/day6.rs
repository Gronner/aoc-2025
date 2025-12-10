use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Problem {
    numbers: Vec<i64>,
    operation: char,
}

impl Problem {
    fn compute(&self) -> i64 {
        self.numbers
            .iter()
            .skip(1)
            .fold(self.numbers[0], |acc, n| match self.operation {
                '+' => acc + n,
                '*' => acc * n,
                e => panic!("Unkown math symbol: {e}"),
            })
    }
}

#[aoc_generator(day6)]
fn parse(input: &str) -> String {
    input.to_owned()
}

#[aoc(day6, part1)]
fn part1(input: &str) -> i64 {
    let depth = input.lines().count();
    let numbers = input
        .lines()
        .take(depth - 1)
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<i64>().expect("Line not a number"))
                .collect::<Vec<i64>>()
        })
        .collect::<Vec<_>>();
    let symbols = input
        .lines()
        .last()
        .unwrap()
        .split_whitespace()
        .map(|s| s.chars().next().unwrap())
        .collect::<Vec<_>>();

    let width = numbers[0].len();

    let mut problems = Vec::new();
    for w in 0..width {
        let mut nums = Vec::new();
        for digits in numbers.iter().take(depth - 1) {
            nums.push(digits[w]);
        }
        problems.push(Problem {
            numbers: nums,
            operation: symbols[w],
        });
    }

    problems.iter().map(|prob| prob.compute()).sum()
}

#[aoc(day6, part2)]
fn part2(input: &str) -> i64 {
    let depth = input.lines().count();
    let input_grid = input
        .lines()
        .take(depth - 1)
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut symbols = input.lines().last().unwrap().chars().filter(|c| *c != ' ');

    let mut problems = Vec::new();
    let mut nums = Vec::new();
    for w in 0..input_grid[0].len() {
        let mut number = String::new();
        for d in 0..(depth - 1) {
            if input_grid[d][w].is_ascii_digit() {
                number.push(input_grid[d][w]);
            }
        }
        if !number.is_empty() {
            nums.push(number.parse::<i64>().expect("Number is not a number"));
        } else {
            problems.push(Problem {
                numbers: nums.clone(),
                operation: symbols.next().expect("Not enough symbols"),
            });
            nums.clear();
        }
    }
    problems.push(Problem {
        numbers: nums.clone(),
        operation: symbols.next().expect("Not enough symbols"),
    });

    problems.iter().map(|prob| prob.compute()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(
            part1(&parse(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  
"
            )),
            4277556
        );
    }

    #[test]
    fn part2_example() {
        assert_eq!(
            part2(&parse(
                "123 328  51 64 
 45 64  387 23 
  6 98  215 314
*   +   *   +  "
            )),
            3263827
        );
    }
}
