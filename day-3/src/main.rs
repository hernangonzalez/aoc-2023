use core::ops::Range;
use std::fmt::{Display, Formatter};

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let r1 = part_1::process(&text);
    println!("Part 1: {r1}");
}

#[derive(Copy, Clone, Debug)]
struct Loc {
    x: usize,
    y: usize,
}

impl Loc {
    fn new(y: usize) -> Self {
        Self { x: 0, y }
    }
}

#[derive(Clone, Copy, Debug)]
struct Element<Inner: Clone + Copy> {
    val: Inner,
    loc: Loc,
}

impl<T: Clone + Copy + Display> Display for Element<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.val)
    }
}

impl Element<u32> {
    fn digits(&self) -> usize {
        let mut divisor = 1;
        let mut count = 1;
        while self.val >= divisor * 10 {
            divisor *= 10;
            count += 1;
        }
        count
    }

    fn range(&self) -> Range<usize> {
        let mut range = self.loc.x..self.loc.x + self.digits();
        if range.start > 0 {
            range.start -= 1;
        }
        range.end += 1;
        range
    }

    fn is_around(&self, sym: &Symbol) -> bool {
        let range_x = self.range();
        let dy = self.loc.y.abs_diff(sym.loc.y);
        dy <= 1 && range_x.contains(&sym.loc.x)
    }
}

type Num = Element<u32>;
type Symbol = Element<char>;

#[derive(Debug)]
enum Item {
    Num(Num),
    Symbol(Symbol),
}

impl Item {
    fn num(&self) -> Option<Num> {
        match self {
            Self::Num(n) => Some(*n),
            _ => None,
        }
    }

    fn sym(&self) -> Option<Symbol> {
        match self {
            Self::Symbol(s) => Some(*s),
            _ => None,
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

fn parse_line(y: usize, line: &str) -> Vec<Item> {
    let mut coll = Vec::new();
    let mut num = 0;
    let mut loc = Loc::new(y);

    for (x, c) in line.chars().enumerate() {
        if let Some(d) = c.to_digit(10) {
            if num == 0 {
                loc.x = x;
            }
            num *= 10;
            num += d;
            continue;
        }

        if num != 0 {
            let item = Item::Num(Num { val: num, loc });
            coll.push(item);
            loc = Loc::new(y);
            num = 0;
        }

        if c != '.' {
            loc.x = x;
            let item = Item::Symbol(Symbol { val: c, loc });
            coll.push(item);
            loc = Loc::new(y);
        }
    }

    if num != 0 {
        let item = Item::Num(Num { val: num, loc });
        coll.push(item);
    }

    coll
}

fn parse(text: &str) -> Vec<Item> {
    text.lines()
        .map(|l| l.trim())
        .filter(|l| !l.is_empty())
        .enumerate()
        .flat_map(|(i, l)| parse_line(i, l))
        .collect()
}

mod part_1 {
    use crate::parse;

    pub fn process(text: &str) -> u32 {
        let items = parse(text);
        let syms = items.iter().filter_map(|e| e.sym());
        let nums = items.iter().filter_map(|e| e.num());
        nums.filter(|n| syms.clone().any(|s| n.is_around(&s)))
            .map(|n| n.val)
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_1: &str = r#"
    467..114..
    ...*......
    ..35..633.
    ......#...
    617*......
    .....+.58.
    ..592.....
    ......755.
    ...$.*....
    .664.598..
    "#;

    #[test]
    fn test_part_1() {
        let sum = part_1::process(SAMPLE_1);
        assert_eq!(sum, 4361);
    }

    #[test]
    fn test_part_1_input() {
        let text = std::fs::read_to_string("input.txt").unwrap();
        let start = 5;
        let text = &text[141 * start..141 * (3 + start)];

        let sum = part_1::process(text);
        let check = (288 + 896 + 469 + 146 + 790 + 194)
            + (730 + 250 + 359 + 462 + 138 + 49 + 713 + 342 + 604 + 676 + 418)
            + (274 + 346 + 725 + 541 + 600 + 366);

        assert_eq!(sum, check);
    }
}
