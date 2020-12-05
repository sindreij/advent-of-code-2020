use std::fs::read_to_string;

use anyhow::{anyhow, bail, Result};
use regex::Regex;

fn main() -> Result<()> {
    println!("Hello, world!");

    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| is_valid_password(line).unwrap())
        .filter(|is_valid| *is_valid)
        .count())
}

fn part2(input: &str) -> Result<usize> {
    Ok(input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| is_valid_password2(line).unwrap())
        .filter(|is_valid| *is_valid)
        .count())
}

fn is_valid_password2(password: &str) -> Result<bool> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z]): (.*)$").unwrap();

    let regexmatch = re
        .captures(password)
        .ok_or_else(|| anyhow!("Regex did not match"))?;

    let num_1: usize = regexmatch.get(1).unwrap().as_str().parse().unwrap();
    let num_2: usize = regexmatch.get(2).unwrap().as_str().parse().unwrap();
    let char = regexmatch.get(3).unwrap().as_str().chars().next().unwrap();
    let pass = regexmatch.get(4).unwrap().as_str();

    let chars = pass.chars().collect::<Vec<_>>();
    let first_match = chars[num_1 - 1] == char;
    let second_match = chars[num_2 - 1] == char;

    Ok((first_match || second_match) && !(first_match && second_match))
}

fn is_valid_password(password: &str) -> Result<bool> {
    let re = Regex::new(r"^(\d+)-(\d+) ([a-zA-Z])(.*)$").unwrap();

    let regexmatch = re
        .captures(password)
        .ok_or_else(|| anyhow!("Regex did not match"))?;

    let num_from = regexmatch.get(1).unwrap().as_str().parse().unwrap();
    let num_to = regexmatch.get(2).unwrap().as_str().parse().unwrap();
    let char = regexmatch.get(3).unwrap().as_str().chars().next().unwrap();
    let pass = regexmatch.get(4).unwrap().as_str();

    let instances = pass.chars().filter(|ch| *ch == char).count();

    Ok(instances >= num_from && instances <= num_to)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_password() {
        assert!(is_valid_password("1-3 a: abcde").unwrap());
        assert!(is_valid_password("2-9 c: ccccccccc").unwrap());
        assert!(!is_valid_password("1-3 b: cdefg").unwrap());
    }

    #[test]
    fn test_is_valid_password2() {
        assert!(is_valid_password2("1-3 a: abcde").unwrap());
        assert!(!is_valid_password2("1-3 b: cdefg").unwrap());
        assert!(!is_valid_password2("2-9 c: ccccccccc").unwrap());
    }

    //     #[test]
    //     fn test_part1_1() {
    //         assert_eq!(
    //             part1(
    //                 "1-3 a: abcde
    // 1-3 b: cdefg
    // 2-9 c: ccccccccc
    // "
    //             )
    //             .unwrap(),
    //             514579
    //         )
    //     }
}
