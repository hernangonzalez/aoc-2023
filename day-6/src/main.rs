use anyhow::Context;
use anyhow::Result;
use std::str::FromStr;

fn main() {
    let input = r#"
    Time:        56     71     79     99
    Distance:   334   1135   1350   2430
    "#;
    let res1 = part_1::process(input).unwrap();
    println!("Part 1: {res1}");
}

#[allow(dead_code)]
#[derive(Debug)]
struct Race {
    time: u32,
    dist: u32,
}

struct Boat(u32);

impl Boat {
    fn distance_by(&self, t: u32) -> u32 {
        self.0 * t
    }
}

#[derive(Debug)]
struct Races(Vec<Race>);

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

        let parse_line = |l: &str| -> Result<Vec<u32>> {
            let nums = l.splitn(2, ':').last().context("Empty line")?;
            let vec: Vec<_> = nums
                .split_whitespace()
                .flat_map(|w| w.parse::<u32>())
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

mod part_1 {
    use super::*;

    pub fn process(text: &str) -> Result<u32> {
        let races = Races::from_str(text)?;
        races
            .into_iter()
            .map(|race| {
                (0..=race.time)
                    .map(|s| Boat(s).distance_by(race.time - s))
                    .filter(|d| *d > race.dist)
                    .count()
            })
            .map(|c| c as u32)
            .filter(|c| *c > 0)
            .reduce(|l, r| l * r)
            .context("No victories?")
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
}
