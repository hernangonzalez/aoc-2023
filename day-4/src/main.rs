use anyhow::{Context, Result};
use std::str::FromStr;

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let res1 = part_1::process(&text);
    println!("Part 1: {res1}");
}

#[allow(dead_code)]
struct Card {
    id: u32,
    win: Vec<u32>,
    pick: Vec<u32>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut iter = s.splitn(2, ':');
        let id: u32 = iter
            .next()
            .context("missing card header")?
            .split_whitespace()
            .last()
            .context("missing id")?
            .parse()?;
        let nums = iter.next().context("missing numbers")?;
        let mut iter = nums.splitn(2, "|");

        let to_numbers = |text: &str| -> Vec<u32> {
            text.trim()
                .split_whitespace()
                .flat_map(|s| s.parse())
                .collect()
        };

        let win = to_numbers(iter.next().context("missing winners")?);
        let pick = to_numbers(iter.next().context("missing picked numbers")?);

        Ok(Self { id, win, pick })
    }
}

impl Card {
    fn matches(&self) -> Vec<u32> {
        self.pick
            .iter()
            .filter(|p| self.win.contains(p))
            .copied()
            .collect()
    }

    fn match_count(&self) -> u32 {
        self.matches().len() as u32
    }
}

fn parse(text: &str) -> Vec<Card> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .map(|l| l.parse())
        .flatten()
        .collect()
}

mod part_1 {
    use crate::parse;

    pub fn process(text: &str) -> u32 {
        let cards = parse(text);
        cards
            .iter()
            .map(|c| {
                let count = c.match_count();
                match count {
                    0 | 1 => count,
                    _ => (0..count - 1).fold(1, |a, _| a * 2),
                }
            })
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use crate::part_1::process;

    const SAMPLE: &str = r#"
    Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
    Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
    Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
    Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
    Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
    Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
    "#;

    #[test]
    fn test_part_1() {
        let res = process(SAMPLE);
        assert_eq!(res, 13);
    }
}
