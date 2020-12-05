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

fn part1(input: &str) -> Result<u32> {
    Ok(input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_seat_str(line).ok())
        .map(|(row, col)| row * 8 + col)
        .max()
        .unwrap())
}

fn part2(input: &str) -> Result<u32> {
    let used_seats = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .filter_map(|line| parse_seat_str(line).ok())
        .map(|(row, col)| row * 8 + col)
        .collect::<HashSet<_>>();

    let min_seat = *used_seats.iter().min().unwrap();
    let max_seat = *used_seats.iter().max().unwrap();

    let all_seats = min_seat..=max_seat;
    let free_seats = all_seats
        .into_iter()
        .filter(|seat| !used_seats.contains(seat))
        .collect::<Vec<_>>();

    Ok(free_seats[0])
}

fn parse_seat_str(input: &str) -> Result<(u32, u32)> {
    let mut row_min = 0;
    let mut row_max = 128;

    let mut col_min = 0;
    let mut col_max = 8;

    for chr in input.chars() {
        match chr {
            'F' => row_max = row_max - (row_max - row_min) / 2,
            'B' => row_min = row_min + (row_max - row_min) / 2,

            'L' => col_max = col_max - (col_max - col_min) / 2,
            'R' => col_min = col_min + (col_max - col_min) / 2,
            _ => bail!("Invalid chr, {}", chr),
        }
    }

    Ok((row_min, col_min))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_seat_str() {
        assert_eq!(parse_seat_str("FBFBBFFRLR").unwrap(), (44, 5));
    }
}
