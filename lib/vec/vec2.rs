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