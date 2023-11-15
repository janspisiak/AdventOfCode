use std::{collections::VecDeque, fs, ops::Neg};
use euclid::{Vector2D, UnknownUnit, vec2};
use itertools::Itertools;

use crate::int::{Computer, ComputerState};
mod int;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
enum TileType {
    #[default] Empty,
    Wall,
    Block,
    HPaddle,
    Ball,
}

impl Into<char> for TileType {
    fn into(self) -> char {
        match self {
            Self::Empty => ' ',
            Self::Wall => 'W',
            Self::Block => 'B',
            Self::HPaddle => '_',
            Self::Ball => 'O',
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Default)]
struct Tile {
    pos: Vector2D<i64, UnknownUnit>,
    content: TileType,
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let prog_path = "aoc13/prog.txt";
    // println!("Prog at {}, input at {}", prog_path, input_path);

    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut comp_mem = orig_prog.clone();
    comp_mem.resize(8000, 0);
    // adjust arcade money in memory
    comp_mem[0] = 2;
    let mut comp = Computer {
        eip: 0,
        esp: 0,
        mem: comp_mem,
        ins: VecDeque::from([0]),
        outs: VecDeque::new(),
    };

    let map_width = 35i64;
    let map_heigth: i64 = 23i64;
    let mut tiles: Vec<Tile> = vec![Tile::default(); (map_width * map_heigth) as usize];
    let mut block_counter = 0;
    let mut ball: Vector2D<i64, UnknownUnit> = vec2(0, 0);
    let mut paddle: Vector2D<i64, UnknownUnit> = vec2(0, 0);
    let mut score = 0;
    let output_ring_max = 12;
    let mut output_ring: VecDeque<Tile> = VecDeque::with_capacity(output_ring_max);
    let debug = |tiles: &Vec<Tile>, output_ring: &VecDeque<Tile>| {
        let mut tiles_clone = tiles.clone();
        output_ring.iter().for_each(|t| {
            let tile = &mut tiles_clone[(t.pos.y * map_width + t.pos.x) as usize];
            tile.content = match t.content {
                TileType::Ball => t.content,
                TileType::Empty => TileType::Wall,
                _ => tile.content
            };
            // println!("[{}, {}] {:?}", t.pos.x, t.pos.y, t.content)
        });
        tiles_clone.chunks(map_width as usize).into_iter().enumerate().for_each(|(i, ch)| {
            let chars: String = ch.iter().map(|t| -> char { t.content.into() }).collect();
            println!("{: >2} {}", i, chars);
        });
    };

    let mut state = ComputerState::Interrupt;
    while ComputerState::Interrupt == state && comp.outs.len() < 3 {
        state = comp.run().unwrap();
        if comp.outs.len() >= 3 {
            let (x, y, t) = comp.outs.drain(..3).collect_tuple().unwrap();
            if x == -1 && y == 0 {
                score = t;
                continue;
            }

            let tile_content: TileType = match t {
                0 => TileType::Empty,
                1 => TileType::Wall,
                2 => TileType::Block,
                3 => TileType::HPaddle,
                4 => TileType::Ball,
                _ => panic!("Unknown tile")
            };
            let tile_index = (y * map_width + x) as usize;
            tiles[tile_index] = Tile {
                pos: vec2(x, y),
                content: tile_content,
            };
            if output_ring.len() >= output_ring_max {
                output_ring.pop_front();
            }
            output_ring.push_back(tiles[tile_index]);

            match tile_content {
                TileType::Block => block_counter += 1,
                TileType::Ball => {
                    ball = vec2(x, y);
                }
                TileType::HPaddle => {
                    paddle = vec2(x, y);
                }
                _ => ()
            }

            if paddle != vec2(0, 0) && tile_content == TileType::Ball {
                let dir: i64 = (ball.x - paddle.x).signum();
                if comp.ins.is_empty() {
                    comp.ins.push_back(dir);
                } else {
                    let front = comp.ins.front_mut().unwrap();
                    *front = dir;
                }
            }
        }
    }

    println!("block tiles {} score {} state {:?}", block_counter, score, state);
    debug(&tiles, &output_ring);
}
