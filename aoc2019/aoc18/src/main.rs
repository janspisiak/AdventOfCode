use std::{fs, collections::{VecDeque, HashMap, BinaryHeap}};

use bitvec::{prelude::*, view::BitView};
use euclid::{Vector2D, UnknownUnit};
use itertools::Itertools;

type Vec2i = Vector2D<i32, UnknownUnit>;
const fn vec2i(x: i32, y: i32) -> Vec2i {
    return Vec2i::new( x, y );
}

trait VectorRotate {
    fn rotate_left(&self) -> Self;
    fn rotate_right(&self) -> Self;
}

impl VectorRotate for Vec2i {
    fn rotate_left(&self) -> Vec2i {
        vec2i(self.y, -self.x)
    }
    fn rotate_right(&self) -> Vec2i {
        vec2i(-self.y, self.x)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    Path,
    Wall,
    Key(char),
    Gate(char)
}

impl From<char> for Tiles {
    fn from(ch: char) -> Self {
        match ch {
            '#' => Tiles::Wall,
            '.' => Tiles::Path,
            'a'..='z' | '@' | '0'..='9' => Tiles::Key(ch.to_ascii_uppercase()),
            'A'..='Z' => Tiles::Gate(ch),
            _ => panic!("Unknown tile")
        }
    }
}

impl Into<char> for Tiles {
    fn into(self) -> char {
        match self {
            Self::Path => '.',
            Self::Wall => '#',
            Self::Key(ch) => ch.to_ascii_lowercase(),
            Tiles::Gate(ch) => ch,
        }
    }
}

struct Map {
    tsize: Vec2i,
    tiles: Vec<Tiles>,
}

const ALL_DIRS: &[Vec2i] = &[
    vec2i( 0, -1),
    vec2i( 0,  1),
    vec2i(-1,  0),
    vec2i( 1,  0),
];

fn inside_size(size: Vec2i) -> impl Fn(&Vector2D<i32, UnknownUnit>) -> bool {
    move |pos: &Vec2i| {
        !pos.lower_than(vec2i(0, 0)).any() &&
        !pos.greater_than(vec2i(size.x - 1, size.y - 1)).any()
    }
}

impl Map {
    fn tile(&self, pos: Vec2i) -> &Tiles {
        &self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn tile_mut(&mut self, pos: Vec2i) -> &mut Tiles {
        &mut self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn shrink_tiles(&mut self) {
        self.tiles.truncate((self.tsize.x * self.tsize.y) as usize);
    }

    fn path_cross(&self) -> Vec<Vec2i> {
        self.tiles.iter()
            .enumerate()
            .filter_map(|(i, &t)| {
                if t == Tiles::Path {
                    let pos = vec2i(i as i32 % self.tsize.x, i as i32 / self.tsize.x);
                    let is_cross = |&pos: &Vec2i| {
                        ALL_DIRS.iter()
                            .filter_map(move |&d| {
                                Some(pos + d).filter(inside_size(self.tsize))
                            })
                            .all(|p| *self.tile(p) == Tiles::Path)
                    };
                    Some(pos).filter(is_cross)
                } else {
                    None
                }
            })
            .collect()
    }

    fn shortest_path(&self, start: Vec2i, end: Vec2i) -> (usize, Vec<char>) {
        // BFS and collect gates
        let mut open = VecDeque::from([start]);
        let mut visited = HashMap::from([(start, start)]);
        while !open.is_empty() {
            let pos = open.pop_front().unwrap();
            if pos == end {
                break;
            }
            let adjacent = ALL_DIRS.iter()
                .filter_map(|&d| {
                    Some(pos + d)
                        .filter(inside_size(self.tsize))
                        .filter(|&p| *self.tile(p) != Tiles::Wall)
                });
            for a in adjacent {
                if visited.get(&a).is_none() {
                    open.push_back(a);
                    visited.insert(a, pos);
                }
            }
        }

        let mut gates = vec![];
        let mut path = vec![];
        let mut parent = visited.get(&end);
        while let Some(&p) = parent {
            parent = visited.get(&p);
            path.push(p);
            if parent.is_some_and(|&pr| pr == p) {
                break;
            }
            if let Tiles::Gate(g) = *self.tile(p) {
                gates.push(g);
            }
        }
        (path.len(), gates)
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

type GateMask = u32;

fn part_one(keys: &Vec<(Vec2i, char)>, key_matrix: &Vec<Vec<(u32, u32)>>) {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HeapItem {
        cost: u32,
        last: u32,
        taken: GateMask,
    }
    impl PartialOrd for HeapItem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&self.cost))
        }
    }
    impl Ord for HeapItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    // we want to cull states where we arrive at key X with collected keys Ks
    // but we already visited this key with lower cost
    // first "key" is always '@'
    let init_state = HeapItem{ cost: 0, last: 0, taken: 0b1 };
    let mut key_paths: BinaryHeap<HeapItem> = BinaryHeap::from([ init_state ]);
    let mut shortest = u32::MAX;
    let mut optimal: HashMap<(u32, u32), u32> = HashMap::new();
    let keys_mask = u32::MAX >> (32_u32 - keys.len() as u32);
    while !key_paths.is_empty() {
        let item = key_paths.pop().unwrap();
        // println!("path {:016b} score {}", item.taken, item.cost);
        if item.taken & keys_mask == keys_mask {
            println!("path {:016b} score {}", item.taken, item.cost);
            shortest = shortest.min(item.cost);
            break;
        }
        let optimal_key = (item.last, item.taken);
        let lowest = *optimal.get(&optimal_key).unwrap_or(&u32::MAX);
        if lowest <= item.cost {
            continue;
        } else {
            optimal.insert(optimal_key, item.cost);
        }

        let taken_bits = item.taken.view_bits::<Lsb0>();
        key_paths.extend(key_matrix[item.last as usize].iter()
            .enumerate()
            .filter_map(|(key, &(key_cost, gate_mask))| {
                if key_cost > 0 // traversible
                    && !taken_bits[key] // not taken yet
                    && (gate_mask & item.taken == gate_mask) // has all keys
                {
                    let mut new_taken = item.taken;
                    new_taken.view_bits_mut::<Lsb0>().set(key, true);
                    Some(HeapItem {
                        cost: key_cost + item.cost,
                        last: key as u32,
                        taken: new_taken,
                    })
                } else {
                    None
                }
            })
        );
    }
    println!("shortest {}", shortest);
}

fn part_two(keys: &Vec<(Vec2i, char)>, key_matrix: &Vec<Vec<(u32, u32)>>) {
    #[derive(Debug, Clone, PartialEq, Eq)]
    struct HeapItem {
        cost: u32,
        robots: [u32; 4],
        taken: GateMask,
    }
    impl PartialOrd for HeapItem {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(other.cost.cmp(&self.cost))
        }
    }
    impl Ord for HeapItem {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            other.cost.cmp(&self.cost)
        }
    }

    // we want to cull states where we arrive at key X with collected keys Ks
    // but we already visited this key with lower cost
    // first "key" is always '@'
    let init_state = HeapItem{ cost: 0, robots: [0, 1, 2, 3], taken: 0b1111 };
    let mut key_paths: BinaryHeap<HeapItem> = BinaryHeap::from([ init_state ]);
    let mut shortest = u32::MAX;
    let mut optimal: HashMap<([u32; 4], u32), u32> = HashMap::new();
    let keys_mask = u32::MAX >> (32_u32 - keys.len() as u32);
    println!("keys_mask {:b} {:b}", keys_mask, key_paths.peek().unwrap().taken);
    while !key_paths.is_empty() {
        let item = key_paths.pop().unwrap();
        // println!("path {:b} score {} robots {:?}", item.taken, item.cost, item.robots);
        if item.taken & keys_mask == keys_mask {
            println!("path     {:b} score {}", item.taken, item.cost);
            shortest = shortest.min(item.cost);
            break;
        }

        // check if this state is optimal so far
        let optimal_key = (item.robots, item.taken);
        let lowest = *optimal.get(&optimal_key).unwrap_or(&u32::MAX);
        if lowest <= item.cost {
            continue;
        } else {
            optimal.insert(optimal_key, item.cost);
        }

        for (robot, &last_pos) in item.robots.iter().enumerate() {
            let taken_bits = item.taken.view_bits::<Lsb0>();
            key_paths.extend(key_matrix[last_pos as usize].iter()
                .enumerate()
                .filter_map(|(key, &(key_cost, gate_mask))| {
                    if key_cost > 0 // traversible
                        && !taken_bits[key] // not taken yet
                        && (gate_mask & item.taken == gate_mask) // has all keys
                    {
                        let mut new_taken = item.taken;
                        new_taken.view_bits_mut::<Lsb0>().set(key, true);
                        let mut new_robots = item.robots;
                        new_robots[robot] = key as u32;
                        Some(HeapItem {
                            cost: key_cost + item.cost,
                            robots: new_robots,
                            taken: new_taken,
                        })
                    } else {
                        None
                    }
                })
            );
        }
    }
    println!("shortest {}", shortest);
}


