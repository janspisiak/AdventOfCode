use std::fs;

fn series_prev(prev: &[i64]) -> i64 {
    let new: Vec<_> = prev.windows(2)
        .map(|s| {
            let (a, b) = (s[0], s[1]);
            b - a
        })
        .collect();
    if new.iter().all(|&v| v == 0) {
        0
    } else {
        let diff = series_prev(new.as_slice());
        new.first().unwrap() - diff
    }
}

fn series_next(prev: &[i64]) -> i64 {
    let new: Vec<_> = prev.windows(2)
        .map(|s| {
            let (a, b) = (s[0], s[1]);
            b - a
        })
        .collect();
    if new.iter().all(|&v| v == 0) {
        0
    } else {
        let diff = series_next(new.as_slice());
        new.last().unwrap() + diff
    }
}

fn part_one(s: &str) -> Option<i64> {
    Some(s.lines()
        .map(|l| {
            let series: Vec<_> = l.split_ascii_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect();
            let diff = series_next(&series);
            series.last().unwrap() + diff
        })
        .sum())
}

fn part_two(s: &str) -> Option<i64> {
    Some(s.lines()
        .map(|l| {
            let series: Vec<_> = l.split_ascii_whitespace()
                .map(|w| w.parse::<i64>().unwrap())
                .collect();
            let diff = series_prev(&series);
            series.first().unwrap() - diff
        })
        .sum())
}

fn main() {
    let input_str = fs::read_to_string("aoc2023/aoc09/input.txt")
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
&"0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(114));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), Some(2));
    }
}