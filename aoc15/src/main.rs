use core::panic;
use std::{collections::{VecDeque, HashMap, BinaryHeap}, fs};
use euclid::{Vector2D, UnknownUnit, vec2};

use crate::int::{Computer, ComputerState};
mod int;

type Vec2i = Vector2D<i32, UnknownUnit>;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Dir {
    North,
    South,
    West,
    East,
}

impl Dir {
    fn iter_all() -> impl Iterator<Item = Dir> {
        (1..5).into_iter().map(|x| Dir::from(x))
    }
}

impl From<Vec2i> for Dir {
    fn from(v: Vec2i) -> Self {
        match v {
            Vec2i { x:  0, y: -1, .. } => Dir::North,
            Vec2i { x:  0, y:  1, .. } => Dir::South,
            Vec2i { x: -1, y:  0, .. } => Dir::West,
            Vec2i { x:  1, y:  0, .. } => Dir::East,
            _ => panic!("Wrong direction"),
        }
    }
}

impl From<i64> for Dir {
    fn from(value: i64) -> Self {
        match value {
            1 => Dir::North,
            2 => Dir::South,
            3 => Dir::West,
            4 => Dir::East,
            _ => panic!("Wrong direction"),
        }
    }
}

impl From<Dir> for i64 {
    fn from(d: Dir) -> i64 {
        match d {
            Dir::North => 1,
            Dir::South => 2,
            Dir::West => 3,
            Dir::East => 4,
        }
    }
}

