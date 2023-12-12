use color_eyre::Result;

fn main() -> Result<()> {
    color_eyre::install()?;
    let text = std::fs::read_to_string("input.txt")?;
    let res = part_1::process(&text);
    println!("Part 1: {res}");
    Ok(())
}

struct Sequence(Vec<i64>);

impl Sequence {
    fn next(&self) -> i64 {
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

        grid.iter().flat_map(|v| v.last()).copied().sum()
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
}

mod part_1 {
    use super::*;

    pub fn process(text: &str) -> i64 {
        text.lines()
            .map(|l| l.trim())
            .filter(|l| !l.is_empty())
            .map(|l| Sequence::from(l))
            .map(|s| s.next())
            .sum()
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
}
