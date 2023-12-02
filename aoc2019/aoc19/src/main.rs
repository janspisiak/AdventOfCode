use core::panic;
use std::{collections::VecDeque, fs, ops::Range};
use euclid::{Vector2D, UnknownUnit, vec2};

use crate::int::{Computer, ComputerState};
mod int;

type Vec2i = Vector2D<i32, UnknownUnit>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Beam,
    None,
}

impl From<u8> for Tiles {
    fn from(value: u8) -> Self {
        match value {
            0 => Tiles::None,
            1 => Tiles::Beam,
            _ => panic!("Unknown tile")
        }
    }
}

impl Into<char> for Tiles {
    fn into(self) -> char {
        match self {
            Self::None => '.',
            Self::Beam => '#',
        }
    }
}

struct Map {
    tsize: Vec2i,
    tiles: Vec<Tiles>,
}

impl Map {
    fn tile_mut(&mut self, pos: Vec2i) -> &mut Tiles {
        &mut self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn shrink_tiles(&mut self) {
        self.tiles.truncate((self.tsize.x * self.tsize.y) as usize);
    }

    fn _debug_tiles(&self) {
        let rows: Vec<_> = self.tiles.chunks(self.tsize.x as usize).into_iter()
            .map(|ch| -> Vec<char> {
                ch.iter().map(|&t| -> char { t.into() }).collect()
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

fn part_one(orig_prog: &Vec<i64>) {
    let mut map = Map {
        tsize: vec2(50, 50),
        tiles: vec![Tiles::None; 50 * 50],
    };

    let mut map_pos: Vec2i = vec2(0, 0);
    let mut state = ComputerState::Interrupt;
    let mut tile_counter = 0;
    while map_pos.y < map.tsize.y
    {
        let mut comp = Computer {
            eip: 0,
            esp: 0,
            mem: orig_prog.clone(),
            ins: VecDeque::from([map_pos.x as i64, map_pos.y as i64]),
            outs: VecDeque::new(),
        };

        state = comp.run().unwrap();
        if let Some(out) = comp.outs.pop_front() {
            let out_tile = match out {
                0 => Tiles::None,
                1 => Tiles::Beam,
                _ => panic!("Unknown program output")
            };
            // println!("{:?} found {:?}", map_pos, out_tile);
            let map_tile = map.tile_mut(map_pos);
            *map_tile = out_tile;
            tile_counter += 1;
            map_pos.x = tile_counter % map.tsize.x;
            map_pos.y = tile_counter / map.tsize.x;
        } else {
            // println!("No output");
            break;
        }
    }
    println!("End at {:?} with {:?}", map_pos, state);
    map.shrink_tiles();
    map._debug_tiles();
    let beam_tiles = map.tiles.iter().filter(|&&t| t == Tiles::Beam).count();
    println!("Beam tiles {}", beam_tiles);
}

fn part_two(orig_prog: &Vec<i64>, rect_size: i32) {
    fn compute_beam(orig_prog: &Vec<i64>, pos: Vec2i) -> Option<i64> {
        let mut comp = Computer {
            eip: 0,
            esp: 0,
            mem: orig_prog.clone(),
            ins: VecDeque::from([pos.x as i64, pos.y as i64]),
            outs: VecDeque::new(),
        };

        comp.run().unwrap();
        comp.outs.pop_front()
    }

    let mut map_pos: Vec2i = vec2(0, 0);
    let mut range_q: VecDeque<Option<Range<usize>>> = VecDeque::new();
    loop {
        let mut maybe_range_start = None;
        let mut range_length = 1;
        let range = loop {
            let out = compute_beam(orig_prog, map_pos).unwrap();
            let tile = match out {
                0 => Tiles::None,
                1 => Tiles::Beam,
                _ => panic!("Unknown program output")
            };
            if let Some(range_start) = maybe_range_start {
                match tile {
                    Tiles::Beam => range_length += 1,
                    Tiles::None => break Some(range_start..range_start + range_length),
                }
            } else {
                match tile {
                    Tiles::Beam => maybe_range_start = Some(map_pos.x as usize),
                    Tiles::None => {
                        // some random heuristic to stop looking
                        if map_pos.x > map_pos.y * rect_size {
                            break None;
                        }
                    },
                }
            }
            map_pos.x += 1;
        };
        map_pos.x = range.clone().map_or(0, |r| r.start as i32);
        map_pos.y += 1;

        range_q.push_back(range.clone());
        if range_q.len() >= rect_size as usize {
            if let Some(first) = range_q.pop_front().unwrap() {
                let last = range.clone().unwrap();
                if first.end as i32 - last.start as i32 >= rect_size {
                    let left_corner: Vec2i = vec2(first.end as i32 - rect_size, map_pos.y - rect_size);
                    println!("{:?} {:?} {:?}", first, last, left_corner);
                    println!("Result {}", left_corner.x * 10000 + left_corner.y);
                    break;
                }
            }
        }
    }
}

fn main() {
    let prog_path = "aoc19/prog.txt";
    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let mut orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();
    orig_prog.resize(8000, 0);

    // part_one(&orig_prog);
    part_two(&orig_prog, 100)
}
