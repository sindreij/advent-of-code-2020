mod vector;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::{anyhow, bail, Result};
use maplit::{hashmap, hashset};

use crate::vector::Vector;

fn main() -> Result<()> {
    println!("part1: {}", part1(&read_input()?)?);
    println!("part2: {}", part2(&read_input()?)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum State {
    Empty,
    Occupied,
    Floor,
}

fn part2(input: &str) -> Result<usize> {
    let seats = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'L' => State::Empty,
                    '.' => State::Floor,
                    _ => panic!("Invalid input, {}", char),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = seats.len() as i64;
    let width = seats[0].len() as i64;

    let mut state = seats;

    loop {
        let new_state = state
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        if *seat == State::Floor {
                            return State::Floor;
                        }

                        let (x, y) = (x as i64, y as i64);
                        let start = Vector(x, y);

                        let num_occupied = [
                            Vector(-1, -1),
                            Vector(-1, 0),
                            Vector(-1, 1),
                            Vector(0, -1),
                            Vector(0, 1),
                            Vector(1, -1),
                            Vector(1, 0),
                            Vector(1, 1),
                        ]
                        .iter()
                        .filter(|direction| {
                            for steps in 1.. {
                                let Vector(x, y) = start + **direction * steps;
                                if !(x >= 0 && y >= 0 && x < width && y < height) {
                                    return false;
                                }

                                let seat = state[y as usize][x as usize];

                                match seat {
                                    State::Empty => return false,
                                    State::Occupied => return true,
                                    State::Floor => {}
                                }
                            }
                            unreachable!();
                        })
                        .count();

                        match seat {
                            State::Empty if num_occupied == 0 => State::Occupied,
                            State::Occupied if num_occupied >= 5 => State::Empty,
                            otherwise => *otherwise,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if state == new_state {
            break;
        }

        state = new_state;
    }

    Ok(state
        .iter()
        .map(|row| row.iter().filter(|el| **el == State::Occupied).count())
        .sum())
}

fn print(input: &[Vec<State>]) {
    for row in input {
        for col in row {
            let chr = match col {
                State::Empty => "L",
                State::Occupied => "#",
                State::Floor => ".",
            };
            print!("{}", chr);
        }
        println!();
    }
    println!();
    println!();
}

fn part1(input: &str) -> Result<usize> {
    let seats = input
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|char| match char {
                    'L' => State::Empty,
                    '.' => State::Floor,
                    _ => panic!("Invalid input, {}", char),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let height = seats.len() as i32;
    let width = seats[0].len() as i32;

    let mut state = seats;

    loop {
        let new_state = state
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, seat)| {
                        if *seat == State::Floor {
                            return State::Floor;
                        }

                        let (x, y) = (x as i32, y as i32);

                        let num_occupied = [
                            (x - 1, y - 1),
                            (x - 1, y),
                            (x - 1, y + 1),
                            (x, y - 1),
                            (x, y + 1),
                            (x + 1, y - 1),
                            (x + 1, y),
                            (x + 1, y + 1),
                        ]
                        .iter()
                        .filter(|(x, y)| *x >= 0 && *y >= 0 && *x < width && *y < height)
                        .filter(|(x, y)| state[*y as usize][*x as usize] == State::Occupied)
                        .count();

                        match seat {
                            State::Empty if num_occupied == 0 => State::Occupied,
                            State::Occupied if num_occupied >= 4 => State::Empty,
                            otherwise => *otherwise,
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        if state == new_state {
            break;
        }

        state = new_state;
    }

    Ok(state
        .iter()
        .map(|row| row.iter().filter(|el| **el == State::Occupied).count())
        .sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            )
            .unwrap(),
            37
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL"
            )
            .unwrap(),
            26
        )
    }
}
