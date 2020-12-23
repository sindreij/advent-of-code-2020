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
    let mut numbers = input
        .split("\n")
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<i64>>();

    numbers.sort();

    let mut jolt = 0;

    let mut counts = HashMap::new();

    for number in numbers {
        let diff = number - jolt;
        *counts.entry(diff).or_insert(0) += 1;
        jolt = number;
    }

    // The one between the last and the laptop
    *counts.entry(3).or_insert(0) += 1;

    dbg!(&counts);

    Ok(counts.get(&1).unwrap_or(&0) * counts.get(&3).unwrap_or(&0))
}

fn part2(input: &str) -> Result<i64> {
    let mut numbers = input
        .split("\n")
        .filter_map(|n| n.parse().ok())
        .collect::<Vec<i64>>();

    numbers.push(0);

    numbers.sort();
    // There are no duplicates

    let mut routes = HashMap::new();

    for (index, number) in numbers.iter().enumerate() {
        if *number == 0 {
            routes.insert(number, 1);
            continue;
        }

        let mut number_of_routes = 0;
        for possible_parent in numbers
            .iter()
            .skip(0.max((index as i32) - 3) as usize)
            .take(3)
        {
            if number - possible_parent <= 3 {
                number_of_routes += routes.get(possible_parent).unwrap_or(&0);
            }
        }

        routes.insert(number, number_of_routes);
    }

    Ok(routes[numbers.last().unwrap()])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "16
10
15
5
1
11
7
19
6
12
4
"
            )
            .unwrap(),
            7 * 5
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "16
10
15
5
1
11
7
19
6
12
4
"
            )
            .unwrap(),
            8
        )
    }
}
