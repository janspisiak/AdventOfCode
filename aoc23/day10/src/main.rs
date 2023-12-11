use std::fs;

fn part_one(s: &str) -> Option<u64> {
    s.chars().find()
    Some(0)
}

fn part_two(s: &str) -> Option<u64> {
    Some(0)
}

fn main() {
    let input_str = fs::read_to_string("aoc2023/aoc06/input.txt")
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
&"7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(0));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), Some(0));
    }
}