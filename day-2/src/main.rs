use anyhow::{Context, Result};
use std::ops::Add;
use std::str::FromStr;

fn main() -> Result<()> {
    let text = std::fs::read_to_string("input.txt")?;
    let limit = Config {
        red: 12,
        green: 13,
        blue: 14,
    };
    let res = part_1::process(&text, limit);
    println!("Result: {res}");
    Ok(())
}

#[derive(Default, Debug)]
struct DiceSet {
    red: u32,
    green: u32,
    blue: u32,
}

type Config = DiceSet;

#[derive(Debug)]
struct Game {
    id: u32,
    set: Vec<DiceSet>,
}

#[derive(Debug)]
enum Dice {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct DiceRoll(Dice, u32);

mod parse {
    use super::*;

    impl FromStr for Dice {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "red" => Ok(Self::Red),
                "green" => Ok(Self::Green),
                "blue" => Ok(Self::Blue),
                _ => Err(anyhow::anyhow!("No dice")),
            }
        }
    }

    impl FromStr for DiceRoll {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            let mut iter = s.trim().splitn(2, " ");
            let lhs = iter.next().context("missing count")?;
            let rhs = iter.next().context("missing color")?;
            Ok(Self(rhs.parse()?, lhs.parse()?))
        }
    }

    impl FromStr for DiceSet {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let sets = s.split(',').map(|s| s.parse::<DiceRoll>().unwrap());
            let sum = sets.fold(Self::default(), |acc, x| acc + x);
            Ok(sum)
        }
    }

    impl FromStr for Game {
        type Err = anyhow::Error;

        fn from_str(s: &str) -> Result<Self> {
            let mut split = s.splitn(2, ':');
            let id = split.next().context("missing game")?;
            let set = split.next().context("missing set")?;

            let id: u32 = id.split(' ').last().context("Missing ID")?.parse()?;
            let set: Result<Vec<DiceSet>, Self::Err> =
                set.split(';').map(|s| s.parse::<DiceSet>()).collect();

            Ok(Self { id, set: set? })
        }
    }
}

impl Add<DiceRoll> for DiceSet {
    type Output = Self;

    fn add(self, rhs: DiceRoll) -> Self::Output {
        let mut acc = self;
        match rhs.0 {
            Dice::Red => acc.red += rhs.1,
            Dice::Green => acc.green += rhs.1,
            Dice::Blue => acc.blue += rhs.1,
        };
        acc
    }
}

impl DiceSet {
    fn contains(&self, other: &DiceSet) -> bool {
        other.red <= self.red && other.green <= self.green && other.blue <= self.blue
    }
}

impl Game {
    fn is_valid(&self, limit: &Config) -> bool {
        self.set.iter().all(|s| limit.contains(s))
    }
}

mod part_1 {
    use crate::{Config, Game};

    pub fn process(text: &str, config: Config) -> u32 {
        text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Game>().unwrap())
            .filter(|l| l.is_valid(&config))
            .map(|g| g.id)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = r#"
    Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
    Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
    Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
    Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
    Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
    "#;

    #[test]
    fn test_sample_1() {
        let cfg = Config {
            red: 12,
            green: 13,
            blue: 14,
        };
        let res = part_1::process(SAMPLE_1, cfg);
        assert_eq!(res, 8)
    }
}
