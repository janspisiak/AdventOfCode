use std::{fs, collections::HashMap};

fn build_map<'a>(lines_it: impl Iterator<Item = &'a str>) -> HashMap<&'a str, (&'a str, &'a str)> {
    lines_it.fold(HashMap::new(), |mut m, node_str| {
        let (key, childs_list) = node_str.split_once('=').unwrap();
        let (left, right) = childs_list.trim_matches(|c: char| c == '(' || c == ')' || c.is_ascii_whitespace())
            .split_once(',').unwrap();
        m.insert(key.trim(), (left.trim(), right.trim()));
        m
    })
}

fn count_path(from: &str, end: &str, instr_str: &str, map: &HashMap<&str, (&str, &str)>) -> u64 {
    let mut curr = from;
    let mut cnt: u64 = 0;
    let mut instr_it = instr_str.chars().cycle();
    while !curr.ends_with(end) {
        let instr = instr_it.next().unwrap();
        let &(left, right) = map.get(curr).unwrap();
        match instr {
            'L' => curr = left,
            'R' => curr = right,
            _ => panic!(),
        }
        cnt += 1;
    }
    cnt
}

fn part_one(s: &str) -> Option<u64> {
    let mut lines_it = s.lines();
    let (instr_str, _) = (lines_it.next().unwrap(), lines_it.next().unwrap());
    let map = build_map(lines_it);
    Some(count_path("AAA", "ZZZ", instr_str, &map))
}

fn part_two(s: &str) -> Option<u64> {
    let mut lines_it = s.lines();
    let (instr_str, _) = (lines_it.next().unwrap(), lines_it.next().unwrap());
    let map = build_map(lines_it.clone());
    let nodes = lines_it.filter_map(|l| {
        let key = l.split_ascii_whitespace().next().unwrap();
        Some(key).filter(|k| k.ends_with("A"))
    });

    let cnt = nodes.map(|curr| {
            count_path(curr, "Z", instr_str, &map)
        })
        .fold(1, |a, c| {
            num_integer::lcm(a, c)
        });
    Some(cnt)
}

fn main() {
    let input_str = fs::read_to_string("aoc2023/aoc08/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {:?}", part_one(&input_str));
    println!("part_two {:?}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_int_roots() {
    }

    const EXAMPLE_STR: &'static str =
&"RL

AAA = (BBB, CCC)
BBB = (ZZZ, EEE)
CCC = (EEE, GGG)
DDD = (DDD, DDD)
EEE = (EEE, BBB)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(4));
    }

    const EXAMPLE_STR_TWO: &'static str =
&"LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR_TWO), Some(6));
    }
}