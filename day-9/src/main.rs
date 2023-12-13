use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let text = std::fs::read_to_string("day-9/input.txt")?;
    let res = part_1::process(&text);
    println!("Part 1: {res}");
    let res = part_2::process(&text);
    println!("Part 2: {res}");
    Ok(())
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn grid(&self) -> Vec<Vec<i64>> {
        let mut grid = Vec::new();
        grid.push(self.0.clone());

        let is_at_end = |v: &Vec<i64>| v.iter().all(|i| *i == 0);

        let mut vec = &self.0;
        while !is_at_end(vec) {
            let deltas = vec
                .iter()
                .enumerate()
                .skip(1)
                .map(|(i, val)| val - vec[i - 1])
                .collect::<Vec<_>>();
            grid.push(deltas);
            vec = grid.last().unwrap();
        }

        grid
    }

    fn next(&self) -> i64 {
        self.grid().iter().flat_map(|v| v.last()).sum()
    }

    fn prev(&self) -> i64 {
        self.grid()
            .iter()
            .flat_map(|v| v.first())
            .copied()
            .rev()
            .reduce(|acc, i| i - acc)
            .unwrap()
    }
}

mod parse {
    use super::*;

    impl From<&str> for Sequence {
        fn from(s: &str) -> Self {
            Sequence(
                s.split_whitespace()
                    .flat_map(|c| c.parse())
                    .collect::<Vec<_>>(),
            )
        }
    }

    pub fn sequence(text: &str) -> Vec<Sequence> {
        text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| Sequence::from(l))
            .collect()
    }
}

mod part_1 {
    use super::*;

    pub fn process(text: &str) -> i64 {
        parse::sequence(text).iter().map(|s| s.next()).sum()
    }
}

mod part_2 {
    use super::*;

    pub fn process(text: &str) -> i64 {
        parse::sequence(text).iter().map(|s| s.prev()).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    0 3 6 9 12 15
    1 3 6 10 15 21
    10 13 16 21 30 45
    "#;

    #[test]
    fn test_part_1() {
        let res = part_1::process(SAMPLE);
        assert_eq!(res, 114);
    }

    #[test]
    fn test_part_2() {
        let res = part_2::process(SAMPLE);
        assert_eq!(res, 2)
    }
}
