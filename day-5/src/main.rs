use anyhow::{Context, Result};
use itertools::Itertools;
use regex::Regex;
use std::ops::Range;
use std::str::FromStr;
use strum::EnumString;

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let res1 = part_1::process(&text).unwrap();
    println!("Part 1: {res1}");
    let res2 = part_2::process(&text).unwrap();
    println!("Part 2: {res2}");
}

type ID = u64;

#[derive(Debug)]
enum Seed {
    Single(ID),
    Group(Range<ID>),
}

#[derive(EnumString, Clone, Copy, Debug, PartialEq)]
#[strum(serialize_all = "lowercase")]
enum Category {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

#[allow(dead_code)]
struct RangeMap {
    org: Range<ID>,
    dst: Range<ID>,
}

#[allow(dead_code)]
struct ConversionMap {
    src: Category,
    dst: Category,
    ranges: Vec<RangeMap>,
}

struct ConversionMaps(Vec<ConversionMap>);

impl ConversionMaps {
    fn map_from(&self, cat: Category) -> Option<&ConversionMap> {
        self.0.iter().find(|m| m.src == cat)
    }

    fn seed_location(&self, id: ID) -> ID {
        let mut origin = Category::Seed;
        let mut id = id;
        while let Some(map) = self.map_from(origin) {
            id = map.resolve(id);
            if map.dst == Category::Location {
                break;
            }
            origin = map.dst;
        }
        id
    }

    fn min_seed_location(&self, range: &Range<ID>) -> ID {
        range
            .clone()
            .map(|id| self.seed_location(id))
            .min()
            .unwrap()
    }
}

impl ConversionMap {
    fn new(src: Category, dst: Category) -> Self {
        Self {
            src,
            dst,
            ranges: Vec::new(),
        }
    }

    fn resolve(&self, id: ID) -> ID {
        let Some(range) = self.ranges.iter().find(|r| r.org.contains(&id)) else {
            return id;
        };

        let diff = id.abs_diff(range.org.start);
        range.dst.start + diff
    }
}

impl FromStr for Seed {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let id = s.parse()?;
        Ok(Self::Single(id))
    }
}

trait SeedParser {
    fn parse_seeds(&self, line: &str) -> Result<Vec<Seed>>;
}

struct SingleSeedParser;

impl SeedParser for SingleSeedParser {
    fn parse_seeds(&self, line: &str) -> Result<Vec<Seed>> {
        let ids = line.split(':').last().context("missing ids")?;
        ids.split_whitespace().map(|s| s.parse::<Seed>()).collect()
    }
}

struct RangeSeedParser;

impl SeedParser for RangeSeedParser {
    fn parse_seeds(&self, line: &str) -> Result<Vec<Seed>> {
        let ids = line.split(':').last().context("missing ids")?;
        let ids = ids
            .split_whitespace()
            .flat_map(|s| s.parse::<ID>())
            .batching(|it| match it.next() {
                None => None,
                Some(x) => match it.next() {
                    None => None,
                    Some(y) => Some((x, y)),
                },
            });
        Ok(ids
            .map(|(id, len)| id..id + len)
            .map(|r| Seed::Group(r))
            .collect())
    }
}

fn parse_maps<'a>(lines: impl Iterator<Item = &'a str>) -> Result<ConversionMaps> {
    let cat_rgx = Regex::new("(?<src>[a-z]+)-to-(?<dst>[a-z]+)")?;
    let range_rgx = Regex::new("(?<dst>[0-9]+) (?<src>[0-9]+) (?<len>[0-9]+)")?;

    let mut res = Vec::new();
    let mut conv: Option<ConversionMap> = None;
    for l in lines {
        if let Some(cats) = cat_rgx.captures(l) {
            let src = (&cats["src"]).parse::<Category>()?;
            let dst = (&cats["dst"]).parse::<Category>()?;
            if let Some(active) = conv {
                res.push(active);
            }
            conv = Some(ConversionMap::new(src, dst));
        }

        if let Some(range) = range_rgx.captures(l) {
            let src = (&range["src"]).parse::<ID>()?;
            let dst = (&range["dst"]).parse::<ID>()?;
            let len = (&range["len"]).parse::<ID>()?;
            let org = src..src + len;
            let dst = dst..dst + len;
            let map = RangeMap { org, dst };
            conv.as_mut()
                .context("Missing categories")?
                .ranges
                .push(map);
        }
    }

    if let Some(active) = conv {
        res.push(active);
    }

    Ok(ConversionMaps(res))
}

fn parse(text: &str, parser: impl SeedParser) -> Result<(Vec<Seed>, ConversionMaps)> {
    let mut lines = text.lines().map(|l| l.trim()).filter(|l| !l.is_empty());
    let seeds = lines.next().context("missing seeds")?;
    let seeds = parser.parse_seeds(seeds)?;
    let maps = parse_maps(lines)?;
    Ok((seeds, maps))
}

mod part_1 {
    use super::*;

    pub fn process(text: &str) -> Result<ID> {
        let (seeds, maps) = parse(text, SingleSeedParser)?;
        seeds
            .iter()
            .flat_map(|s| match s {
                Seed::Single(id) => Some(*id),
                _ => None,
            })
            .map(|s| maps.seed_location(s))
            .min()
            .context("Could not map locations")
    }
}

mod part_2 {
    use super::*;

    pub fn process(text: &str) -> Result<ID> {
        let (seeds, maps) = parse(text, RangeSeedParser)?;
        seeds
            .iter()
            .flat_map(|s| match s {
                Seed::Group(g) => Some(g),
                _ => None,
            })
            .map(|s| maps.min_seed_location(s))
            .min()
            .context("Could not map locations")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    seeds: 79 14 55 13

    seed-to-soil map:
    50 98 2
    52 50 48

    soil-to-fertilizer map:
    0 15 37
    37 52 2
    39 0 15

    fertilizer-to-water map:
    49 53 8
    0 11 42
    42 0 7
    57 7 4

    water-to-light map:
    88 18 7
    18 25 70

    light-to-temperature map:
    45 77 23
    81 45 19
    68 64 13

    temperature-to-humidity map:
    0 69 1
    1 0 69

    humidity-to-location map:
    60 56 37
    56 93 4
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE).unwrap();
        assert_eq!(res, 35);
    }

    #[test]
    fn test_part_2() {
        let res = part_2::process(SAMPLE).unwrap();
        assert_eq!(res, 46);
    }
}
