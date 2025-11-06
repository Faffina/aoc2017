use std::{collections::HashSet, fs::read_to_string};

pub fn first() {
    let valid_pass = read_to_string("data/4")
        .unwrap()
        .lines()
        .filter(|x| { 
            let x: Vec<&str> = x.split_whitespace().collect();
            let before = x.len();
            let x: HashSet<&str> = x.iter().cloned().collect();
            let after = x.len();
            before == after
        })
        .count();
    println!("{valid_pass}");
}

pub fn second() {
    let valid_pass = read_to_string("data/4")
        .unwrap()
        .lines()
        .filter(|x| { 
            let x: Vec<&str> = x.split_whitespace().collect();
            let before = x.len();
            let x: HashSet<String> = x.iter().cloned().map(|y| {
                let mut v: Vec<char> = y.chars().collect();
                v.sort();
                let v: String = v.iter().collect();
                v
            }).collect();
            let after = x.len();
            before == after
        })
        .count();
    println!("{valid_pass}");
}
