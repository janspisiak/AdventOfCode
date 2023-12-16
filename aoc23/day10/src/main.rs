use std::{fs, collections::VecDeque};
extern crate js_math;
use js_math::vec2::*;
type Vec2i = Vec2<i32>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Ground,
    Start,
    NS,
    WE,
    NE,
    NW,
    SE,
    SW,
}

impl TryFrom<u8> for Tile {
    type Error = &'static str;
    fn try_from(c: u8) -> Result<Tile, Self::Error> {
        match c {
            b'.' => Ok(Tile::Ground),
            b'S' => Ok(Tile::Start),
            b'|' => Ok(Tile::NS),
            b'-' => Ok(Tile::WE),
            b'L' => Ok(Tile::NE),
            b'J' => Ok(Tile::NW),
            b'F' => Ok(Tile::SE),
            b'7' => Ok(Tile::SW),
            _ => Err("Unable to convert byte to tile")
        }
    }
}

impl Into<char> for Tile {
    fn into(self) -> char {
        match self {
            Tile::Ground => '.',
            Tile::Start => 'S',
            Tile::NS => '│',
            Tile::WE => '─',
            Tile::NE => '└',
            Tile::NW => '┘',
            Tile::SE => '┌',
            Tile::SW => '┐',
        }
    }
}

impl Tile {
    fn vertical_same(&self, o: &Tile) -> bool {
        match self {
            Tile::NS => match o {
                Tile::NS | Tile::NW | Tile::NE | Tile::SW | Tile::SE => true,
                _ => false,
            }
            Tile::NW | Tile::NE => match o {
                Tile::NS | Tile::NW | Tile::NE => true,
                _ => false
            },
            Tile::SW | Tile::SE => match o {
                Tile::NS | Tile::SW | Tile::SE => true,
                _ => false,
            },
            Tile::Ground | Tile::Start | Tile::WE => false,
        }
    }
}

struct Map {
    tsize: Vec2i,
    tiles: Vec<Tile>,
}

const ALL_DIRS: &[Vec2i] = &[
    make_vec2( 0, -1),
    make_vec2( 0,  1),
    make_vec2(-1,  0),
    make_vec2( 1,  0),
];

impl Map {
    fn index(&self, pos: Vec2i) -> usize {
        (self.tsize.x * pos.y + pos.x) as usize
    }
    fn pos(&self, i: usize) -> Vec2i {
        make_vec2(i as i32 % self.tsize.x, i as i32 / self.tsize.x)
    }
    fn tile(&self, pos: Vec2i) -> &Tile {
        &self.tiles[self.index(pos)]
    }
    fn _tile_mut(&mut self, pos: Vec2i) -> &mut Tile {
        let i = self.index(pos);
        &mut self.tiles[i]
    }
    fn inside(&self, pos: &Vec2i) -> bool {
        !pos.lt_any(make_vec2(0, 0)) &&
        !pos.gt_any(make_vec2(self.tsize.x - 1, self.tsize.y - 1))
    }

    fn find_start(&mut self) -> Vec2i {
        let start_index = self.tiles.iter().position(|&t| t == Tile::Start).unwrap();
        let start_pos = self.pos(start_index);
        let dirs: Vec<_> = ALL_DIRS.iter().enumerate()
            .filter_map(|(i, d)| {
                let pos = start_pos + *d;
                Some(i)
                .filter(|_| self.inside(&pos))
                .filter(|_| {
                    let found = self.adjacent(pos).find(|&f| f == start_pos);
                    found.is_some()
                })
            })
            .collect();
        let replace = match &dirs[..] {
            [0, 1] => Tile::NS,
            [2, 3] => Tile::WE,
            [0, 3] => Tile::NE,
            [0, 2] => Tile::NW,
            [1, 3] => Tile::SE,
            [1, 2] => Tile::SW,
            _ => panic!(),
        };
        self.tiles[start_index] = replace;
        start_pos
    }

