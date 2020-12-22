use std::{
    cmp::max,
    collections::{HashMap, HashSet},
    fs::read_to_string,
    ops::RangeInclusive,
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

#[derive(Debug, Copy, Clone)]
enum OpCode {
    Nop(i32),
    Acc(i32),
    Jmp(i32),
}

fn part1(input: &str) -> Result<i32> {
    let instructions = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            match &*parts {
                [instruction, value] => Some((*instruction, value.parse::<i32>().ok()?)),
                _ => None,
            }
        })
        .filter_map(|(instruction, value)| match instruction {
            "nop" => Some(OpCode::Nop(value)),
            "acc" => Some(OpCode::Acc(value)),
            "jmp" => Some(OpCode::Jmp(value)),
            _ => None,
        })
        .collect::<Vec<_>>();

    let mut acc = 0;
    let mut pc = 0;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&pc) {
            break;
        }
        visited.insert(pc);
        match instructions[pc] {
            OpCode::Nop(_) => {}
            OpCode::Acc(value) => acc += value,
            OpCode::Jmp(value) => pc = ((pc as i32) + value - 1) as usize,
        }
        pc += 1
    }

    Ok(acc)
}

fn run(code: &Vec<OpCode>) -> Option<i32> {
    let mut acc = 0;
    let mut pc = 0;
    let mut visited = HashSet::new();

    loop {
        if pc == code.len() {
            return Some(acc);
        }
        if visited.contains(&pc) {
            return None;
        }
        visited.insert(pc);
        match code[pc] {
            OpCode::Nop(_) => {}
            OpCode::Acc(value) => acc += value,
            OpCode::Jmp(value) => pc = ((pc as i32) + value - 1) as usize,
        }
        pc += 1
    }
}

fn part2(input: &str) -> Result<i32> {
    let instructions = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .filter_map(|line| {
            let parts = line.split(" ").collect::<Vec<&str>>();
            match &*parts {
                [instruction, value] => Some((*instruction, value.parse::<i32>().ok()?)),
                _ => None,
            }
        })
        .filter_map(|(instruction, value)| match instruction {
            "nop" => Some(OpCode::Nop(value)),
            "acc" => Some(OpCode::Acc(value)),
            "jmp" => Some(OpCode::Jmp(value)),
            _ => None,
        })
        .collect::<Vec<_>>();

    run(&instructions);

    for (i, op) in instructions.iter().enumerate() {
        match op {
            OpCode::Acc(_) => {}
            OpCode::Nop(val) => {
                let mut code = instructions.clone();
                code[i] = OpCode::Jmp(*val);
                if let Some(result) = run(&code) {
                    println!("Changed to jmp {}", i);
                    return Ok(result);
                }
            }
            OpCode::Jmp(val) => {
                let mut code = instructions.clone();
                code[i] = OpCode::Nop(*val);
                if let Some(result) = run(&code) {
                    println!("Changed to nop {}", i);
                    return Ok(result);
                }
            }
        }
    }

    Ok(-1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"
            )
            .unwrap(),
            5
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part1(
                "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6
"
            )
            .unwrap(),
            8
        )
    }
}
