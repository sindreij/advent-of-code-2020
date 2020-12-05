use std::fs::read_to_string;

use anyhow::Result;

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
    let hill = Hill::new(input);

    Ok(hill.calc_slope(3, 1))
}

fn part2(input: &str) -> Result<usize> {
    let hill = Hill::new(input);

    Ok(hill.calc_slope(1, 1)
        * hill.calc_slope(3, 1)
        * hill.calc_slope(5, 1)
        * hill.calc_slope(7, 1)
        * hill.calc_slope(1, 2))
}

struct Hill {
    data: Vec<Vec<char>>,
}

impl Hill {
    fn new(input: &str) -> Self {
        Hill {
            data: input
                .split("\n")
                .filter(|line| !line.is_empty())
                .map(|line| line.chars().collect::<Vec<_>>())
                .collect::<Vec<_>>(),
        }
    }

    fn calc_slope(&self, right: usize, down: usize) -> usize {
        self.data
            .iter()
            .step_by(down)
            .enumerate()
            .filter(|(y, line)| {
                let x = (y * right) % line.len();
                line[x] == '#'
            })
            .count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_name() {
        let output = part1(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        )
        .unwrap();

        assert_eq!(output, 7);
    }

    #[test]
    fn test_part2() {
        let output = part2(
            "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#",
        )
        .unwrap();

        assert_eq!(output, 336);
    }
}
