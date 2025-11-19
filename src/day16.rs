use std::{collections::HashMap, fs::read_to_string};

enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char)
}

fn parse() -> Vec<DanceMove> {
    read_to_string("data/16")
        .unwrap()
        .split(",")
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .map(|x| {
            let instruction = x.chars().nth(0).unwrap();
            let x = x.get(1..).unwrap();
            match instruction {
                's' => {
                    let x: usize = x.parse().unwrap();
                    DanceMove::Spin(x)
                },
                'x' => {
                    let (a, b) =  x.split_once('/').unwrap();
                    let a: usize = a.parse().unwrap();
                    let b: usize = b.parse().unwrap();
                    DanceMove::Exchange(a, b)
                },
                'p' => {
                    let a = x.chars().nth(0).unwrap();
                    let b = x.chars().nth(2).unwrap();
                    DanceMove::Partner(a, b)
                }
                _ => panic!("invalid instruction {instruction} {x}")
            }
        }).collect()
}

pub fn first() {
    let mut programs: [char; 16] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
    let moves = parse();
    for m in moves.iter() {
        match m {
            DanceMove::Spin(n) => programs.rotate_right(*n),
            DanceMove::Exchange(a, b) => programs.swap(*a, *b),
            DanceMove::Partner(a, b) => {
                let a = programs.iter().position(|x| *x == *a).unwrap();
                let b = programs.iter().position(|x| *x == *b).unwrap();
                programs.swap(a, b);
            }
        }
    }
    let programs: String = programs.iter().collect();
    println!("{programs}");
}

fn dance(programs: &mut [char; 16], moves: &Vec<DanceMove>) {
    for m in moves.iter() {
        match m {
            DanceMove::Spin(n) => programs.rotate_right(*n),
            DanceMove::Exchange(a, b) => programs.swap(*a, *b),
            DanceMove::Partner(a, b) => {
                let a = programs.iter().position(|x| *x == *a).unwrap();
                let b = programs.iter().position(|x| *x == *b).unwrap();
                programs.swap(a, b);
            }
        }
    }
}


pub fn second() {
    let mut programs: [char; 16] = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p'];
    let mut visited: HashMap<String, usize> = HashMap::new();
    let moves = parse();
    for i in 0..1000000000 {
        let key: String = programs.iter().collect();

        if let Some(p) = visited.get(&key) {
            let period = i - p;
            let remaing = (1000000000 - i) % period;

            for _ in 0..remaing {
                dance(&mut programs, &moves);
            }
            break;
        }
        visited.insert(key, i);
        dance(&mut programs, &moves);
    }
    let programs: String = programs.iter().collect();
    println!("{programs}");
}
