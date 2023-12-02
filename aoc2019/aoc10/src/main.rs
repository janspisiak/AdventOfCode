use std::{fs, error::Error, ops, f64::consts::PI, cmp::Ordering};
use itertools::Itertools;
use num::{Zero, ToPrimitive};

#[derive(Debug, Clone, Copy, PartialEq)]
struct Vec2<T>
{
    x: T,
    y: T,
}

impl<T> Vec2<T>
    where T: ops::Sub<Output = T> + ops::Add<Output = T> + ops::Mul<Output = T> + Zero + ToPrimitive + Copy
{
    fn new(x: T, y: T) -> Vec2<T> {
        Vec2{ x, y }
    }

    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }

    fn to_f32(&self) -> Vec2<f32> {
        Vec2{ x: self.x.to_f32().unwrap(), y: self.y.to_f32().unwrap() }
    }

    fn to_i32(&self) -> Vec2<i32> {
        Vec2{ x: self.x.to_i32().unwrap(), y: self.y.to_i32().unwrap() }
    }
    fn dot_self(&self) -> T {
        self.x * self.x + self.y * self.y
    }

    fn dot(&self, rhs: Vec2<T>) -> T {
        self.x * rhs.x + self.y * rhs.y
    }

    fn norm(&self) -> f32 {
        let dot = self.dot_self();
        dot.to_f32().unwrap().sqrt()
    }

    fn det(&self, rhs: Vec2<T>) -> T {
        self.x * rhs.y - self.y * rhs.x
    }

    fn cross(&self, rhs: Vec2<T>) -> T {
        self.det(rhs)
    }
}

impl<T> ops::Add<Vec2<T>> for Vec2<T>
    where T: ops::Add<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn add(self, rhs: Vec2<T>) -> Vec2<T> {
        Vec2{ x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl<T> ops::Sub<Vec2<T>> for Vec2<T>
    where T: ops::Sub<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Vec2{ x: self.x - rhs.x, y: self.y - rhs.y }
    }
}

impl<T> ops::Mul<T> for Vec2<T>
    where T: ops::Mul<Output = T> + Copy
{
    type Output = Vec2<T>;

    fn mul(self, rhs: T) -> Vec2<T> {
        Vec2{ x: self.x * rhs, y: self.y * rhs }
    }
}

fn make_vec2<T> (x: T, y: T) -> Vec2<T>
    where T: ToPrimitive + Copy
{
    return Vec2 { x, y };
}

type Point = Vec2<i32>;

#[derive(Debug, Clone, Copy)]
struct Line {
	from: Vec2<i32>,
	dir: Vec2<i32>,
}

impl Line {
    fn perp(&self, point: Point) -> Vec2<i32> {
        let to_p = point - self.from;
        to_p - (self.dir.to_f32() * (to_p.dot(self.dir) as f32 / self.dir.dot_self() as f32)).to_i32()
    }

    fn dist(&self, point: Point) -> f32 {
        self.perp(point).norm()
    }

    fn same(&self, line: &Line) -> bool {
        self.dir.dot(line.dir).abs() == 1 // same direction
        && self.perp(line.from).is_zero() // normal to origin is zero
    }
}

fn main() -> Result<(), Box<dyn Error>> {
	let input_path = "aoc10/input.txt";
	let input_str = fs::read_to_string(input_path)
		.expect("Something went wrong reading the file");

    let asteroids: Vec<_> = input_str
        .split('\n')
        .enumerate()
        .flat_map(|(y, l)|
            l.chars()
            .enumerate()
            .filter_map(move |(x, c)|
                if c == '#' {
                    Some(make_vec2(x as i32, y as i32))
                } else {
                    None
                }
            )
        ).collect();

    let mut asteroid_score = vec![0; asteroids.len()];
    // println!("asteroids: {:?}", asteroids);

    asteroids.iter()
        .enumerate()
        .combinations(2)
        .for_each(|points| {
            match points[..] {
                [(fi, &from), (ti, &to)] => {
                    let line = Line{ from: from, dir: to - from };
                    let occluded = asteroids.iter().any(|&p| {
                        if from == p || to == p {
                            return false;
                        }
                        let on_line: bool = line.perp(p).is_zero();
                        if from.y == 2 && to.y == 2 {
                            // println!("from: {:?} to: {:?} p: {:?} on_line {}", from, to, p, on_line);
                        }
                        if !on_line {
                            return false;
                        }
                        let u = (p.x - line.from.x) as f32 / line.dir.x as f32;
                        let v = (p.y - line.from.y) as f32 / line.dir.y as f32;
                        let is_between = u >= 0.0 && u <= 1.0 || v >= 0.0 && v <= 1.0;
                        // println!("from: {:?} to: {:?} p: {:?} u {} v {}", from, to, p, u, v);
                        return is_between;
                    });
                    if !occluded {
                        asteroid_score[fi] += 1;
                        asteroid_score[ti] += 1;
                    }
                }
                _ => panic!("bad match")
            }
        });

    let mut scores = asteroid_score.into_iter()
        .enumerate()
        .collect::<Vec<_>>();
    scores.sort_by(|a, b| b.1.cmp(&a.1));
    let laser_origin = asteroids[scores[0].0];
    println!("ast: {:?} with sccore {}", laser_origin, scores[0].1);

    let mut astByAngle: Vec<_> = asteroids.iter()
        .filter_map(|&a| {
            if a == laser_origin {
                return None;
            }
            let up = make_vec2(0, -1);
            let dir = a - laser_origin;
            let det = up.det(dir) as f64;
            let dot = up.dot(dir) as f64;
            let angle = (-det).atan2(-dot) + PI;
            Some((angle, dir.dot_self(), a))
        })
        .collect();
    astByAngle.sort_unstable_by(|a, b| {
        let angle_cmp = a.0.total_cmp(&b.0);
        if angle_cmp == Ordering::Equal {
            return a.1.cmp(&b.1);
        }
        return angle_cmp;
    });
    println!("{:?}", &astByAngle[0..3]);

    let mut removed_count = 0;
    let mut the_asteroid = make_vec2(0, 0);
    while !astByAngle.is_empty() {
        let mut last_angle = -1.0;
        astByAngle.retain(|&(angle, _, a)| {
            if angle != last_angle {
                last_angle = angle;
                removed_count += 1;
                if removed_count == 200 {
                    the_asteroid = a;
                }
                false
            } else {
                true
            }
        })
    }
    println!("the ast {:?} with {}", the_asteroid, the_asteroid.x * 100 + the_asteroid.y);

    Ok(())
}