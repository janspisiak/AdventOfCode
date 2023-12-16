use std::fs;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Roots {
    Two(i64, i64),
    One(i64),
    None,
}
fn find_int_roots(t: i64, l: i64) -> Roots {
    // f(x) = -x^2 + xt - l = 0
    // D = b^2 - 4ac
    // a = -1, b = t, c = -l
    let dis = t * t - 4 * l;
    let rc = t as f32 * 0.5f32;
    if dis > 0 {
        let first = rc + (dis as f32).sqrt() * (- 0.5f32);
        let sec = rc - (dis as f32).sqrt() * (- 0.5f32);
        Roots::Two(first.floor() as i64 + 1, sec.ceil() as i64 - 1)
    } else if dis == 0 && rc == rc.trunc() {
        Roots::One(rc as i64)
    } else {
        Roots::None
    }
}

fn part_one(s: &str) -> Option<u64> {
    let mut lines_it = s.lines();
    let (_, times) = lines_it.next()?
        .split_once(':')?;
    let (_, dists) = lines_it.next()?
        .split_once(':')?;
    let r = times.split_ascii_whitespace()
        .zip(dists.split_ascii_whitespace())
        .map(|(time, dist)| {
            let roots = find_int_roots(time.parse::<i64>().unwrap(), dist.parse::<i64>().unwrap());
            let c = match roots {
                Roots::Two(a, b) => b - a + 1,
                Roots::One(_) => 1,
                Roots::None => 1,
            };
            c as u64
        })
        .product();
    Some(r)
}

fn part_two(s: &str) -> Option<u64> {
    let mut lines_it = s.lines();
    let (_, times) = lines_it.next()?
        .split_once(':')?;
    let (_, dists) = lines_it.next()?
        .split_once(':')?;

    let time: String = times.split_ascii_whitespace().collect();
    let dist: String = dists.split_ascii_whitespace().collect();
    let roots = find_int_roots(time.parse::<i64>().unwrap(), dist.parse::<i64>().unwrap());
    let c = match roots {
        Roots::Two(a, b) => b - a + 1,
        Roots::One(_) => 1,
        Roots::None => 1,
    };
    Some(c as u64)
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
    fn test_find_int_roots() {
        assert_eq!(find_int_roots(7, 9), Roots::Two(2, 5));
        assert_eq!(find_int_roots(15, 40), Roots::Two(4, 11));
        assert_eq!(find_int_roots(30, 200), Roots::Two(11, 19));
    }

    const EXAMPLE_STR: &'static str =
&"Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(288));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), Some(71503));
    }
}