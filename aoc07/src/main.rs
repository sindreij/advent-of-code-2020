use std::{
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

fn part1(input: &str) -> Result<usize> {
    let bags = parse(input)?;

    let mut direct_parent = HashMap::new();

    for (parent, content) in bags {
        for (child, count) in content {
            direct_parent
                .entry(child)
                .or_insert(HashSet::new())
                .insert(parent.clone());
        }
    }

    let mut has_checked = HashSet::new();
    let mut to_check = vec!["shiny gold"];

    while let Some(name) = to_check.pop() {
        dbg!(name);
        has_checked.insert(name.to_owned());
        if let Some(direct_parents) = direct_parent.get(name) {
            for can_contain in direct_parents {
                if !has_checked.contains(can_contain) {
                    to_check.push(can_contain);
                }
            }
        }
    }

    Ok(has_checked.len() - 1)
}

fn part2(input: &str) -> Result<usize> {
    Ok(12)
}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    Literal(String),
    Number(u32),
    Comma,
    Dot,
    Bag,
    Contain,
}

fn parse(input: &str) -> Result<HashMap<String, HashMap<String, u32>>> {
    let mut tokens = vec![];

    let mut chars = input.chars().peekable();

    loop {
        match chars.next() {
            None => break,
            Some(ch) if ch.is_whitespace() => {}
            Some(ch) if ch.is_numeric() => {
                let mut number = ch.to_string();
                while let Some(ch) = chars.peek() {
                    if ch.is_numeric() {
                        number.push(*ch);
                        chars.next();
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Number(number.parse().unwrap()))
            }
            Some(ch) if ch.is_alphabetic() => {
                let mut literal = ch.to_string();
                while let Some(ch) = chars.peek() {
                    if ch.is_alphanumeric() {
                        literal.push(*ch);
                        chars.next();
                    } else {
                        break;
                    }
                }

                tokens.push(match &*literal {
                    "contain" => Token::Contain,
                    "bag" => Token::Bag,
                    "bags" => Token::Bag,
                    something => Token::Literal(something.to_owned()),
                })
            }
            Some('.') => tokens.push(Token::Dot),
            Some(',') => tokens.push(Token::Comma),
            Some(thing_else) => bail!("Unkown character: {:?}", thing_else),
        }
    }

    let mut result = HashMap::new();
    let mut tokens = tokens.into_iter().peekable();

    while let Some(token) = tokens.next() {
        let mut take_next = || {
            tokens
                .next()
                .ok_or_else(|| anyhow!("Expected token, got EOF"))
        };

        let first_name = match token {
            Token::Literal(name) => name,
            _ => bail!("Expected literal, got {:?}", token),
        };
        let second_name = match take_next()? {
            Token::Literal(name) => name,
            something => bail!("Expected literal, got {:?}", something),
        };

        match take_next()? {
            Token::Bag => {}
            something => bail!("Expected bag, got {:?}", something),
        }

        match take_next()? {
            Token::Contain => {}
            something => bail!("Expected contain, got {:?}", something),
        }

        let mut content = HashMap::new();

        loop {
            let count = match take_next()? {
                Token::Number(count) => count,
                Token::Literal(val) if val == "no" => {
                    // check for "no other bags"
                    match take_next()? {
                        Token::Literal(val) if val == "other" => {}
                        something => bail!("Expected \"other\", got {:?}", something),
                    }
                    match take_next()? {
                        Token::Bag => {}
                        something => bail!("Expected bag, got {:?}", something),
                    }
                    match take_next()? {
                        Token::Dot => {}
                        something => bail!("Expected dot, got {:?}", something),
                    }
                    break;
                }
                something => bail!("Expected number, got {:?}", something),
            };

            let first_name = match take_next()? {
                Token::Literal(name) => name,
                something => bail!("Expected literal, got {:?}", something),
            };
            let second_name = match take_next()? {
                Token::Literal(name) => name,
                something => bail!("Expected literal, got {:?}", something),
            };

            content.insert(format!("{} {}", first_name, second_name), count);

            match take_next()? {
                Token::Bag => {}
                something => bail!("Expected contain, got {:?}", something),
            }

            match take_next()? {
                Token::Comma => {} // continue
                Token::Dot => break,
                something => bail!("Expected comma or dot, got {:?}", something),
            }
        }

        result.insert(format!("{} {}", first_name, second_name), content);
    }

    Ok(result)
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use super::*;

    fn s(s: &str) -> String {
        s.to_owned()
    }

    #[test]
    fn test_parse() {
        let output = parse(
            "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
faded blue bags contain no other bags.
",
        )
        .unwrap();

        assert_eq!(
            output,
            hashmap! {
                s("light red") => hashmap![s("bright white") => 1, s("muted yellow") => 2],
                s("dark orange") => hashmap![s("bright white") => 3, s("muted yellow") => 4],
                s("faded blue") => hashmap![],
            }
        )
    }

    #[test]
    fn test_part1() {
        assert_eq!(
            part1(
                "light red bags contain 1 bright white bag, 2 muted yellow bags.
        dark orange bags contain 3 bright white bags, 4 muted yellow bags.
        bright white bags contain 1 shiny gold bag.
        muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
        shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
        dark olive bags contain 3 faded blue bags, 4 dotted black bags.
        vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
        faded blue bags contain no other bags.
        dotted black bags contain no other bags.
        "
            )
            .unwrap(),
            4
        );
    }
}
