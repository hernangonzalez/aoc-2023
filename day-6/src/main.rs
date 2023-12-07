use anyhow::Context;
use anyhow::Result;
use std::str::FromStr;

fn main() {
    let input = r#"
    Time:        56     71     79     99
    Distance:   334   1135   1350   2430
    "#;
    let res1 = part_1::process(input).unwrap();
    let res2 = part_2::process(input).unwrap();
    println!("Part 1: {res1}");
    println!("Part 2: {res2}");
}

#[allow(dead_code)]
#[derive(Debug)]
struct Race {
    time: u64,
    dist: u64,
}
#[derive(Debug)]
struct Races(Vec<Race>);

struct Boat(u64);

impl Boat {
    fn distance_by(&self, t: u64) -> u64 {
        self.0 * t
    }
}

impl IntoIterator for Races {
    type Item = Race;
    type IntoIter = std::vec::IntoIter<Race>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl FromStr for Races {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
        let times = lines.next().context("missing times")?;
        let dists = lines.next().context("missing distances")?;

        let parse_line = |l: &str| -> Result<Vec<u64>> {
            let nums = l.splitn(2, ':').last().context("Empty line")?;
            let vec: Vec<_> = nums
                .split_whitespace()
                .flat_map(|w| w.parse::<u64>())
                .collect();
            Ok(vec)
        };

        let times = parse_line(times)?;
        let dists = parse_line(dists)?;

        Ok(Races(
            times
                .iter()
                .copied()
                .zip(dists)
                .map(|(time, dist)| Race { time, dist })
                .collect(),
        ))
    }
}

impl Race {
    fn possible_victories(&self) -> u32 {
        (0..=self.time)
            .map(|s| Boat(s).distance_by(self.time - s))
            .filter(|d| *d > self.dist)
            .count() as u32
    }
}

mod part_1 {
    use super::*;

    pub fn process(text: &str) -> Result<u32> {
        let races = Races::from_str(text)?;
        races
            .into_iter()
            .map(|race| race.possible_victories())
            .filter(|c| *c > 0)
            .reduce(|l, r| l * r)
            .context("No victories?")
    }
}

mod part_2 {
    use super::*;

    pub fn process(text: &str) -> Result<u32> {
        let text = text.replace(" ", "");
        let races = Races::from_str(&text)?;
        Ok(races
            .into_iter()
            .map(|race| race.possible_victories())
            .sum())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    Time:      7  15   30
    Distance:  9  40  200
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE).unwrap();
        assert_eq!(res, 288);
    }

    #[test]
    fn test_part_2() {
        let res = part_2::process(SAMPLE).unwrap();
        assert_eq!(res, 71503);
    }
}