impl From<Dir> for Vec2i {
    fn from(dir: Dir) -> Self {
        match dir {
            Dir::North => vec2( 0, -1),
            Dir::South => vec2( 0,  1),
            Dir::West  => vec2(-1,  0),
            Dir::East  => vec2( 1,  0),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Unknown,
    Wall,
    Empty,
    Oxygen,
}

impl From<i64> for Tiles {
    fn from(value: i64) -> Self {
        match value {
            0 => Tiles::Wall,
            1 => Tiles::Empty,
            2 => Tiles::Oxygen,
            _ => panic!("Unknown tile")
        }
    }
}

impl Into<char> for Tiles {
    fn into(self) -> char {
        match self {
            Self::Unknown => '.',
            Self::Wall => '#',
            Self::Empty => ' ',
            Self::Oxygen => 'O',
        }
    }
}

struct Map {
    tiles_w: usize,
    tiles: Vec<Tiles>,
}

impl Map {
    fn tile(&self, pos: Vec2i) -> &Tiles {
        &self.tiles[(self.tiles_w as i32 * pos.y + pos.x) as usize]
    }
    fn tile_mut(&mut self, pos: Vec2i) -> &mut Tiles {
        &mut self.tiles[(self.tiles_w as i32 * pos.y + pos.x) as usize]
    }

    fn debug_tiles(&self, start: Vec2i, path_to: &Vec<Dir>) {
        let mut rows: Vec<_> = self.tiles.chunks(self.tiles_w as usize).into_iter()
            .map(|ch| -> Vec<char> {
                ch.iter().map(|&t| -> char { t.into() }).collect()
            })
            .collect();
        let mut pos = start;
        path_to.iter()
            .for_each(|&d| {
                let ch = match d {
                    Dir::North => '↑',
                    Dir::South => '↓',
                    Dir::West => '←',
                    Dir::East => '→',
                };
                rows[pos.y as usize][pos.x as usize] = ch;
                pos += Vec2i::from(d);
            }
        );
        rows[start.y as usize][start.x as usize] = 'S';
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

    fn path_to_nearest(&self, from: Vec2i, test: Tiles, out: &mut Vec<Dir>) {
        let mut queue: VecDeque<Vec2i> = VecDeque::from([from]);
        let mut traversed: HashMap<Vec2i, Vec2i> = HashMap::from([(from, from)]);
        let mut to_pos: Option<Vec2i> = None;
        'queue: while let Some(parent_pos) = queue.pop_front() {
            for dir in Dir::iter_all() {
                let tile_pos: Vec2i = parent_pos + Vec2i::from(dir);
                let tile = *self.tile(tile_pos);
                let existing = traversed.get(&tile_pos);
                if tile == Tiles::Wall {
                    continue;
                } else if existing.is_none() {
                    traversed.entry(tile_pos).or_insert(parent_pos);
                    if tile == test {
                        to_pos = Some(tile_pos);
                        break 'queue;
                    }
                    queue.push_back(tile_pos);
                }
            }
        }
        out.clear();
        // backtract directions and fill output
        while let Some(to) = to_pos {
            let parent = traversed[&to];
            if to == parent {
                break;
            }
            out.push(Dir::from(to - parent));
            to_pos = Some(parent);
        }
    }

    // reverse path_to
    fn rpath_to(&self, orig: Vec2i, dest: Vec2i, out: &mut Vec<Dir>, debug: bool) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        struct HeapItem {
            prio: i64,
            pos: Vec2i,
        }
        impl PartialOrd for HeapItem {
            fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
                Some(other.prio.cmp(&self.prio))
            }
        }
        impl Ord for HeapItem {
            fn cmp(&self, other: &Self) -> std::cmp::Ordering {
                other.prio.cmp(&self.prio)
            }
        }

        let _debug_traversed = |traversed: &HashMap<Vec2i, (Vec2i, i64)>| {
            println!("Traversed length {} orig {:?} dest {:?}", traversed.len(), orig, dest);
            let mut rows: Vec<_> = self.tiles.chunks(self.tiles_w as usize).into_iter()
                .map(|ch| -> Vec<char> {
                    ch.iter().map(|&t| -> char { t.into() }).collect()
                })
                .collect();
            traversed.iter().for_each(|(&pos, &(par, _))| {
                if (pos - par).square_length() == 1 {
                    let ch = match Dir::from(pos - par) {
                        Dir::North => '↑',
                        Dir::South => '↓',
                        Dir::West => '←',
                        Dir::East => '→',
                    };
                    rows[pos.y as usize][pos.x as usize] = ch;
                }
            });
            rows[orig.y as usize][orig.x as usize] = 'S';
            rows[dest.y as usize][dest.x as usize] = 'E';
            rows.iter().filter_map(|r| {
                let s = String::from_iter(r.iter());
                if s.trim().is_empty() {
                    None
                } else {
                    Some(s)
                }
            }).for_each(|s| println!("{}", s));
            println!("");
        };

        let mut queue: BinaryHeap<HeapItem> = BinaryHeap::from([HeapItem{ prio: 0, pos: orig }]);
        let mut traversed: HashMap<Vec2i, (Vec2i, i64)> = HashMap::from([(orig, (orig, 0))]);
        let tiles_lower: Vec2i = vec2(1, 1);
        let tiles_upper: Vec2i = vec2(self.tiles_w as i32 - 2, self.tiles_w as i32 - 2);
        while let Some(parent) = queue.pop() {
            if parent.pos == dest {
                break;
            }
            let parent_cost = traversed[&parent.pos].1;
            for dir in Dir::iter_all() {
                let tile_pos: Vec2i = parent.pos + Vec2i::from(dir);
                let tile = *self.tile(tile_pos);
                let new_cost = parent_cost + 1;
                let new_traversed = traversed.get(&tile_pos);
                // check for walls and map edges
                if tile == Tiles::Wall
                    || tile_pos.lower_than(tiles_lower).any()
                    || tile_pos.greater_than(tiles_upper).any() {
                    continue;
                } else if new_traversed.is_none() || new_cost < new_traversed.unwrap().1  {
                    traversed.insert(tile_pos, (parent.pos, new_cost));
                    let dest_dir = (dest - tile_pos).abs().to_i64();
                    let tile_prio = new_cost + dest_dir.x + dest_dir.y;
                    queue.push(HeapItem{ prio: tile_prio, pos: tile_pos });
                }
            }
        }

        out.clear();
        if debug {
            _debug_traversed(&traversed);
        }
        // backtract directions and fill output
        let mut to_pos = Some(dest);
        while let Some(to) = to_pos {
            let maybe_from = traversed.get(&to);
            if let Some(&from) = maybe_from {
                if to == from.0 {
                    break;
                }
                out.push(Dir::from(to - from.0));
                to_pos = Some(from.0);
            } else {
                break;
            }
        }
    }

