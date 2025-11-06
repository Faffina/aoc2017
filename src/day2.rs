use std::fs::read_to_string;

pub fn first() {
    let sum:usize = read_to_string("data/2")
        .unwrap()
        .lines()
        .filter_map(|x| {
            let digits: Vec<_> = x
                .split_whitespace()
                .filter_map(|y| y.parse().ok())
                .collect();
            let one:usize = *digits.iter().max()?;
            let two:usize = *digits.iter().min()?;
            Some(one - two)
        })
    .sum();
    println!("part one: {}", sum);
}

pub fn second() {
    let sum:usize = read_to_string("data/2")
        .unwrap()
        .lines()
        .filter_map(|x| {
            let digits: Vec<_> = x
                .split_whitespace()
                .filter_map(|y| y.parse::<usize>().ok())
                .collect();
            for one in digits.iter(){
                for two in digits.iter() {
                    if *one % *two == 0 && *one != *two{
                        return Some(*one / *two);
                    }
                }
            }
            None
        })
    .sum();
    println!("part two: {}", sum);
}
