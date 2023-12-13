use anyhow::Result;
use itertools::Itertools;
use std::cmp::Ordering;
use strum::Display;

fn main() {
    let text = std::fs::read_to_string("day-7/input.txt").unwrap();
    let res = process(&text);

    #[cfg(not(feature = "joker"))]
    println!("Part 1: {res}");

    #[cfg(feature = "joker")]
    println!("Part 2: {res}");
}

#[cfg(feature = "joker")]
const CARD_ORDER: &str = "AKQT98765432J";

#[cfg(not(feature = "joker"))]
const CARD_ORDER: &str = "AKQJT98765432";

#[cfg(feature = "joker")]
const JOKER: Card = Card('J');

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Card(char);

impl Card {
    fn order(&self) -> usize {
        CARD_ORDER.find(self.0).expect("Not a legal card")
    }

    #[cfg(feature = "joker")]
    fn is_joker(&self) -> bool {
        *self == JOKER
    }
}

impl PartialOrd<Self> for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.order().cmp(&other.order())
    }
}

#[derive(Copy, Clone, Display, Debug, Eq, PartialEq, Ord, PartialOrd)]
enum Kind {
    Five,
    Four,
    Full,
    Three,
    TwoPair,
    Pair,
    High,
}

impl Kind {
    fn build(cards: &[Card; 5]) -> Self {
        let groups = cards.iter().sorted().rev().copied().group_by(|c| *c);
        let mut groups = groups
            .into_iter()
            .map(|(k, g)| (k, g.count()))
            .sorted_by(|lhs, rhs| rhs.1.cmp(&lhs.1))
            .collect_vec();

        #[cfg(feature = "joker")]
        if groups.len() > 1 {
            let find = groups
                .iter()
                .enumerate()
                .find(|(_, g)| g.0.is_joker())
                .map(|(i, g)| (i, g.1));

            if let Some((i, count)) = find {
                groups.remove(i);
                groups[0].1 += count;
            }
        }

        match groups[0].1 {
            5 => Kind::Five,
            4 => Kind::Four,
            3 => match groups.len() {
                2 => Kind::Full,
                _ => Kind::Three,
            },
            2 => match groups[1].1 {
                2 => Kind::TwoPair,
                _ => Kind::Pair,
            },
            _ => Kind::High,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Hand(Kind, [Card; 5]);

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
            .map(|(i, c)| c.cmp(&other.1[i]))
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
            let coll: [Card; 5] = s
                .chars()
                .map(Card)
                .collect_vec()
                .try_into()
                .expect("5 cards");
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

fn process(text: &str) -> usize {
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
    fn test_part_2() {
        let res = process(SAMPLE);
        assert_eq!(res, 5905);
    }
}
