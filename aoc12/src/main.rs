use std::{fs, fmt::Display};
use euclid::{self, Vector3D, UnknownUnit, vec3, bvec3};
use itertools::Itertools;
use num::integer::lcm;

#[derive(Debug, Clone, PartialEq)]
struct Moon {
    pos: Vector3D<i64, UnknownUnit>,
    vel: Vector3D<i64, UnknownUnit>,
}

impl Display for Moon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "pos=<x={: >2}, y={: >2}, z={: >2}>, vel=<x={: >2}, y={: >2}, z={: >2}>",
            self.pos.x, self.pos.y, self.pos.z,
            self.vel.x, self.vel.y, self.vel.z
        )
    }
}

fn main() {
    let input_path = "aoc12/input.txt";
    let input_str = fs::read_to_string(input_path).expect("Something went wrong reading the file");
    let orig_moons: Vec<Moon> = input_str
        .lines()
        .map(|line| {
            let coords: Vec<i64> = line
                .trim_matches(&['<', '>'] as &[_])
                .split(',')
                .filter_map(|v| {
                    v.split_once('=')
                        .and_then(|(_, r)| r.parse::<i64>().ok())
                })
                .collect();
            match coords[..] {
                [x, y, z] => {
                    Moon {
                        pos: vec3(x, y, z),
                        vel: vec3(0, 0, 0),
                    }
                },
                _ => panic!("Bad input format")
            }
        })
        .collect();
    let mut moons = orig_moons.clone();
    // println!("{:#?}", moons);

    let mut period: Vector3D<i64, UnknownUnit> = vec3(0, 0, 0);
    for stepi in 1..i64::MAX {
        for pair in (0..moons.len()).into_iter().combinations(2) {
            match pair.as_slice() {
                &[ai, bi] => {
                    let diff: Vector3D<i64, UnknownUnit> = (moons[ai].pos - moons[bi].pos)
                        .clamp(vec3(-1, -1, -1), vec3(1, 1, 1));
                    // println!("{} {} is {:?}", ai, bi, diff);
                    moons[bi].vel += diff;
                    moons[ai].vel += diff.component_mul(vec3(-1, -1, -1));
                },
                _ => panic!("Bad pair match")
            }
        }

        for m in &mut moons {
            m.pos += m.vel;
        }

        let is_orig = moons.iter()
            .zip(orig_moons.iter())
            .fold(bvec3(true, true, true), |a, (m, o)| {
                a.and(m.vel.equal(vec3(0, 0, 0))).and(m.pos.equal(o.pos))
            });

        if period.x == 0 && is_orig.x {
            period.x = stepi;
        }
        if period.y == 0 && is_orig.y {
            period.y = stepi;
        }
        if period.z == 0 && is_orig.z {
            period.z = stepi;
        }
        // check if we found all periods
        if period.not_equal(vec3(0, 0, 0)).all() {
            break;
        }

        if stepi % 10000 == 0 {
            println!("iter {}", stepi);
            // moons.iter().for_each(|m| println!("{}", m));
            // break;
        }
    }
    println!("period {:?} lcm {}", period, lcm(lcm(period.x, period.y), period.z));

    let energy = moons.iter().fold(0i64, |a, m| {
        let pot = m.pos.abs().to_array().iter().sum::<i64>();
        let kin = m.vel.abs().to_array().iter().sum::<i64>();
        println!("pot {} kin {} sum {}", pot, kin, pot * kin);
        a + pot * kin
    });
    println!("energy {}", energy);
}
