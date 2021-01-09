mod vector;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::{anyhow, bail, Result};
use maplit::{hashmap, hashset};
use vector::Vector;

fn main() -> Result<()> {
    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

#[derive(Debug)]
enum Instruction {
    North(i64),
    South(i64),
    East(i64),
    West(i64),
    TurnLeft(i64),
    TurnRight(i64),
    MoveForward(i64),
}

fn part1(input: &str) -> Result<i64> {
    use Instruction::*;
    let instructions = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let command = &line[0..1];
            let value: i64 = (&line[1..]).parse().unwrap();

            match command {
                "N" => North(value),
                "S" => South(value),
                "E" => East(value),
                "W" => West(value),
                "L" => TurnLeft(value),
                "R" => TurnRight(value),
                "F" => MoveForward(value),
                _ => panic!("Could not parse {}", line),
            }
        });

    let mut pos = Vector(0, 0);
    let mut direction = Vector(1, 0);
    for inst in instructions {
        match inst {
            North(val) => pos += Vector::NORTH * val,
            South(val) => pos += Vector::SOUTH * val,
            East(val) => pos += Vector::EAST * val,
            West(val) => pos += Vector::WEST * val,
            TurnLeft(val) => direction = direction.rotate(-val),
            TurnRight(val) => direction = direction.rotate(val),
            MoveForward(val) => pos += direction * val,
        }
    }

    Ok(pos.x().abs() + pos.y().abs())
}

fn part2(input: &str) -> Result<i64> {
    use Instruction::*;
    let instructions = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            let command = &line[0..1];
            let value: i64 = (&line[1..]).parse().unwrap();

            match command {
                "N" => North(value),
                "S" => South(value),
                "E" => East(value),
                "W" => West(value),
                "L" => TurnLeft(value),
                "R" => TurnRight(value),
                "F" => MoveForward(value),
                _ => panic!("Could not parse {}", line),
            }
        });

    let mut pos = Vector(0, 0);
    let mut waypoint = Vector::NORTH * 1 + Vector::EAST * 10;
    for inst in instructions {
        match inst {
            North(val) => waypoint += Vector::NORTH * val,
            South(val) => waypoint += Vector::SOUTH * val,
            East(val) => waypoint += Vector::EAST * val,
            West(val) => waypoint += Vector::WEST * val,
            TurnLeft(val) => waypoint = waypoint.rotate(-val),
            TurnRight(val) => waypoint = waypoint.rotate(val),
            MoveForward(val) => pos += waypoint * val,
        }
    }

    Ok(pos.x().abs() + pos.y().abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "F10
N3
F7
R90
F11"
            )
            .unwrap(),
            25
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "F10
N3
F7
R90
F11"
            )
            .unwrap(),
            286
        )
    }
}
