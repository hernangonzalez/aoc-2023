use anyhow::{Context, Result};
use std::collections::HashMap;

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let res = part_1::process(&text);
    println!("Part 1: {res}");
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Directions(Vec<Direction>);

#[derive(Clone, Debug, Hash, Eq, PartialEq)]
struct Key(String);

#[derive(Clone, Debug)]
struct Instruction(Key, Key);

#[derive(Debug)]
struct Instructions(HashMap<Key, Instruction>);

impl<'a> Instructions {
    fn path(&'a self, key: &Key) -> Option<&'a Instruction> {
        self.0.get(key)
    }
}

#[derive(Debug)]
struct Map {
    dir: Directions,
    inst: Instructions,
}

impl Map {
    fn route(&self, start: Key, end: Key) -> Result<Vec<Instruction>> {
        let mut dirs = self.dir.0.iter().cycle();
        anyhow::ensure!(start != end);

        let mut vec = Vec::new();
        let mut pivot = &start;
        while let Some(path) = self.inst.path(&pivot) {
            pivot = match dirs.next().context("next direction")? {
                Direction::Left => &path.0,
                Direction::Right => &path.1,
            };

            vec.push((*path).clone());
            if *pivot == end {
                break;
            }
        }

        Ok(vec)
    }
}

mod parse {
    use super::*;
    use anyhow::Context;
    use regex::Regex;
    use std::str::FromStr;

    const INSTRUCTION_REGEX: &str = r"(?<key>[A-Z]+) = \((?<left>[A-Z]+), (?<right>[A-Z]+)\)";

    impl TryFrom<char> for Direction {
        type Error = anyhow::Error;

        fn try_from(c: char) -> Result<Self> {
            match c {
                'L' => Ok(Self::Left),
                'R' => Ok(Self::Right),
                _ => Err(anyhow::anyhow!("Not a direction")),
            }
        }
    }

    impl FromStr for Directions {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self> {
            Ok(Self(
                s.chars().flat_map(|c| c.try_into()).collect::<Vec<_>>(),
            ))
        }
    }

    impl Instructions {
        fn build<'l, L>(lines: &mut L) -> Result<Instructions>
        where
            L: Iterator<Item = &'l str>,
        {
            let regex = Regex::new(INSTRUCTION_REGEX)?;
            let mut map = HashMap::new();
            for line in lines {
                let caps = regex.captures(line).context("regex captures")?;
                let key = Key((&caps["key"]).to_string());
                let lhs = Key((&caps["left"]).to_string());
                let rhs = Key((&caps["right"]).to_string());
                map.insert(key, Instruction(lhs, rhs));
            }
            Ok(Self(map))
        }
    }

    impl Map {
        pub fn build<'l, L>(lines: L) -> Result<Self>
        where
            L: Iterator<Item = &'l str>,
        {
            let mut lines = lines;
            let dir: Directions = lines.next().context("directions")?.parse()?;
            anyhow::ensure!(!dir.0.is_empty());

            let inst = Instructions::build(&mut lines)?;
            anyhow::ensure!(lines.next() == None);

            Ok(Self { dir, inst })
        }
    }
}

mod part_1 {
    use crate::{Key, Map};

    pub fn process(s: &str) -> usize {
        let lines = s.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let map = Map::build(lines).expect("map");
        let path = map
            .route(Key("AAA".to_string()), Key("ZZZ".to_string()))
            .expect("path from AAA to ZZZ");
        path.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    RL

    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    "#;

    const SAMPLE_2: &str = r#"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_1_sample_2() {
        let res = part_1::process(SAMPLE_2);
        assert_eq!(res, 6);
    }
}
