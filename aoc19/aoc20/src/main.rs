use std::{fs, collections::{VecDeque, HashMap, BinaryHeap}};

use euclid::{Vector2D, UnknownUnit};
use itertools::Itertools;

type Vec2i = Vector2D<i32, UnknownUnit>;
const fn vec2i(x: i32, y: i32) -> Vec2i {
    return Vec2i::new( x, y );
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tiles {
    None,
    Path,
    Wall,
    Label(char),
    Portal(usize, bool),
}

impl From<char> for Tiles {
    fn from(ch: char) -> Self {
        match ch {
            ' ' => Tiles::None,
            '#' => Tiles::Wall,
            '.' => Tiles::Path,
            'A'..='Z' => Tiles::Label(ch),
            _ => panic!("Unknown tile")
        }
    }
}

impl Into<char> for Tiles {
    fn into(self) -> char {
        match self {
            Self::None => ' ',
            Self::Path => '.',
            Self::Wall => '#',
            Self::Label(ch) => ch,
            Self::Portal(i, _) => char::from(u8::from(b'A') + i as u8),
        }
    }
}

struct Portal {
    label: String,
    pos: Vec2i,
    path: Vec2i,
    dest: usize,
    is_inner: bool,
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

fn inside_size_offset(size: Vec2i, offset: i32) -> impl Fn(&Vector2D<i32, UnknownUnit>) -> bool {
    move |pos: &Vec2i| {
        !pos.lower_than(vec2i(offset, offset)).any() &&
        !pos.greater_than(vec2i(size.x - 1 - offset, size.y - 1 - offset)).any()
    }
}

impl Map {
    fn tile(&self, pos: Vec2i) -> &Tiles {
        &self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
    }
    fn tile_mut(&mut self, pos: Vec2i) -> &mut Tiles {
        &mut self.tiles[(self.tsize.x * pos.y + pos.x) as usize]
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

fn part_one(portals: Vec<Portal>, dist_matrix: Vec<Vec<(u32, i32)>>) {
    let (start_pi, _) = portals.iter().find_position(|p| p.label == "AA").unwrap();
    let (end_pi, _) = portals.iter().find_position(|p| p.label == "ZZ").unwrap();

    // optimal path djikstra
    let mut costs = vec![u32::MAX; portals.len()];
    costs[start_pi] = 0;
    let mut not_taken: Vec<_> = (0..portals.len()).collect();

    while let Some((pi, _)) = costs.iter()
        .enumerate()
        .filter(|(i, _)| not_taken.contains(i))
        .min_by_key(|&(_, cost)| cost)
    {
        let this_cost = costs[pi];
        // println!("portal {} cost {} not_taken {:?}", portals[pi].label, this_cost, not_taken);
        for (di, &(path_cost, _)) in dist_matrix[pi].iter()
            .enumerate()
            .filter(|&(_, &(c, _))| c > 0)
        {
            costs[di] = costs[di].min(this_cost + path_cost);
        }
        // println!("{:?}", costs);
        not_taken.retain(|&v| v != pi);
    }
    println!("end_cost {}", costs[end_pi]);
}

fn part_two(portals: Vec<Portal>, dist_matrix: Vec<Vec<(u32, i32)>>) {
    let (start_pi, _) = portals.iter().find_position(|p| p.label == "AA").unwrap();
    let (end_pi, _) = portals.iter().find_position(|p| p.label == "ZZ").unwrap();

    #[derive(Debug, Default, Clone, PartialEq, Eq)]
    struct HeapItem {
        cost: u32,
        pos: u32,
        level: i32,
        prev: u32,
        plevel: i32,
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
    #[derive(Debug, Default, Copy, Clone, PartialEq, Eq)]
    struct PathItem {
        cost: u32,
        parent: u32,
        plevel: i32,
    }

    let init_state = HeapItem{ pos: start_pi as u32, cost: 0, level: 0, prev: start_pi as u32, plevel: 0 };
    let mut open: BinaryHeap<HeapItem> = BinaryHeap::from([ init_state ]);
    let mut optimal: HashMap<(u32, i32), PathItem> = HashMap::new();
    let mut shortest = u32::MAX;
    while !open.is_empty() {
        let item = open.pop().unwrap();

        // check if this state is optimal so far
        let optimal_key = (item.pos, item.level);
        let lowest = optimal.get(&optimal_key);
        if lowest.is_some_and(|i| i.cost <= item.cost) {
            continue;
        } else {
            optimal.insert(optimal_key, PathItem{
                cost: item.cost,
                parent: item.prev,
                plevel: item.plevel
            });
        }

        // println!("{}:{} with {}", portals[item.pos as usize].label, item.level, item.cost);

        // check end state
        if item.pos == end_pi as u32 && item.level == 0 {
            println!("shortest path {}", item.cost);
            shortest = item.cost;
            break;
        }

        open.extend(dist_matrix[item.pos as usize].iter()
            .enumerate()
            .filter_map(|(next_pos, &(next_cost, level_change))| {
                let next_level = item.level as i32 + level_change;
                if next_cost > 0 && next_level >= 0 {
                    Some(HeapItem {
                        pos: next_pos as u32,
                        cost: item.cost + next_cost,
                        level: next_level,
                        prev: item.pos,
                        plevel: item.level,
                    })
                } else {
                    None
                }
            })
        );
    }
    let mut parent = optimal.get(&(end_pi as u32, 0));
    let mut path: Vec<_> = vec![];
    path.push(PathItem { cost: shortest, parent: end_pi as u32, plevel: 0 });
    while let Some(prev) = parent {
        parent = optimal.get(&(prev.parent, prev.plevel));
        if prev.cost == 0 {
            break;
        }
        path.push(*prev);
    }
    path.reverse();
    path.iter().tuple_windows().for_each(|(p, n)| {
        println!("{}:{} -> {}:{} for {: >3}", portals[p.parent as usize].label, p.plevel, portals[n.parent as usize].label, n.plevel, p.cost);
    });
}

fn init(input_str: &str) -> (Vec<Portal>, Vec<Vec<(u32, i32)>>) {
    let row_size = input_str.lines().map(|l| l.bytes().count()).max().unwrap();
    let row_height = input_str.lines().count();

    let mut map = Map {
        tsize: vec2i(row_size as i32, row_height as i32),
        tiles: vec![Tiles::None; (row_size * row_height) as usize],
    };
    input_str.lines()
        .enumerate()
        .for_each(|(y, l)| l.chars()
            .enumerate()
            .for_each(|(x, ch)| {
                *map.tile_mut(vec2i(x as i32, y as i32)) = Tiles::from(ch);
            }));

    // init portals
    let inner_portal = inside_size_offset(map.tsize, 3);
    let mut portals: Vec<Portal> = vec![];
    for (i, t) in map.tiles.iter().enumerate() {
        if let Tiles::Label(label_end) = *t {
            let pos = vec2i(i as i32 % map.tsize.x, i as i32 / map.tsize.x);
            let maybe_path = ALL_DIRS.iter()
                .filter_map(move |&d| {
                    Some(pos + d).filter(inside_size(map.tsize))
                })
                .find(|&p| *map.tile(p) == Tiles::Path);

            if let Some(path_pos) = maybe_path {
                let away_dir = pos - path_pos;
                let label_pos = pos + away_dir;
                let ls = map.tile(label_pos);
                if let Tiles::Label(label_start) = *ls {
                    let label = String::from_iter(if away_dir.y == 1 || away_dir.x == 1 {
                        [label_end, label_start]
                    } else {
                        [label_start, label_end]
                    });

                    let is_inner = inner_portal(&path_pos);
                    portals.push(Portal{ label, is_inner, pos, path: path_pos, dest: 0 });
                }
            }
        }
    }
    portals.sort_by(|a, b| a.label.cmp(&b.label));
    // remove labels
    for t in map.tiles.iter_mut() {
        if let Tiles::Label(_) = *t {
            *t = Tiles::None;
        }
    }
    // place portals
    for (i, p) in portals.iter().enumerate() {
        *map.tile_mut(p.pos) = Tiles::Portal(i, p.is_inner);
    }
    // pair portals
    for i in 0..(portals.len() as i32 - 1).max(0) as usize {
        if portals[i].label == portals[i + 1].label {
            portals[i].dest = i + 1;
            portals[i + 1].dest = i;
        }
    }

    // map._debug_tiles();

    // build distance matrix with BFS
    let mut dist_matrix = vec![vec![(0u32, 0i32); portals.len()]; portals.len()];
    for (pi, portal) in portals.iter().enumerate() {
        let start = portal.path;
        let mut open: VecDeque<(Vec2i, i32)> = VecDeque::from([(start, 0)]);
        let mut visited = HashMap::from([(start, 1)]);
        let mut pcosts = vec![];
        while !open.is_empty() {
            let (pos, cost) = open.pop_front().unwrap();
            let tile = *map.tile(pos);
            let was_visited = visited.get(&pos);
            if was_visited.is_some_and(|&existing| existing <= cost) {
                continue;
            } else {
                visited.insert(pos, cost);
            }
            match tile {
                Tiles::Portal(src_index, is_inner) => {
                    let is_entry = ["AA", "ZZ"].contains(&&(portals[src_index].label)[..]);
                    let dst_index = if is_entry { src_index} else { portals[src_index].dest };
                    if src_index != pi {
                        let fixed_cost = if is_entry { cost - 1 } else { cost };
                        let level_change = if is_entry { 0 } else if is_inner { 1 } else { -1 };
                        pcosts.push((dst_index, fixed_cost as u32, level_change));
                    }
                },
                Tiles::Path => {
                    open.extend(ALL_DIRS.iter()
                        .filter_map(|&d| {
                            Some(pos + d)
                                .filter(inside_size(map.tsize))
                                .map(|p| (p, cost + 1))
                        }));
                },
                _ => ()
            }
        }
        // println!("{:?}", pcosts);

        for (di, cost, lc) in pcosts {
            let (existing, _) = dist_matrix[pi][di];
            if existing == 0 || existing > cost {
                dist_matrix[pi][di] = (cost, lc);
            }
        }
    }

    // debug distance matrix
    // dist_matrix.iter().enumerate().for_each(|(i, k)| {
    //     println!("{: >3} -> {}", portals[i].label, k.iter()
    //         .enumerate()
    //         .filter_map(|(k, &(p, _))| Some(format!("({} {: >3})", portals[k].label, p)).filter(|_| p != 0))
    //         // .map(|&p| format!("{: >3}", p))
    //         .join(" "));
    // });
    return (portals, dist_matrix);
}

fn main() {
    let input_path = "aoc20/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");

    let (portals, dist_matrix) = init(&input_str);
    // part_one(map, portals, dist_matrix);
    part_two(portals, dist_matrix);
}
