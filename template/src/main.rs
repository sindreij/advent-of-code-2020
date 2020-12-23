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
    Ok(12)
}

fn part2(input: &str) -> Result<i64> {
    Ok(12)
}
