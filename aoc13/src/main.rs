use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs::read_to_string,
};

use anyhow::{anyhow, bail, Result};
use maplit::{hashmap, hashset};

use utils::Vector;

fn main() -> Result<()> {
    let input = read_input()?;
    println!("part1: {}", part1(&input)?);
    println!("part2: {}", part2(&input)?);

    Ok(())
}

fn read_input() -> Result<String> {
    Ok(read_to_string("input.txt")?)
}

fn part1(input: &str) -> Result<u64> {
    let mut lines = input.split('\n');
    let ts: u64 = lines.next().unwrap().parse().unwrap();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .filter_map(|bus| bus.parse().ok())
        .collect::<Vec<u64>>();

    dbg!(busses
        .iter()
        .map(|id| (id, id - (ts % id)))
        .collect::<Vec<_>>());

    let (min_bus, rem) = busses
        .iter()
        .map(|id| (id, (id - (ts % id)) % id))
        .min_by_key(|(_, val)| *val)
        .unwrap();

    Ok(min_bus * rem)
}

#[derive(Debug)]
struct BusDescription {
    id: u64,
    delta: u64,
}

fn part2(input: &str) -> Result<u64> {
    let mut lines = input.split('\n');
    lines.next();
    let busses = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(index, bus)| {
            Some(BusDescription {
                id: bus.parse().ok()?,
                delta: index as u64,
            })
        })
        .collect::<Vec<_>>();

    // The first t where all busses we have looked at stops at the correct place
    let mut start = 0;
    // The interval between t's where the stops are correct
    let mut repeat: u64 = 1;

    for bus in busses {
        let mut t = start;
        dbg!(&bus);

        let busdelta = bus.delta % bus.id;
        dbg!(busdelta);

        loop {
            // if t % 10000 == 0 {
            //     println!("{}", t);
            // }

            let delta = (bus.id - (t % bus.id)) % bus.id;
            // dbg!(delta);
            // dbg!(t, delta);
            if delta == busdelta {
                start = t;
                repeat = repeat * bus.id;
                dbg!(start, repeat);
                break;
            }
            t += repeat;
        }
    }

    Ok(start)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "939
7,13,x,x,59,x,31,19"
            )
            .unwrap(),
            295
        )
    }

    #[test]
    fn test_part2() {
        assert_eq!(
            part2(
                "939
7,13,x,x,59,x,31,19"
            )
            .unwrap(),
            1068781
        )
    }

    #[test]
    fn test_part2_2() {
        assert_eq!(
            part2(
                "939
17,x,13,19"
            )
            .unwrap(),
            3417
        )
    }
    #[test]
    fn test_part2_3() {
        assert_eq!(
            part2(
                "939
67,7,59,61"
            )
            .unwrap(),
            754018
        )
    }

    #[test]
    fn test_part2_4() {
        assert_eq!(
            part2(
                "939
67,x,7,59,61"
            )
            .unwrap(),
            779210
        )
    }

    #[test]
    fn test_part2_5() {
        assert_eq!(
            part2(
                "939
1789,37,47,1889"
            )
            .unwrap(),
            1202161486
        )
    }
}
