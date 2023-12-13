use anyhow::{Context, Result};
use num::integer;
use rayon::prelude::*;
use std::collections::HashMap;

fn main() {
    let text = std::fs::read_to_string("day-8/input.txt").unwrap();
    let res = part_1::process(&text);
    println!("Part 1: {res}");

    let res = part_2::process(&text);
    println!("Part 2: {res}");
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

impl Instruction {
    fn next(&self, d: &Direction) -> &Key {
        match d {
            Direction::Left => &self.0,
            Direction::Right => &self.1,
        }
    }
}

#[derive(Debug)]
struct Instructions(HashMap<Key, Instruction>);

impl<'a> Instructions {
    fn path(&'a self, key: &Key) -> Option<&'a Instruction> {
        self.0.get(key)
    }

    fn paths_ending(&'a self, key: char) -> Vec<Instruction> {
        self.0
            .iter()
            .filter_map(|(k, v)| if k.0.ends_with(key) { Some(v) } else { None })
            .cloned()
            .collect::<Vec<_>>()
    }
}

#[derive(Debug)]
struct Map {
    dir: Directions,
    inst: Instructions,
}

impl Map {
    fn navigate(&self, start: Key, end: Key) -> Result<u64> {
        let is_at_end = |k: &Key| -> bool { *k == end };
        let start = self.inst.path(&start).context("first instruction")?;
        self.navigate_end_count(start, &is_at_end)
    }

    fn navigate_end_count<F>(&self, start: &Instruction, eval: &F) -> Result<u64>
    where
        F: FnOnce(&Key) -> bool + Copy,
    {
        let mut dirs = self.dir.0.iter().cycle();
        let mut count = 0;
        let mut curr = start;
        loop {
            let d = dirs.next().context("next direction")?;
            let n = curr.next(d);
            curr = self.inst.path(n).context("next path")?;
            count += 1;
            if eval(n) {
                break;
            };
        }
        Ok(count)
    }

    fn navigate_ends(&self, start: char, end: char) -> Result<u64> {
        let is_at_end = |k: &Key| -> bool { k.0.ends_with(end) };
        let routes = self.inst.paths_ending(start);

        let counts = routes
            .par_iter()
            .flat_map(|i| self.navigate_end_count(i, &is_at_end));

        counts
            .reduce_with(|acc, i| integer::lcm(acc, i))
            .context("")
    }
}

mod parse {
    use super::*;
    use anyhow::Context;
    use regex::Regex;
    use std::str::FromStr;

    const INSTRUCTION_REGEX: &str =
        r"(?<key>[A-Z0-9]+) = \((?<left>[A-Z0-9]+), (?<right>[A-Z0-9]+)\)";

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

    pub fn process(s: &str) -> u64 {
        let lines = s.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let map = Map::build(lines).expect("map");
        map.navigate(Key("AAA".to_string()), Key("ZZZ".to_string()))
            .expect("path from AAA to ZZZ")
    }
}

mod part_2 {
    use crate::Map;

    pub fn process(s: &str) -> u64 {
        let lines = s.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let map = Map::build(lines).expect("map");
        map.navigate_ends('A', 'Z').expect("path from AAA to ZZZ")
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

    const SAMPLE_1_2: &str = r#"
    LLR

    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    "#;

    const SAMPLE_2_1: &str = r#"
    LR
    
    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)    
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE);
        assert_eq!(res, 2);
    }

    #[test]
    fn test_part_1_sample_2() {
        let res = part_1::process(SAMPLE_1_2);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_part_2_sample_1() {
        let res = part_2::process(SAMPLE_2_1);
        assert_eq!(res, 6);
    }
}