    fn adjacent(&'_ self, pos: Vec2i) -> impl Iterator<Item = Vec2i> + '_ {
        let t = self.tile(pos);
        ALL_DIRS.iter().enumerate().filter_map(move |(i, d)| {
            Some(pos + *d)
            .filter(|p| self.inside(p))
            .filter(|p| {
                match t {
                    Tile::Start => {
                        // only if the other tile connects to start
                        let found = self.adjacent(*p).find(|&f| f == pos);
                        found.is_some()
                    },
                    Tile::Ground => false,
                    Tile::NS => [0, 1].contains(&i),
                    Tile::WE => [2, 3].contains(&i),
                    Tile::NE => [0, 3].contains(&i),
                    Tile::NW => [0, 2].contains(&i),
                    Tile::SE => [1, 3].contains(&i),
                    Tile::SW => [1, 2].contains(&i),
                }
            })
        })
    }

    fn _debug_tiles(&self) {
        let rows: Vec<_> = self.tiles.chunks(self.tsize.x as usize).into_iter()
            .map(|ch| -> Vec<char> {
                ch.iter().map(|t| -> char { (*t).into() }).collect()
            })
            .collect();
        rows.iter().filter_map(|r| {
            let s = String::from_iter(r.iter());
            if s.trim().is_empty() {
                None
            } else {
                Some(s)
            }
        }).for_each(|s| println!("{}", s));
        println!("");
    }
}

fn load_map(s: &str) -> Map {
    let row_size = s.bytes().position(|c| c == b'\n').unwrap() as i32;
    let map = Map {
        tsize: make_vec2(row_size, row_size),
        tiles: s.bytes()
            .filter_map(|c: u8| -> Option<Tile> { c.try_into().ok() })
            .collect(),
    };
    // map._debug_tiles();
    map
}

fn part_one(s: &str) -> Option<u64> {
    let map = load_map(s);

    let start_pos = map.pos(map.tiles.iter().position(|&t| t == Tile::Start).unwrap());
    let mut open: VecDeque<(Vec2i, u64)> = VecDeque::from([(start_pos, 0)]);
    let mut visited: Vec<Option<u64>> = vec![None; map.tiles.len()];
    while let Some((pos, cost)) = open.pop_front() {
        let was_visited = visited[map.index(pos)];
        if was_visited.is_some_and(|existing| existing <= cost) {
            continue;
        } else {
            visited[map.index(pos)] = Some(cost);
        }
        let adja: Vec<_> = map.adjacent(pos).collect();
        // println!("at {:?} with {:?}", pos, adja);
        match adja.as_slice() {
            &[one, two] => {
                open.push_back((one, cost + 1));
                open.push_back((two, cost + 1));
            },
            _ => ()
        }
    }
    // visited.chunks(map.tsize.x as usize)
    //     .for_each(|row| {
    //         row.iter().for_each(|c| if let Some(cost) = c {
    //             print!("{: >3}", cost)
    //         } else {
    //             print!("   ")
    //         });
    //         println!();
    //     });
    let max = *visited.iter().max().unwrap();
    max
}

fn part_two(s: &str) -> Option<u64> {
    let mut map = load_map(s);
    let row_size = map.tsize.x;

    let start_pos = map.find_start();
    let mut open: VecDeque<(Vec2i, i64, Vec2i)> = VecDeque::from([(start_pos, 1, make_vec2(0, 0))]);
    let mut visited: Vec<Option<i64>> = vec![None; map.tiles.len()];
    while let Some((pos, prev_winding, prev_corner)) = open.pop_front() {
        let tile = *map.tile(pos);
        let prev_tile = *map.tile(prev_corner);
        let is_same_row = pos.y == prev_corner.y;
        // println!("at {:?} with {:?}", pos, tile);
        let (winding, corner) = match tile {
            Tile::NS => (prev_winding, prev_corner),
            Tile::WE => (0, prev_corner),
            Tile::NE | Tile::NW | Tile::SW | Tile::SE => {
                if tile.vertical_same(&prev_tile) {
                    (-prev_winding, pos)
                } else {
                    (if is_same_row { 0 } else { prev_winding }, pos)
                }
            },
            Tile::Ground | Tile::Start => panic!(),
        };
        visited[map.index(pos)] = Some(winding);
        let next_winding = if winding != 0 { winding } else { prev_winding };
        let adja: Vec<_> = map.adjacent(pos).collect();
        if adja.len() == 2 {
            for p in adja {
                let was_visited = visited[map.index(p)];
                if was_visited.is_none() {
                    open.push_back((p, next_winding, corner));
                }
            }
        }
    }
    // debug view
    // visited.chunks(row_size as usize)
    //     .for_each(|row| {
    //         let s: String = row.iter().map(|c| match c {
    //             Some(1) => "↑",
    //             Some(-1) => "↓",
    //             Some(0) => "0",
    //             _ => ".",
    //         }).collect();
    //         println!("{s}");
    //     });
    let count = visited.iter().enumerate()
        .filter(|&(i, v)| {
            // only check points not part of the loop
            if v.is_none() {
                let row_end = ((i as i32 / row_size) + 1) * row_size;
                let winding: i64 = visited[i..row_end as usize].iter()
                    .fold(0, |a, c| {
                        match c {
                            Some(x) => a + x,
                            None => a,
                        }
                    });
                winding != 0
            } else {
                false
            }
        })
        .count();
    Some(count as u64)
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
        assert_eq!(part_one(EXAMPLE_STR), Some(8));
    }

    const EXAMPLE_STR_TWO: &'static str =
&"FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L";

    #[test]
    fn test_part_two() {
        assert_eq!(part_two(EXAMPLE_STR_TWO), Some(10));
    }
}