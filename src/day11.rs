use std::{
    fs::read_to_string,
    ops::{Add, Sub},
};

pub fn first() {
    let s: String = read_to_string("data/11").unwrap();
    let final_postion = s
        .split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .fold(HexPos::zero(), |acc, x| acc + x);
    let distance = HexPos::zero().distance(final_postion);
    println!("found distance: {}", distance)
}

pub fn second() {
    let s: String = read_to_string("data/11").unwrap();
    let mut max_distnace: usize = 0;
    let _ = s
        .split(',')
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .fold(HexPos::zero(), |acc, x| {
            let new_point = acc + x;
            let distance = HexPos::zero().distance(new_point); 
            if distance > max_distnace {
                max_distnace = distance;
            }
            new_point
        });
    println!("found distance: {}", max_distnace)
}

#[derive(Debug, Clone, Copy)]
struct HexPos {
    q: i64,
    r: i64,
    s: i64,
}

impl HexPos {
    fn get_neighbotr(direction: &str) -> HexPos {
        match direction {
            "n" => HexPos { q: 0, r: -1, s: 1 },
            "ne" => HexPos { q: 1, r: -1, s: 0 },
            "se" => HexPos { q: 1, r: 0, s: -1 },
            "s" => HexPos { q: 0, r: 1, s: -1 },
            "sw" => HexPos { q: -1, r: 1, s: 0 },
            "nw" => HexPos { q: -1, r: 0, s: 1 },
            _ => HexPos { q: 0, r: 0, s: 0 },
        }
    }

    fn distance(&self, rhs: Self) -> usize {
        ((self.q - rhs.q).abs() + (self.r - rhs.r).abs() + (self.s - rhs.s).abs()) as usize / 2
    }

    fn zero() -> HexPos {
        HexPos { q: 0, r: 0, s: 0 }
    }
}

impl Add<&str> for HexPos {
    type Output = HexPos;

    fn add(self, rhs: &str) -> HexPos {
        self + HexPos::get_neighbotr(rhs)
    }
}

impl Add for HexPos {
    type Output = HexPos;

    fn add(self, rhs: Self) -> HexPos {
        HexPos {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
            s: self.s + rhs.s,
        }
    }
}
