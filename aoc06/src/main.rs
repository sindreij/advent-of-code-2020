use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
};

use anyhow::{anyhow, bail, ensure, Result};
use maplit::{hashmap, hashset};
use regex::Regex;

fn main() -> Result<()> {
    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

fn part1(input: &str) -> Result<usize> {
    Ok(input
        .split("\n\n")
        .map(|group| {
            group
                .split('\n')
                .map(|answers| answers.chars().collect::<HashSet<_>>())
                .fold(HashSet::new(), |a, b| {
                    a.union(&b).copied().collect::<HashSet<_>>()
                })
                .len()
        })
        .sum())
}

fn part2(input: &str) -> Result<usize> {
    Ok((input
        .split("\n\n")
        .filter(|a| !a.is_empty())
        .map(|group| {
            group
                .split('\n')
                .filter(|a| !a.is_empty())
                .map(|answers| answers.chars().collect::<HashSet<_>>())
                .fold(hashset! {'æ'}, |acc, x| {
                    if acc.contains(&'æ') && acc.len() == 1 {
                        // For the first element, just return
                        x
                    } else {
                        acc.intersection(&x).copied().collect::<HashSet<_>>()
                    }
                })
                .len()
        })
        .collect::<Vec<_>>())
    .iter()
    .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let output = part1(
            "abc

a
b
c

ab
ac

a
a
a
a

b
",
        )
        .unwrap();

        assert_eq!(output, 11);
    }

    #[test]
    fn test_part2() {
        let output = part2(
            "abc

a
b
c

ab
ac

a
a
a
a

b
",
        )
        .unwrap();

        assert_eq!(output, 6);
    }
}
