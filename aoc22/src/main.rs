use std::{fs, ops::Shr};

use num::{Num, Bounded, One, Zero};

#[derive(Debug, Clone, Copy)]
enum Op {
    Sub(i64),
    Mul(i64),
    Neg,
}

fn parse_ops(path: &str, cards_len: i64) -> Vec<Op> {
    let input_str = fs::read_to_string(path)
        .expect("Something went wrong reading the file");

    let ops: Vec<_> = input_str.lines().map(|l| {
        if l.starts_with("cut") {
            let (_, cut_str) = l.rsplit_once(' ').expect("Wrong line format");
            let mut cut: i64 = cut_str.parse().expect("cut parsing failed");
            cut = if cut < 0 { cut + cards_len as i64 } else { cut };
            Op::Sub(cut)
        } else if l.starts_with("deal with increment") {
            let (_, incr_str) = l.rsplit_once(' ').expect("Wrong line format");
            let incr: i64 = incr_str.parse().expect("deal_inc parsing failed");
            Op::Mul(incr)
        } else if l.starts_with("deal into new stack") {
            Op::Neg
        } else {
            panic!("Unknown op")
        }
    }).collect();
    ops
}

fn mod_exp<T>(base: T, exponent: T, modulus: T) -> T where T: Num + PartialOrd + Shr<T, Output=T> + Copy + Bounded {
    let one: T = One::one();
    let two: T = one + one;
    let zero: T = Zero::zero();
    let max: T = Bounded::max_value();

    assert!((modulus - one)  < (max / (modulus - one)));

    let mut result = one;
    let mut b = base % modulus;
    let mut exp = exponent;

    while exp > zero {
        if exp % two == one {
            result = (result * b) % modulus;
        }
        exp = exp >> one;
        b = (b * b) % modulus;
    }
    result
}

fn _run_ops(ops: &Vec<Op>, pos: i64, cards_len: i64) -> i64 {
    let mut card_pos = pos;
    for op in ops.iter() {
        match op {
            Op::Sub(cut) => {
                card_pos -= cut;
                if card_pos < 0 {
                    card_pos += cards_len;
                }
            },
            Op::Mul(incr) => {
                card_pos = (card_pos * incr) % cards_len;
            },
            Op::Neg => {
                card_pos = cards_len - 1 - card_pos;
            }
        // println!("card_pos {}", card_pos);
        }
    }
    card_pos
}

fn egcd(mut a: i64, mut b: i64) -> (i64, i64, i64) {
    let (mut x, mut y) = (0, 1);
    let (mut u, mut v) = (1, 0);
    while a != 0 {
        let (qout, rem) = (b / a, b % a);
        let (m, n) = (x - u * qout, y - v * qout);
        (b,a, x,y, u,v) = (a,rem, u,v, m,n);
    }
    return (b, x, y);
}

// (a^0 + a^1 + a^2 + a^3 .. + a^n) % m
fn pow_sum_mod(a: i64, n: i64, m: i64) -> i64 {
    if n == 0 {
        1
    } else if n % 2 == 0 {
        // even
        let sqm = mul_mod_i128(a, a, m);
        let rest = pow_sum_mod(sqm, (n - 2) / 2, m);
        let next_a = a + sqm;
        1 + mul_mod_i128(next_a, rest, m)
    } else {
        // odd
        let sqm = mul_mod_i128(a, a, m);
        let rest = pow_sum_mod(sqm, (n - 1) / 2, m);
        let next_a = (1 + a) as i64;
        mul_mod_i128(next_a, rest, m)
    }
}

fn mul_mod_i128(a: i64, b: i64, m: i64) -> i64 {
    ((a as i128) * (b as i128) % (m as i128)) as i64
}

enum FromCard {
    Factory(i64),
    Shuffled(i64),
}

fn main() {
    let tests = [
        ("aoc22/test3.txt", 10, FromCard::Factory(2), Some(1), 1),
        ("aoc22/test3.txt", 10, FromCard::Factory(5), Some(2), 1),
        ("aoc22/test3.txt", 10, FromCard::Factory(4), Some(5), 1),
        ("aoc22/input.txt", 10007, FromCard::Factory(2019), Some(2496), 1),
        ("aoc22/input.txt", 119315717514047, FromCard::Shuffled(2020), Some(56894170832118), 101741582076661i64),
    ];

    for (path, cards_len, pos, maybe_expect, exp) in tests {
        let ops = parse_ops(path, cards_len);

        // fold operations to mul and add terms
        let (mul, add) = ops.iter().fold((1, 0), |(m, a), o| {
            match o {
                Op::Sub(x) => (m, (a - x + cards_len) % cards_len),
                Op::Mul(x) => (m * x % cards_len, a * x % cards_len),
                Op::Neg => (- m, (- a + cards_len - 1) % cards_len)
            }
        });
        // reverse fold operations
        let (rmul, radd) = ops.iter().rev().fold((1, 0), |(m, a), &o| {
            match o {
                Op::Sub(x) => (m, (a + x) % cards_len),
                Op::Mul(x) => {
                    let (g, s, _) = egcd(x, cards_len);
                    assert_eq!(g, 1, "Numbers have to be coprime");
                    (mul_mod_i128(s, m, cards_len), mul_mod_i128(s, a, cards_len))
                },
                Op::Neg => (- m, (- a + cards_len - 1) % cards_len)
            }
        });

        // f(x) = (a * x + b) % m
        // calculates applying f exp times
        fn linear_exp(x: i64, a: i64, b: i64, exp: i64, m: i64) -> i64 {
            // we have to cast to i128
            let rmule = mod_exp(a as i128, exp as i128, m as i128) as i64;
            let sum_mod = pow_sum_mod(a, exp - 1, m);
            let radde = mul_mod_i128(b, sum_mod, m);
            (mul_mod_i128(rmule, x, m) + radde) % m
        }

        match pos {
            FromCard::Factory(x) => {
                let shuffled = linear_exp(x, mul, add, exp, cards_len);
                if let Some(expect) = maybe_expect {
                    assert_eq!(shuffled, expect, "file {path} with cards_len {cards_len}");
                } else {
                    println!("shuffled {shuffled}");
                }

                let unshuffled = linear_exp(shuffled, rmul, radd, exp, cards_len);
                assert_eq!(x, unshuffled);
            },
            FromCard::Shuffled(x) => {
                let unshuffled = linear_exp(x, rmul, radd, exp, cards_len);
                if let Some(expect) = maybe_expect {
                    assert_eq!(unshuffled, expect, "file {path} with cards_len {cards_len}");
                } else {
                    println!("unshuffled {unshuffled}");
                }

                let shuffled = linear_exp(unshuffled, mul, add, exp, cards_len);
                assert_eq!(x, shuffled);
            }
        }
    }
}
