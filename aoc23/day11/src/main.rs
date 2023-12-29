use std::fs;
extern crate js_math;
use itertools::Itertools;
use js_math::vec2::*;
type Vec2i = Vec2<i32>;

fn parse_galaxies(s: &str) -> Vec<Vec2i> {
    s.lines()
    .enumerate()
    .flat_map(|(y, l)| {
        l.bytes()
        .enumerate()
        .filter_map(move |(x, b)|
            Some(make_vec2(x as i32, y as i32)).filter(|_| b == b'#')
        )
    })
    .collect()
}

fn count_empty(mut galaxy_it: impl Iterator<Item = i32>, tsize: i32) -> Vec<i32> {
    // lets count empty rows
    let mut last = galaxy_it.next();
    let mut sum = 0;
    let mut empty_rows = vec![];
    for y in 0..tsize {
        if let Some(g) = last {
            if g > y {
                sum += 1;
            } else {
                last = galaxy_it.find(|&f| f > y);
            }
        }
        empty_rows.push(sum);
    }
    empty_rows
}

fn shortest_path(s: &str, expansion: i32) -> Option<u64> {
    let galaxies = parse_galaxies(s);
    let tsize = make_vec2(
        galaxies.iter().max_by_key(|g| g.x).unwrap().x + 1,
        galaxies.iter().max_by_key(|g| g.y).unwrap().y + 1,
    );
    // galaxies are now naturally sorted on y
    let empty_rows = count_empty(galaxies.iter().map(|g| g.y), tsize.y);
    // sort by x as well
    let mut gbyx: Vec<_> = galaxies.iter().map(|g| g.x).collect();
    gbyx.sort();
    let empty_columns = count_empty(gbyx.into_iter(), tsize.x);

    let expanded: Vec<_> = galaxies.iter()
        .map(|g| {
            let xoff = empty_columns[g.x as usize] * (expansion - 1);
            let yoff = empty_rows[g.y as usize] * (expansion - 1);
            make_vec2(g.x + xoff, g.y + yoff)
        })
        .collect();

    let s: u64 = expanded.iter()
        .combinations(2)
        .map(|v| {
            match v[..] {
                [a, b] => (b.y.abs_diff(a.y) + b.x.abs_diff(a.x)) as u64,
                _ => panic!()
            }
        })
        .sum();
    Some(s)
}

fn part_one(s: &str) -> Option<u64> {
    shortest_path(s, 2)
}

fn part_two(s: &str) -> Option<u64> {
    shortest_path(s, 1000000)
}

fn main() {
    let input_str = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
        .expect("Something went wrong reading the file");

    println!("part_one {:?}", part_one(&input_str));
    println!("part_two {:?}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_STR: &'static str =
&"...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";

    #[test]
    fn test_count_empty() {
        let galaxies = parse_galaxies(EXAMPLE_STR);
        let tsize = make_vec2(
            galaxies.iter().max_by_key(|g| g.x).unwrap().x + 1,
            galaxies.iter().max_by_key(|g| g.y).unwrap().y + 1,
        );
        println!("{:?}", galaxies.iter().map(|g| g.y).collect::<Vec<_>>());
        assert_eq!(count_empty(galaxies.iter().map(|g| g.y), tsize.y), vec![0, 0, 0, 1, 1, 1, 1, 2, 2, 2]);
        let mut gbyx = galaxies.clone();
        gbyx.sort_by_key(|i| i.x);
        assert_eq!(count_empty(gbyx.iter().map(|g| g.x), tsize.x), vec![0, 0, 1, 1, 1, 2, 2, 2, 3, 3]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(EXAMPLE_STR), Some(374));
    }

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR), Some(82000210));
    }
}