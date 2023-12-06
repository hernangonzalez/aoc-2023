use core::ops::Range;

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let r1 = part_1::process(&text);
    let r2 = part_2::process(&text);
    println!("Part 1: {r1}");
    println!("Part 2: {r2}");
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

type Num = Element<u32>;
type Symbol = Element<char>;

impl Symbol {
    fn is_gear(&self) -> bool {
        self.val == '*'
    }
}

impl Num {
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

    fn is_adyacent(&self, sym: &Symbol) -> bool {
        let range_x = self.range();
        let dy = self.loc.y.abs_diff(sym.loc.y);
        dy <= 1 && range_x.contains(&sym.loc.x)
    }
}

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
        nums.filter(|n| syms.clone().any(|s| n.is_adyacent(&s)))
            .map(|n| n.val)
            .sum()
    }
}

mod part_2 {
    use crate::parse;

    pub fn process(text: &str) -> u32 {
        let items = parse(text);
        let nums = items.iter().filter_map(|e| e.num());
        let gears = items.iter().filter_map(|e| e.sym()).filter(|s| s.is_gear());

        gears
            .map(|g| {
                nums.clone()
                    .filter(|n| n.is_adyacent(&g))
                    .map(|n| n.val)
                    .collect::<Vec<_>>()
            })
            .filter(|v| v.len() == 2)
            .map(|v| v.iter().copied().reduce(|l, r| l * r).unwrap_or(0))
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
    fn test_part_2() {
        let sum = part_2::process(SAMPLE_1);
        assert_eq!(sum, 467 * 35 + 755 * 598);
    }
}