    fn fill_from(&self, orig: Vec2i) -> (Vec2i, i64) {
        let mut queue: VecDeque<(Vec2i, i64)> = VecDeque::from([(orig, 0)]);
        let mut tiles = self.tiles.clone();
        let mut last = *queue.front().unwrap();
        while let Some((pos, depth)) = queue.pop_front() {
            last = (pos, depth);
            tiles[(self.tiles_w as i32 * pos.y + pos.x) as usize] = Tiles::Oxygen;
            for dir in Dir::iter_all() {
                let tile_pos: Vec2i = pos + Vec2i::from(dir);
                let tile = tiles[(self.tiles_w as i32 * tile_pos.y + tile_pos.x) as usize];
                if tile == Tiles::Empty {
                    queue.push_back((tile_pos, depth + 1));
                }
            }
        }
        last
    }
}

fn main() {
    // let args: Vec<String> = env::args().collect();
    let prog_path = "aoc15/prog.txt";
    // println!("Prog at {}, input at {}", prog_path, input_path);

    let prog_str = fs::read_to_string(prog_path).expect("Something went wrong reading the file");

    let orig_prog: Vec<i64> = prog_str
        .replace("\n", "")
        .split(',')
        .map(|line| line.parse::<i64>().unwrap())
        .collect();

    let mut comp_mem = orig_prog.clone();
    comp_mem.resize(8000, 0);
    let mut comp = Computer {
        eip: 0,
        esp: 0,
        mem: comp_mem,
        ins: VecDeque::from([]),
        outs: VecDeque::new(),
    };

    let tiles_w = 60usize;
    let mut map = Map {
        tiles_w,
        tiles: vec![Tiles::Unknown; tiles_w * tiles_w],
    };

    let start_pos: Vec2i = vec2(tiles_w as i32 / 2, tiles_w as i32 / 2);
    let mut robot_pos: Vec2i = start_pos;
    let mut path_to: Vec<Dir> = vec![Dir::North];
    let mut oxygen_pos: Option<Vec2i> = None;
    let mut state = ComputerState::Interrupt;
    let mut counter = 3000; // loop stop
    while ComputerState::Interrupt == state && counter > 0 {
        counter -= 1;
        let maybe_robot_dir = path_to.pop();
        // If we don't have any path - break
        // should end once we have the whole map explored
        if maybe_robot_dir.is_none() {
            break;
        }
        let robot_dir = maybe_robot_dir.unwrap();
        comp.ins.push_back(i64::from(robot_dir));
        state = comp.run().unwrap();
        if comp.outs.len() >= 1 {
            let hit = Tiles::from(comp.outs.pop_front().unwrap());
            // println!("[{}, {}] went {:?} hit {:?}", robot_pos.x, robot_pos.y, robot_dir, hit);
            match hit {
                Tiles::Wall => {
                    let wall_pos = robot_pos + Vec2i::from(robot_dir);
                    *map.tile_mut(wall_pos) = Tiles::Wall;
                    path_to.clear();
                }
                Tiles::Empty => {
                    robot_pos += Vec2i::from(robot_dir);
                    *map.tile_mut(robot_pos) = Tiles::Empty;
                }
                Tiles::Oxygen => {
                    robot_pos += Vec2i::from(robot_dir);
                    *map.tile_mut(robot_pos) = Tiles::Oxygen;
                    oxygen_pos = Some(robot_pos);
                    // we might not have the full map even though we found the oxygen
                }
                _ => panic!("Unknown output")
            }

            if path_to.is_empty() {
                map.path_to_nearest(robot_pos, Tiles::Unknown, &mut path_to);
            }
        }
    }
    map.debug_tiles(robot_pos, &path_to);
    println!("oxygen at {:?} after {} steps", oxygen_pos, counter);
    if let Some(oxy) = oxygen_pos {
        map.rpath_to(start_pos, oxy, &mut path_to, true);
        path_to.reverse();
        map.debug_tiles(start_pos, &path_to);
        println!("shortest path to oxygen {}", path_to.len());
        // oxygen fill
        let last = map.fill_from(oxy);
        println!("fill steps {}", last.1);
    }


}
