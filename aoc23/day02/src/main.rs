use std::{fs, ops, iter::Sum};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct RGB {
    r: u8,
    g: u8,
    b: u8,
}

impl RGB {
    fn new(r: u8, g: u8, b: u8) -> RGB {
        RGB{ r, g, b }
    }

    fn all_le(&self, rhs: &RGB) -> bool {
        self.r <= rhs.r && self.g <= rhs.g && self.b <= rhs.b
    }

    fn comp_max(&self, rhs: &RGB) -> RGB {
        RGB{ r: self.r.max(rhs.r), g: self.g.max(rhs.g), b: self.b.max(rhs.b) }
    }

    fn comp_mul(&self) -> u64 {
        self.r as u64 * self.g as u64 * self.b as u64
    }
}

impl ops::Add<RGB> for RGB
{
    type Output = RGB;
    fn add(self, rhs: RGB) -> RGB {
        RGB{ r: self.r + rhs.r, g: self.g + rhs.g, b: self.b + rhs.b }
    }
}

impl Sum for RGB {
    fn sum<I>(iter: I) -> Self
        where I: Iterator<Item = RGB>,
    {
        iter.fold(RGB { r: 0, g: 0, b: 0 }, |a, b| a + b)
    }
}

fn parse_colors(s: &str) -> impl Iterator<Item = RGB> + '_ {
    s.split(';') // 3 red, 2 green
    .map(|colors| colors.split(',')
        .map(|c| {
            let mut it = c.split_ascii_whitespace();
            let val = it.next().unwrap().parse::<u8>().unwrap();
            let color = it.next().unwrap();
            match color {
                "red" =>    RGB{ r: val, g: 0, b: 0 },
                "green" =>  RGB{ r: 0, g: val, b: 0 },
                "blue" =>   RGB{ r: 0, g: 0, b: val },
                _ => panic!("Unknown color")
            }
        })
        .sum()
    )
}

fn part_one(s: &str, constraints: RGB) -> u64 {
    s.lines()
    .filter_map(|l| {
        //game GID: 3 blue, 2 red; 1 green
        let (gs, states) = l.split_once(':').unwrap();
        let (_, gid) = gs.split_once(' ').unwrap();
        let all_possible = parse_colors(states).all(|color| color.all_le(&constraints));
        if all_possible {
            Some(gid.parse::<u64>().unwrap())
        } else {
            None
        }
    })
    .sum()
}

fn part_two(s: &str) -> u64 {
    s.lines()
    .map(|l| {
        //game GID: 3 blue, 2 red; 1 green
        let (_, states) = l.split_once(':').unwrap();
        let min_cubes = parse_colors(states).fold(RGB::new(0, 0, 0), |a, c| a.comp_max(&c));
        min_cubes.comp_mul()
    })
    .sum()
}

fn main() {
    let input_str = fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt")
        .expect("Something went wrong reading the file");

    println!("part_one {}", part_one(&input_str, RGB::new(12, 13, 14)));
    println!("part_two {}", part_two(&input_str));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rgb() {
        assert!(RGB::new(0, 2, 0).all_le(&RGB::new(1, 2, 3)));
        assert!(!RGB::new(0, 3, 0).all_le(&RGB::new(1, 2, 3)));
    }

    #[test]
    fn test_parse_color() {
        assert_eq!(parse_colors("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green").collect::<Vec<_>>(),
            vec![RGB::new(4, 0, 3), RGB::new(1, 2, 6), RGB::new(0, 2, 0)]);
    }

    #[test]
    fn test_part_one() {
        assert_eq!(part_one(
"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 14 blue, 13 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
            RGB::new(12, 13, 14)), 8);
    }
}
