use std::fs;

fn to_win_count(s: &'_ str) -> impl Iterator<Item = usize> + '_ {
    s.lines()
    .map(|l| {
        let (_, nums_str) = l.split_once(':').expect("Wrong line format");
        let (win_str, play_str) = nums_str.split_once('|').expect("Wrong numbers list format");
        let win_nums: Vec<_> = win_str.split_ascii_whitespace()
            .map(|s| s.parse::<u64>().expect("Wrong number format"))
            .collect();

        play_str.split_ascii_whitespace()
            .map(|s| s.parse::<u64>().expect("Wrong number format"))
            .filter(|n| win_nums.contains(n))
            .count()
    })
}

fn part_one(s: &str) -> u64 {
    to_win_count(s)
    .map(|win_count| {
        if win_count > 0 {
            2u64.pow(win_count as u32 - 1)
        } else {
            0
        }
    })
    .sum()
}

fn part_two(s: &str) -> u64 {
    let win_counts: Vec<_> = to_win_count(s).collect();
    let mut card_counts: Vec<usize> = vec![1; win_counts.len()];
    for (i, wc) in win_counts.iter().enumerate() {
        let cur_count = card_counts[i];
        let clen = card_counts.len();
        let (cfrom, cto) = (clen.min(i + 1), clen.min(i + 1 + wc));
        for next_count in &mut card_counts[cfrom..cto] {
            *next_count += cur_count;
        }
    }
    let card_sum: usize = card_counts.iter().sum();
    card_sum as u64
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

    const EXAMPLE_STR: &'static str =
&"Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), 13);
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), 30);
    }
}