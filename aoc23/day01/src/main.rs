use std::{fs, iter};

fn parse_one(s: &str) -> Option<u64> {
    let radix = 10;
    let to_digit = |c: char| c.to_digit(radix);
    let first = s.chars().find_map(to_digit);
    let last = s.chars().rev().find_map(to_digit);
    match (first, last)
    {
        (Some(tens), Some(ones)) => Some((tens * 10 + ones) as u64),
        _ => None
    }
}

const ALL_DIGITS: &'static [&'static str] = &[
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine"
];

fn find_digit(s: &str, from: usize) -> Option<(usize, u32)> {
    let radix = 10;
    let mut rs = from;
    loop {
        if rs >= s.len() {
            break None;
        }
        let slice = &s[rs..];
        rs += 1;
        let first = slice.chars().next().and_then(|c: char| c.to_digit(radix));
        if let Some(d) = first {
            break Some((rs, d));
        }
        let digit_index = ALL_DIGITS.iter().position(|&d| slice.starts_with(d));
        if let Some(di) = digit_index {
            // cannot optimize due to overlap
            // rs += ALL_DIGITS[di].len() - 1;
            break Some((rs, (di + 1) as u32));
        }
    }
}

fn parse_digits(s: &'_ str) -> impl Iterator<Item = u32> + '_ {
    let mut rs = 0;
    iter::from_fn(move || {
        match find_digit(s, rs) {
            Some((next, digit)) => {
                rs = next;
                Some(digit)
            },
            _ => None,
        }
    })
}

fn parse_two(s: &str) -> Option<u64> {
    let mut iter = parse_digits(s);
    let first = iter.next();
    let last = iter.last().or(first);
    match (first, last)
    {
        (Some(tens), Some(ones)) => Some((tens * 10 + ones) as u64),
        _ => None
    }
}

fn part_one(s: &str) -> u64 {
    s.lines()
    .filter_map(|l| parse_one(l))
    .sum()
}

fn part_two(s: &str) -> u64 {
    s.lines()
    .map(|l| parse_two(l).unwrap())
    .sum()
}

fn main() {
    let input_str = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {}", part_one(&input_str));
    println!("part_two {}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        assert_eq!(parse_one("pqr3stu8vwx"), Some(38));
        assert_eq!(parse_one("treb7uchet"), Some(77));
    }

    #[test]
    fn part_one_test() {
        assert_eq!(
            part_one(
&"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"),
            142
        );
    }

    #[test]
    fn parse_digits_test() {
        assert_eq!(parse_digits(&"4nineeightseven2").collect::<Vec<_>>(), vec![4, 9, 8, 7, 2]);
        assert_eq!(parse_digits(&"7pqrstsixteen").collect::<Vec<_>>(), vec![7, 6]);
        assert_eq!(parse_digits(&"28gtbkszmrtmnineoneightmx").collect::<Vec<_>>(), vec![2, 8, 9, 1, 8]);
    }

    #[test]
    fn parse_two_test() {
        assert_eq!(parse_two("zoneight234"), Some(14));
        assert_eq!(parse_two("7pqrstsixteen"), Some(76));
        assert_eq!(parse_two("treb7uchet"), Some(77));

        assert_eq!(parse_two("one"), Some(11));
        assert_eq!(parse_two("two"), Some(22));
        assert_eq!(parse_two("three"), Some(33));
        assert_eq!(parse_two("4"), Some(44));
        assert_eq!(parse_two("5"), Some(55));
        assert_eq!(parse_two("6"), Some(66));

        assert_eq!(parse_two("45122"), Some(42));
        assert_eq!(parse_two("jvvslnkdk6qnfzjzvseight55eight"), Some(68));
        assert_eq!(parse_two("4twoeightgrhhkrvtkrzpfive7seven"), Some(47));
        assert_eq!(parse_two("three8gsmkpzsmfvf2"), Some(32));
        assert_eq!(parse_two("fiveeight5sevenone9twoseven"), Some(57));
        assert_eq!(parse_two("4seightjjdkdglspz3vg"), Some(43));
        assert_eq!(parse_two("sevenssrzkspld2"), Some(72));
        assert_eq!(parse_two("qnzcvcthrsgjlnzxmxlppjdpnine8seven7"), Some(97));
        assert_eq!(parse_two("eight7xhvkrcr"), Some(87));
        assert_eq!(parse_two("28gtbkszmrtmnineoneightmx"), Some(28));
    }

    #[test]
    fn part_two_test() {
        assert_eq!(
            part_two(
&"two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"),
            281
        );
    }
}
