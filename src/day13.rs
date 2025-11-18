use std::cmp::min;
use std::fs::read_to_string;

pub fn first() {
    let severity: usize = read_to_string("data/13")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (n, k) = line.split_once(": ")?;
            let n: usize = n.parse().ok()?;
            let k: usize = k.parse().ok()?;
            let t = 2 * k - 2;
            if n % t == 0 { Some(n * k) } else { None }
        })
        .sum();
    println!("{severity}")
}

pub fn second() {
    let scanners: Vec<_> = read_to_string("data/13")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (n, k) = line.split_once(": ")?;
            let n: usize = n.parse().ok()?;
            let k: usize = k.parse().ok()?;
            let t = 2 * k - 2;
            Some((n, t))
        })
        .collect();
    for delay in 0..10000000 {
        if scanners.iter().all(|(n, t)| {
            let n = n + delay;
            n % t != 0
        }) {
            println!("found {delay}");
            break;
        }
    }
}
