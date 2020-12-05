use std::{
    collections::{HashMap, HashSet},
    fs::read_to_string,
};

use anyhow::{anyhow, bail, ensure, Result};
use maplit::{hashmap, hashset};
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
    let passports = parse(input)?;

    let required = hashset! {"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"};

    Ok(passports
        .into_iter()
        .filter(|passport| {
            passport
                .keys()
                .map(|s| &**s)
                .collect::<HashSet<_>>()
                .is_superset(&required)
        })
        .count())
}

fn part2(input: &str) -> Result<usize> {
    let passports = parse(input)?;

    Ok(passports
        .into_iter()
        .filter(|passport| is_valid_passport(passport).is_ok())
        .count())
}

fn is_valid_passport(passport: &HashMap<String, String>) -> Result<()> {
    let byr = passport
        .get("byr")
        .ok_or_else(|| anyhow!("No byr"))?
        .parse::<i32>()?;

    ensure!(byr >= 1920 && byr <= 2002, "Invalid byr");

    let iyr = passport
        .get("iyr")
        .ok_or_else(|| anyhow!("No iyr"))?
        .parse::<i32>()?;

    ensure!(iyr >= 2010 && iyr <= 2020, "Invalid iyr");

    let eyr = passport
        .get("eyr")
        .ok_or_else(|| anyhow!("No eyr"))?
        .parse::<i32>()?;

    ensure!(eyr >= 2020 && eyr <= 2030, "Invalid eyr");

    let hgt = passport.get("hgt").ok_or_else(|| anyhow!("No hgt"))?;
    if hgt.ends_with("cm") {
        let hgt = (&hgt[0..hgt.len() - 2]).parse::<i32>()?;
        ensure!(hgt >= 150 && hgt <= 193, "Invalid hgt");
    } else if hgt.ends_with("in") {
        let hgt = (&hgt[0..hgt.len() - 2]).parse::<i32>()?;
        ensure!(hgt >= 59 && hgt <= 76, "Invalid hgt");
    } else {
        bail!("Invalid hgt, {}", hgt);
    }

    let hcl = passport.get("hcl").ok_or_else(|| anyhow!("No hcl"))?;
    let hcl_re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    ensure!(hcl_re.is_match(hcl), "HCL does not match regex");

    let ecl = passport.get("ecl").ok_or_else(|| anyhow!("No ecl"))?;
    ensure!(
        ecl == "amb"
            || ecl == "blu"
            || ecl == "brn"
            || ecl == "gry"
            || ecl == "grn"
            || ecl == "hzl"
            || ecl == "oth",
        "Invalid eye color"
    );

    let pid = passport.get("pid").ok_or_else(|| anyhow!("No pid"))?;
    let pid_re = Regex::new(r"^\d{9}$").unwrap();

    ensure!(pid_re.is_match(pid), "Pid does not match regex");

    Ok(())
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Value(String),
    Colon,
    TwoNewlines,
}

fn parse(input: &str) -> Result<Vec<HashMap<String, String>>> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();

    loop {
        match chars.next() {
            None => break,
            Some(':') => tokens.push(Token::Colon),
            Some('\n') if chars.peek() == Some(&'\n') => tokens.push(Token::TwoNewlines),
            Some(chr) if chr.is_whitespace() => { /* Ignore whitespace */ }
            Some(chr) => {
                let mut identifier = chr.to_string();
                while let Some(chr) = chars.peek() {
                    if *chr != ':' && !chr.is_whitespace() {
                        identifier.push(*chr);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Value(identifier))
            }
        }
    }

    let mut passports = vec![];
    let mut tokens = tokens.into_iter().peekable();

    while tokens.peek().is_some() {
        let mut passport = HashMap::new();

        loop {
            match tokens.next() {
                None => break,
                Some(Token::TwoNewlines) => break,
                Some(Token::Value(key)) => {
                    ensure!(tokens.next() == Some(Token::Colon), "Colon expected");
                    if let Some(Token::Value(value)) = tokens.next() {
                        passport.insert(key, value);
                    } else {
                        bail!("Value expected");
                    }
                }
                Some(invalid) => bail!("Got token {:?}, expected two newlines or value", invalid),
            }
        }

        passports.push(passport);
    }

    Ok(passports)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn s(input: &str) -> String {
        input.to_owned()
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part1(
                "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in",
            )
            .unwrap(),
            2
        )
    }

    #[test]
    fn test_parse() {
        let result = parse(
            "ecl:gry pid:860033327

iyr:2013 ecl:amb
",
        )
        .unwrap();

        assert_eq!(
            result,
            vec![
                hashmap! {
                    s("ecl") => s("gry"),
                    s("pid") => s("860033327"),
                },
                hashmap! {
                    s("iyr") => s("2013"),
                    s("ecl") => s("amb"),
                },
            ]
        );
    }
}
