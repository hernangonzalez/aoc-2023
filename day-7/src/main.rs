use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use std::fmt::{Debug, Formatter};
use strum::{Display, EnumString};

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let res1 = part_1::process(&text);
    println!("Part 1: {res1}");
}

#[derive(EnumString, Display, Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Card {
    A = 14,
    K = 13,
    Q = 12,
    J = 11,
    T = 10,
    #[strum(serialize = "9")]
    N9 = 9,
    #[strum(serialize = "8")]
    N8 = 8,
    #[strum(serialize = "7")]
    N7 = 7,
    #[strum(serialize = "6")]
    N6 = 6,
    #[strum(serialize = "5")]
    N5 = 5,
    #[strum(serialize = "4")]
    N4 = 4,
    #[strum(serialize = "3")]
    N3 = 3,
    #[strum(serialize = "2")]
    N2 = 2,
}

#[derive(Copy, Clone, Display, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Kind {
    Five = 0,
    Four = 1,
    Full = 2,
    Three = 3,
    TwoPair = 4,
    Pair = 5,
    High = 6,
}

impl Kind {
    fn build(cards: &[Card; 5]) -> Self {
        let groups = cards.iter().sorted().rev().copied().group_by(|c| *c);
        let groups = groups
            .into_iter()
            .map(|(k, g)| (k, g.collect_vec()))
            .sorted_by(|lhs, rhs| rhs.1.len().cmp(&lhs.1.len()))
            .collect_vec();

        match groups[0].1.len() {
            5 => Kind::Five,
            4 => Kind::Four,
            3 => match groups.len() {
                2 => Kind::Full,
                _ => Kind::Three,
            },
            2 => match groups[1].1.len() {
                2 => Kind::TwoPair,
                _ => Kind::Pair,
            },
            _ => Kind::High,
        }
    }
}

#[derive(Eq, PartialEq)]
struct Hand(Kind, [Card; 5]);

impl Debug for Hand {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let cards = self.1.map(|c| c.to_string()).join("");
        let s = format!("{} - {}", &self.0.to_string(), &cards);
        f.write_str(&s.to_string())
    }
}

impl PartialOrd<Self> for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let kind_cmp = self.0.cmp(&other.0);
        if kind_cmp != Ordering::Equal {
            return kind_cmp;
        }

        self.1
            .iter()
            .enumerate()
            .map(|(i, c)| c.cmp(&other.1[i]).reverse())
            .find(|o| *o != Ordering::Equal)
            .unwrap_or(Ordering::Equal)
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
struct Game(Hand, usize);

#[derive(Debug)]
struct Games(Vec<Game>);

mod parse {
    use super::*;
    use anyhow::Context;
    use std::str::FromStr;

    impl FromStr for Hand {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
            anyhow::ensure!(s.len() == 5);
            let mut coll = [Card::A; 5];
            for i in 0..s.len() {
                let char = &s[i..i + 1];
                coll[i] = char.parse()?;
            }
            let kind = Kind::build(&coll);
            Ok(Self(kind, coll))
        }
    }

    impl FromStr for Game {
        type Err = anyhow::Error;
        fn from_str(s: &str) -> Result<Self> {
            let mut iter = s.split_whitespace();
            let cards = iter.next().context("Cards")?;
            let bet = iter.next().context("Bet")?;
            anyhow::ensure!(iter.next() == None);
            Ok(Self(cards.parse()?, bet.parse()?))
        }
    }

    pub fn games(text: &str) -> Result<Games> {
        let games = text
            .lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| l.parse::<Game>().unwrap())
            .collect_vec();
        Ok(Games(games))
    }
}

mod part_1 {
    use crate::parse;
    use itertools::Itertools;

    pub fn process(text: &str) -> usize {
        let games = parse::games(text).unwrap();
        games
            .0
            .iter()
            .sorted()
            .rev()
            .enumerate()
            .map(|(i, g)| (i + 1) * g.1)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    32T3K 765
    T55J5 684
    KK677 28
    KTJJT 220
    QQQJA 483
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE);
        assert_eq!(res, 6440);
    }
}
