use std::{collections::HashSet, fs::read_to_string};

use anyhow::{bail, Result};

fn main() -> Result<()> {
    println!("Hello, world!");

    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

fn part1(input: &str) -> Result<i64> {
    let numbers = input
        .split("\n")
        .filter_map(|line| line.parse().ok())
        .collect::<HashSet<i64>>();

    for number in numbers.iter() {
        let other = 2020 - number;
        if numbers.contains(&other) {
            return Ok(number * other);
        }
    }

    bail!("Did not find a matching number");
}

fn part2(input: &str) -> Result<i64> {
    let numbers = input
        .split("\n")
        .filter_map(|line| line.parse().ok())
        .collect::<HashSet<i64>>();

    for number in numbers.iter() {
        for number2 in numbers.iter() {
            for number3 in numbers.iter() {
                if number + number2 + number3 == 2020 {
                    return Ok(number * number2 * number3);
                }
            }
        }
    }

    bail!("Did not find a matching number");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_1() {
        assert_eq!(
            part1(
                "1721
979
366
299
675
1456
"
            )
            .unwrap(),
            514579
        )
    }
    #[test]
    fn test_part2_1() {
        assert_eq!(
            part2(
                "1721
979
366
299
675
1456
"
            )
            .unwrap(),
            241861950
        )
    }
}
