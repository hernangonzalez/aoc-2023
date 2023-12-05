const NUMS_ENG: [&str; 10] = ["zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn main() {
    let text = std::fs::read_to_string("input.txt").unwrap();
    let sum = process(&text);
    println!("Answer: {sum}");
}

struct AocDigit<'a> {
    inner: &'a str
}

impl<'a> Iterator for AocDigit<'a> {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        let chars = self.inner.chars();

        for c in chars {
            let sub = self.inner;
            if let Some(substr) = self.inner.get(1..) {
                self.inner = substr;
            }

            if let Some(digit) = c.to_digit(10) {
                return Some(digit);
            }

            for (i, w) in NUMS_ENG.iter().enumerate() {
                if sub.starts_with(w) {
                    return Some(i as u32)
                }
            }
        }

        None
    }
}

fn process_line(line: &str) -> u32 {
    let mut nums = AocDigit { inner: line };
    let lhs = nums.next().unwrap_or(0);
    let rhs = nums.last().unwrap_or(lhs);
    let res = lhs * 10 + rhs;
    println!("{line} -> {res}");
    res
}

fn process(text: &str) -> u32 {
    text.lines()
        .filter(|l| l.is_ascii())
        .map(|l|l.trim().to_lowercase())
        .filter(|l| !l.is_empty())
        .map(|l| process_line(&l) )
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = r#"
    1abc2
    pqr3stu8vwx
    a1b2c3d4e5f
    treb7uchet
    "#;

    #[test]
    fn test_sample_part1() {
        let sum = process(SAMPLE);
        assert_eq!(sum, 142);
    }

    const SAMPLE_2: &str = r#"
    two1nine
    eightwothree
    abcone2threexyz
    xtwone3four
    4nineeightseven2
    zoneight234
    7pqrstsixteen
    "#;

    #[test]
    fn test_sample_part2() {
        let sum = process(SAMPLE_2);
        assert_eq!(sum, 281);
    }

    #[test]
    fn test_line() {
        let res = process_line("jjhxddmg5mqxqbgfivextlcpnvtwothreetwonerzk");
        assert_eq!(res, 51);
    }
}