fn main() {
    let input_path = "aoc18/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let row_size = input_str.find('\n').unwrap() as i32;
    let row_height = input_str.lines().count() as i32;

    let mut map = Map {
        tsize: vec2i(row_size, row_height),
        tiles: input_str.lines()
            .flat_map(|l| l.chars().map(|ch| Tiles::from(ch)))
            .collect(),
    };

    map._debug_tiles();

    let mut keys: Vec<_> = map.tiles.iter()
        .enumerate()
        .filter_map(|(i, &t)| {
            let pos = vec2i(i as i32 % map.tsize.x, i as i32 / map.tsize.x);
            match t {
                Tiles::Key(k) => Some((pos, k)),
                _ => None
            }
        })
        .collect();
    keys.sort_by_key(|v| v.1);

    let mut key_matrix = vec![vec![(0u32, 0u32); keys.len()]; keys.len()];
    for ks in keys.iter().map(|(pos, _)| pos).enumerate().combinations(2) {
        if let [(si, &start), (ei, &end)] = ks[0..2] {
            let (path_len, gates) = map.shortest_path(start, end);
            let gate_indices: GateMask = gates.into_iter()
                .map(|g| keys.iter()
                    .find_position(|k| k.1 == g).unwrap().0)
                .fold(0, |mut a, g| {
                    a.view_bits_mut::<Lsb0>().set(g, true);
                    a
                });
            key_matrix[si][ei] = (path_len as u32, gate_indices);
            key_matrix[ei][si] = (path_len as u32, gate_indices);
        }
    }

    println!("    {}", keys.iter().map(|k| format!("{: >3}", k.1)).join(" "));
    key_matrix.iter().enumerate().for_each(|(i, k)| {
        println!("{: >3} {}", keys[i].1, k.iter()
            .map(|&(p, _)| if p > 0 { format!("{: >3}", p) } else { format!("{: >3}", ' ')})
            .join(" "));
        // println!("{: >3} {}", keys[i].1, k.iter().map(|(_, g)| format!("{: >3}", g.len())).join(" "));
    });

    // 5198
    part_one(&keys, &key_matrix);
    // part_two(&keys, &key_matrix);
}
