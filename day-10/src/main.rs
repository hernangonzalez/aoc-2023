use color_eyre::Result;
use std::cmp::Ordering;
use strum::EnumIter;
use strum::IntoEnumIterator;
use Direction::*;

const START_TILE_ID: TileID = 'S';

fn main() -> Result<()> {
    let text = std::fs::read_to_string("day-10/input.txt")?;
    let res = part_1::process(&text);
    println!("Part 1: {res}");
    Ok(())
}

#[derive(Copy, Clone, Debug, EnumIter, Eq, PartialEq)]
enum Direction {
    North,
    South,
    West,
    East,
}

#[derive(Debug, Eq, PartialEq)]
struct Location(usize, usize);

type TileID = char;

#[derive(Debug, Eq, PartialEq)]
struct Tile(TileID, Location);

#[derive(Debug)]
struct TileMap(Vec<Vec<Tile>>);

#[derive(Clone, Debug)]
struct Trail<'a> {
    map: &'a TileMap,
    steps: Vec<&'a Tile>,
    tile: &'a Tile,
}

#[derive(Debug)]
struct Walker<'a> {
    map: &'a TileMap,
    trails: Vec<Trail<'a>>,
}

impl Location {
    fn mov(&self, d: Direction) -> Option<Location> {
        let mut row = self.0 as i64;
        let mut col = self.1 as i64;
        match d {
            North => row -= 1,
            South => row += 1,
            West => col -= 1,
            East => col += 1,
        }
        if row < 0 || col < 0 {
            None
        } else {
            Some(Self(row as usize, col as usize))
        }
    }

    fn cmp_row(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }

    fn cmp_col(&self, other: &Self) -> Ordering {
        self.1.cmp(&other.1)
    }
}

impl Tile {
    fn directions(&self) -> Vec<Direction> {
        let dirs: &[Direction] = match self.0 {
            '|' => &[North, South],
            '-' => &[East, West],
            'L' => &[North, East],
            'J' => &[North, West],
            '7' => &[South, West],
            'F' => &[South, East],
            'S' => &[North, South, West, East],
            _ => &[],
        };
        dirs.to_vec()
    }

    fn connects(&self, other: &Tile) -> bool {
        let lhs = self.directions();
        let rhs = other.directions();

        match self.1.cmp_row(&other.1) {
            Ordering::Less => lhs.contains(&South) && rhs.contains(&North),
            Ordering::Greater => lhs.contains(&North) && rhs.contains(&South),
            Ordering::Equal => match self.1.cmp_col(&other.1) {
                Ordering::Less => lhs.contains(&East) && rhs.contains(&West),
                Ordering::Greater => lhs.contains(&West) && rhs.contains(&East),
                Ordering::Equal => false, // Worlds collide!
            },
        }
    }
}

impl TileMap {
    fn tile_at(&self, loc: Location) -> Option<&Tile> {
        self.0.get(loc.0).map(|v| v.get(loc.1)).flatten()
    }

    fn find(&self, id: TileID) -> Option<&Tile> {
        self.0.iter().find_map(|r| r.iter().find(|t| t.0 == id))
    }

    fn connections(&self, t: &Tile) -> Vec<&Tile> {
        Direction::iter()
            .flat_map(|d| t.1.mov(d))
            .flat_map(|l| self.tile_at(l))
            .filter(|n| t.connects(n))
            .collect()
    }
}

impl<'a> Trail<'a> {
    fn new(map: &'a TileMap, start: &'a Tile) -> Trail<'a> {
        Self {
            map,
            steps: Vec::new(),
            tile: start,
        }
    }

    fn reached(&self, t: &Tile) -> bool {
        self.tile == t
    }

    fn count(&self) -> usize {
        self.steps.len()
    }

    // TODO: could be consumed and skip cloning?
    fn advance(&self) -> Vec<Trail<'a>> {
        let mut steps = self.steps.clone();
        steps.push(self.tile);
        let cons = self.map.connections(self.tile);
        cons.iter()
            .filter(|t| !(self.steps.last() == Some(t)))
            .map(|t| Self {
                map: self.map,
                steps: steps.clone(),
                tile: t,
            })
            .collect()
    }
}

impl<'a> Walker<'a> {
    fn new(map: &'a TileMap, start: &'a Tile) -> Walker<'a> {
        Self {
            map,
            trails: vec![Trail::new(map, start)],
        }
    }

    fn is_dry(&self) -> bool {
        self.trails.is_empty()
    }

    fn reached(&self, t: &Tile) -> Option<&'a Trail> {
        self.trails.iter().find(|trail| trail.reached(t))
    }

    fn advance(&self) -> Walker<'a> {
        let trails = self.trails.iter().flat_map(|t| t.advance()).collect();
        Self {
            map: self.map,
            trails,
        }
    }
}

mod parse {
    use super::*;

    impl From<&str> for TileMap {
        fn from(s: &str) -> Self {
            TileMap(
                s.lines()
                    .map(|l| l.trim())
                    .filter(|l| !l.is_empty())
                    .enumerate()
                    .map(|(row, l)| {
                        l.chars()
                            .enumerate()
                            .map(|(col, c)| Tile(c, Location(row, col)))
                            .collect()
                    })
                    .collect(),
            )
        }
    }
}

mod part_1 {
    use super::*;

    pub fn process(s: &str) -> usize {
        let tiles = TileMap::from(s);
        let start = tiles.find(START_TILE_ID).expect("Start tile 'S'");

        let mut walker = Walker::new(&tiles, &start);
        let mut trail: Option<usize> = None;
        while trail.is_none() && !walker.is_dry() {
            walker = walker.advance();
            trail = walker.reached(start).map(|t| t.count());
        }

        trail.unwrap_or(0) / 2
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    7-F7-
    .FJ|7
    SJLL7
    |F--J
    LJ.LJ
    "#;

    const SAMPLE_2: &str = r#"
    .....
    .S-7.
    .|.|.
    .L-J.
    .....
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE);
        assert_eq!(res, 8);

        let res = part_1::process(SAMPLE_2);
        assert_eq!(res, 4);
    }
}
