use std::{fs, iter};
extern crate js_math;
use js_math::vec2::*;

#[derive(Clone, Copy, Debug, PartialEq)]
struct Symbol{ pos: Point }

#[derive(Clone, Copy, Debug, PartialEq)]
struct Number{ val: u64, from: Point, to: Point }

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
enum ParseGroup {
    Number,
    Space,
    Symbol,
}

impl From<u8> for ParseGroup {
    fn from(value: u8) -> Self {
        match value {
            b'0'..=b'9' => ParseGroup::Number,
            b'.' | b'\n' => ParseGroup::Space,
            _ => ParseGroup::Symbol,
        }
    }
}

fn parse_groups<G: 'static + From<u8> + PartialEq>(s: &'_ str)
    -> impl Iterator<Item = (usize, usize, G)> + '_
{
    let mut npos = 0;
    let mut last: Option<(G, usize)> = None;
    iter::from_fn(move || {
        loop {
            let pos = npos;
            if pos >= s.len() {
                break None;
            }
            npos += 1;
            let c = s.as_bytes()[pos];
            let g: G = c.into();
            if last.as_ref().is_none() || last.as_ref().is_some_and(|lg| lg.0 != g) {
                let prev = last.replace((g, pos));
                if let Some((vg, start)) = prev {
                    break Some((start, pos, vg))
                }
            }
        }
    })
}

fn part_one(s: &str) -> u64 {
    let row_length = s.chars().position(|c| c.is_whitespace()).unwrap() + 1;
    let make_pos = |i: usize| {
        make_vec2((i % row_length) as i32, (i / row_length) as i32)
    };

    let mut numbers: Vec<_> = vec![];
    let mut symbols: Vec<_> = vec![];
    for (from_i, to_i, group) in parse_groups(&s) {
        match group {
            ParseGroup::Number => {
                let val = s[from_i..to_i].parse::<u64>().unwrap();
                numbers.push(Number {
                    val,
                    from: make_pos(from_i)   + make_vec2(-1, -1),
                    to:   make_pos(to_i - 1) + make_vec2( 1,  1),
                });
            },
            ParseGroup::Symbol => {
                symbols.push(Symbol { pos: make_pos(from_i) })
            },
            _ => (),
        };
    }

    numbers.iter()
        .filter_map(|n| {
            let has_adjacent = symbols.iter().any(|s| s.pos.ge(n.from) && s.pos.le(n.to));
            if has_adjacent { Some(n.val) } else { None }
        })
        // .for_each(|n| println!("{n}")); 0
        .sum()
}

fn part_two(s: &str) -> u64 {
    let row_length = s.chars().position(|c| c.is_whitespace()).unwrap() + 1;
    let make_pos = |i: usize| {
        make_vec2((i % row_length) as i32, (i / row_length) as i32)
    };

    let mut numbers: Vec<_> = vec![];
    let mut gears: Vec<_> = vec![];
    for (from_i, to_i, group) in parse_groups(&s) {
        match group {
            ParseGroup::Number => {
                let val = s[from_i..to_i].parse::<u64>().unwrap();
                numbers.push(Number {
                    val,
                    from: make_pos(from_i)   + make_vec2(-1, -1),
                    to:   make_pos(to_i - 1) + make_vec2( 1,  1),
                });
            },
            ParseGroup::Symbol => {
                let name = &s[from_i..to_i];
                if name == "*" {
                    gears.push(Symbol { pos: make_pos(from_i) });
                }
            },
            _ => (),
        };
    }

    gears.iter()
        .map(|g| {
            let neighbours: Vec<_> = numbers.iter()
                .filter(|n| g.pos.ge(n.from) && g.pos.le(n.to))
                .collect();
            match neighbours.as_slice() {
                &[first, second] => {
                    first.val * second.val
                },
                _ => 0
            }
        })
        .sum()
}

fn main() {
    let input_str = fs::read_to_string("aoc2023/aoc03/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {}", part_one(&input_str));
    println!("part_two {}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_groups() {
        assert_eq!(parse_groups::<ParseGroup>(&"123#..*456\n").collect::<Vec<_>>(), vec![
            (0, 3, ParseGroup::Number),
            (3, 4, ParseGroup::Symbol),
            (4, 6, ParseGroup::Space),
            (6, 7, ParseGroup::Symbol),
            (7, 10, ParseGroup::Number),
        ]);
    }

    const EXAMPLE_STR: &'static str =
&"467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), 4361);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), 467835);
    }
}