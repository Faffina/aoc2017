use std::fs::read_to_string;

pub fn first() {
    let mut jumps: Vec<i64> = read_to_string("data/5")
        .unwrap()
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();
    let mut steps = 0;
    let mut index: i64 = 0;
    while index >= 0 && (index as usize) < jumps.len() {
        let next = jumps[index as usize];
        jumps[index as usize] += 1;
        index += next;
        steps += 1;
    }
    println!("{steps}");
}

pub fn second() {
    let mut jumps: Vec<i64> = read_to_string("data/5")
        .unwrap()
        .lines()
        .filter_map(|line| line.parse::<i64>().ok())
        .collect();
    let mut steps = 0;
    let mut index: i64 = 0;
    while index >= 0 && (index as usize) < jumps.len() {
        let next = jumps[index as usize];
        if next >= 3 {
            jumps[index as usize] -= 1;
        } else {
            jumps[index as usize] += 1;
        }
        index += next;
        steps += 1;
    }
    println!("{steps}");
}
