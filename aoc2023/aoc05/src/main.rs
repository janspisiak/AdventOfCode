use std::{fs, ops::Range};

struct RangeOffset(Range<i64>, i64);

struct Map<'a> {
    _name: &'a str,
    ranges: Vec<RangeOffset>,
}

fn parse_seed_ranges<'a>(s: &'a str) -> Vec<Range<i64>> {
    let mut seeds = vec![];
    let mut it = s.split_ascii_whitespace();
    let mut start: Option<i64> = None;
    while let Some(ns) = it.next() {
        let n = ns.parse::<i64>().expect("seed number format");
        if let Some(f) = start.take() {
            seeds.push(f..f + n);
        } else {
            start.insert(n);
        }
    }
    assert!(start.is_none());
    seeds
}

fn parse_maps<'a>(lines: impl IntoIterator<Item = &'a str>) -> Vec<Map<'a>> {
    // parse maps
    let mut lines_it = lines.into_iter();
    let mut maps = vec![];
    while let Some(l) = lines_it.next() {
        if l.ends_with("map:") {
            let (name, _) = l.split_once(" ").expect("Map name definition");
            maps.push(Map{ _name: name, ranges: vec![] });
            continue;
        }
        let range_def: Vec<_> = l.split_ascii_whitespace()
            .map(|ns| ns.parse::<i64>().expect("Number format"))
            .collect();
        match &range_def[..] {
            &[dest_from, src_from, range_len] => {
                let last = maps.last_mut().expect("Range before map definition");
                last.ranges.push(RangeOffset(src_from..src_from + range_len, dest_from  - src_from))
            },
            _ => (),
        }
    }
    for m in maps.iter_mut() {
        m.ranges.sort_by(|RangeOffset(a, _), RangeOffset(b, _)| a.start.cmp(&b.start))
    }
    maps
}

fn part_one(s: &str) -> i64 {
    let mut lines_it = s.lines();
    let seeds_str = lines_it.next().expect("Seeds line definition");
    lines_it.next(); // empty line
    // parse
    let (_, seeds) = seeds_str.split_once(":").expect("Seeds definition");
    let maps = parse_maps(lines_it);

    seeds.split_ascii_whitespace()
        .map(|seed_str| {
            let mut seed: i64 = seed_str.parse().expect("Seed value");
            for m in maps.iter() {
                let found = m.ranges.iter().find(|RangeOffset(r, _)| r.contains(&seed));
                if let Some(RangeOffset(_, t)) = found {
                    seed = seed + t;
                }
            }
            seed
        })
        .min()
        .expect("No seeds defined")
}

enum OverlapCase {
    Super,
    Sub,
    Left,
    Right,
    None
}
fn range_overlap<T: Ord>(range: &Range<T>, by: &Range<T>) -> OverlapCase {
    if range.contains(&by.start) {
        if range.contains(&by.end) {
            OverlapCase::Super
        } else {
            OverlapCase::Right
        }
    } else if by.contains(&range.start) {
        if by.contains(&range.end) {
            OverlapCase::Sub
        } else {
            OverlapCase::Left
        }
    } else {
        OverlapCase::None
    }
}
fn range_split(src: &Range<i64>, by: &[RangeOffset], new_seeds: &mut Vec<Range<i64>>) {
    let mut non_empty_push = |r: Range<i64>| {
        if !r.is_empty() {
            new_seeds.push(r);
        }
    };
    let apply_offset = |r: Range<i64>, off: &i64| {
        r.start + off..r.end + off
    };
    let mut rest = src.clone();
    for RangeOffset(mr, off) in by.iter() {
        let split_res = range_overlap(&rest, mr);
        match split_res {
            OverlapCase::Super => {
                non_empty_push(rest.start..mr.start);
                non_empty_push(apply_offset(mr.clone(), off));
                rest = mr.end..rest.end;
            },
            OverlapCase::Sub => {
                non_empty_push(apply_offset(rest.clone(), off));
                rest = rest.end..rest.end;
                break;
            },
            OverlapCase::Left => {
                non_empty_push(apply_offset(rest.start..mr.end, off));
                rest = mr.end..rest.end;
            },
            OverlapCase::Right => {
                non_empty_push(rest.start..mr.start);
                non_empty_push(apply_offset(mr.start..rest.end, off));
                rest = rest.end..rest.end;
                break;
            },
            OverlapCase::None => ()
        }
    }
    if !rest.is_empty() {
        new_seeds.push(rest);
    }
}

fn part_two(s: &str) -> i64 {
    let mut lines_it = s.lines();
    let seeds_str = lines_it.next().expect("Seeds line definition");
    lines_it.next(); // empty line
    // parse
    let (_, seeds_list) = seeds_str.split_once(":").expect("Seeds definition");
    let maps = parse_maps(lines_it);

    let mut seeds = parse_seed_ranges(seeds_list);
    let mut new_seeds = vec![];
    for m in maps.iter() {
        for s in seeds.iter() {
            range_split(s, &m.ranges, &mut new_seeds)
        }
        seeds.clear();
        seeds.append(&mut new_seeds);
    }
    seeds.iter()
        .map(|r| r.start)
        .min()
        .expect("No seeds defined")
}

fn main() {
    let input_str = fs::read_to_string("aoc2023/aoc05/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {}", part_one(&input_str));
    println!("part_two {}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range_split() {
        let mut new_seeds: Vec<Range<i64>> = vec![];
        range_split(&(1..5), &[RangeOffset(0..3, 1), RangeOffset(3..4, 2)], &mut new_seeds);
        assert_eq!(new_seeds, vec![2..4, 5..6, 4..5]);
    }

    const EXAMPLE_STR: &'static str =
&"seeds: 79 14 55 13

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
56 93 4";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), 35);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), 46);
    }
}