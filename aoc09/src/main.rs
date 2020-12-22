use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::{anyhow, bail, Result};
use maplit::{hashmap, hashset};

fn main() -> Result<()> {
    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

fn part1(input: &str) -> Result<i32> {
    solve_part1(input, 25)
}

fn solve_part1(input: &str, preamble: usize) -> Result<i32> {
    let mut last_numbers = VecDeque::new();

    for number in input
        .split("\n")
        .filter(|a| !a.is_empty())
        .filter_map(|a| a.parse::<i32>().ok())
    {
        if last_numbers.len() == preamble {
            let mut correct = false;
            'outer: for a in &last_numbers {
                for b in &last_numbers {
                    if a + b == number {
                        correct = true;
                        break 'outer;
                    }
                }
            }
            if !correct {
                return Ok(number);
            }
        }
        // check that it is correct
        last_numbers.push_back(number);
        if last_numbers.len() > preamble {
            last_numbers.pop_front();
        }
    }

    Ok(12)
}

fn part2(input: &str) -> Result<i32> {
    solve_part2(input, 25)
}

fn solve_part2(input: &str, preamble: usize) -> Result<i32> {
    let invalid_number = solve_part1(input, preamble)?;

    let numbers = input
        .split("\n")
        .filter(|a| !a.is_empty())
        .filter_map(|a| a.parse::<i32>().ok())
        .collect::<Vec<_>>();

    for start in 0..numbers.len() {
        for end in start..numbers.len() {
            let sum = numbers[start..end].iter().sum::<i32>();
            if sum == invalid_number {
                let max = numbers[start..end].iter().max().unwrap();
                let min = numbers[start..end].iter().min().unwrap();
                return Ok(max + min);
            } else if sum > invalid_number {
                break;
            }
        }
    }

    Ok(12)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_part1() {
        assert_eq!(
            solve_part1(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
                5
            )
            .unwrap(),
            127
        )
    }

    #[test]
    fn test_solve_part2() {
        assert_eq!(
            solve_part2(
                "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576
",
                5
            )
            .unwrap(),
            62
        )
    }
}
