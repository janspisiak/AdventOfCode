use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
#[repr(u8)]
enum HandType {
    None,
    High,
    Pair,
    TwoPairs,
    Three,
    FullHouse,
    Four,
    Five,
}

// A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
fn card_weight_one(c: u8) -> u8 {
    match c {
        n @ b'2'..=b'9' => n - b'0',
        b'T' => 10,
        b'J' => 11,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    }
}
fn card_weight_two(c: u8) -> u8 {
    match c {
        n @ b'2'..=b'9' => n - b'0',
        b'T' => 10,
        b'J' => 1,
        b'Q' => 12,
        b'K' => 13,
        b'A' => 14,
        _ => 0,
    }
}
fn match_hand(s: &[(u8, u8)]) -> HandType {
    match s {
        &[(_, 5)] => HandType::Five,
        &[(_, 4), ..] => HandType::Four,
        &[(_, 3), (_, 2)] => HandType::FullHouse,
        &[(_, 3), ..] => HandType::Three,
        &[(_, 2), (_, 2), ..] => HandType::TwoPairs,
        &[(_, 2), ..] => HandType::Pair,
        &[(_, 1), ..] => HandType::High,
        _ => HandType::None,
    }
}

fn parse_hand(s: &str) -> HandType {
    let mut counts: Vec<(u8, u8)> = Vec::with_capacity(5);
    for b in s.bytes() {
        if let Some(exist) = counts.iter_mut().find(|(k, _)| b == *k) {
            exist.1 += 1;
        } else {
            counts.push((b, 1));
        }
    }
    counts.sort_by(|(_, a), (_, b)| b.cmp(a));
    match_hand(counts.as_slice())
}

fn parse_hand_jokers(s: &str) -> HandType {
    let mut counts: Vec<(u8, u8)> = Vec::with_capacity(5);
    for b in s.bytes() {
        if let Some(exist) = counts.iter_mut().find(|(k, _)| b == *k) {
            exist.1 += 1;
        } else {
            counts.push((b, 1));
        }
    }
    let mut joker_count = 0;
    if let Some(index) = counts.iter().position(|(k, _)| *k == b'J') {
        joker_count = counts[index].1;
        counts.swap_remove(index);
    }
    counts.sort_by(|(_, a), (_, b)| b.cmp(a));
    let mut hand = match_hand(counts.as_slice());
    let apply_joker = |t: HandType| match t {
        HandType::Five | HandType::FullHouse => panic!("Impossible {}", s),
        HandType::Four => HandType::Five,
        HandType::Three => HandType::Four,
        HandType::TwoPairs => HandType::FullHouse,
        HandType::Pair => HandType::Three,
        HandType::High => HandType::Pair,
        HandType::None => HandType::High,
    };
    for _ in 0..joker_count {
        hand = apply_joker(hand);
    }
    hand
}

struct Hand(u64, u64);

fn part_one(s: &str) -> Option<u64> {
    let mut ranks: Vec<_> = s.lines()
        .map(|l| {
            let mut line_it = l.split_ascii_whitespace();
            let (cards_str, bid_str) = (line_it.next().unwrap(), line_it.next().unwrap());
            let hand = parse_hand(cards_str) as u8;
            let weight: u64 = cards_str.bytes()
                .fold(hand as u64, |a, b| {
                    let w = card_weight_one(b);
                    (a << 8) | (w as u64)
                });
            Hand(weight, bid_str.parse::<u64>().unwrap())
        })
        .collect();

    ranks.sort_by(|Hand(a, _), Hand(b, _)| a.cmp(b));
    let s = ranks.iter()
        .enumerate()
        .map(|(i, Hand(_, b))| {
            (i + 1) as u64 * b
        })
        .sum();
    Some(s)
}

fn part_two(s: &str) -> Option<u64> {
    let mut ranks: Vec<_> = s.lines()
        .map(|l| {
            let mut line_it = l.split_ascii_whitespace();
            let (cards_str, bid_str) = (line_it.next().unwrap(), line_it.next().unwrap());
            let hand = parse_hand_jokers(cards_str) as u8;
            let weight: u64 = cards_str.bytes()
                .fold(hand as u64, |a, b| {
                    let w = card_weight_two(b);
                    (a << 8) | (w as u64)
                });
            Hand(weight, bid_str.parse::<u64>().unwrap())
        })
        .collect();

    ranks.sort_by(|Hand(a, _), Hand(b, _)| a.cmp(b));
    let s = ranks.iter()
        .enumerate()
        .map(|(i, Hand(_, b))| {
            (i + 1) as u64 * b
        })
        .sum();
    Some(s)
}

fn main() {
    let input_str = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {:?}", part_one(&input_str));
    println!("part_two {:?}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hand() {
        assert_eq!(parse_hand("33333"), HandType::Five);
        assert_eq!(parse_hand("33332"), HandType::Four);
        assert_eq!(parse_hand("33322"), HandType::FullHouse);
        assert_eq!(parse_hand("33321"), HandType::Three);
        assert_eq!(parse_hand("33221"), HandType::TwoPairs);
        assert_eq!(parse_hand("33210"), HandType::Pair);
    }

    #[test]
    fn test_parse_jokers() {
        assert_eq!(parse_hand_jokers("JJJJJ"), HandType::Five);
        assert_eq!(parse_hand_jokers("3333J"), HandType::Five);
        assert_eq!(parse_hand_jokers("33JJ2"), HandType::Four);
        assert_eq!(parse_hand_jokers("33J22"), HandType::FullHouse);
        assert_eq!(parse_hand_jokers("3JJ21"), HandType::Three);
        assert_eq!(parse_hand_jokers("3J210"), HandType::Pair);
    }

    const EXAMPLE_STR: &'static str =
&"32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(6440));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), Some(5905));
    }
}