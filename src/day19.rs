use std::fs::read_to_string;

pub fn first() {
    let map = parse();
    let mut y = 0;
    let mut steps = 1;
    if let Some(mut x) = find_start(&map) {
        let mut dir = Directions::donw;
        'outer: loop {
            for d in dir.get_next() {
                if let Some((c, ny, nx)) = get(&map, y, x, d) {
                    if c.is_alphabetic() {
                        print!("{c}");
                    }
                    y = ny;
                    x = nx;
                    dir = d;
                    steps += 1;
                    continue 'outer;
                }
            }
            println!(" {steps}");
            break 'outer;
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Directions {
    up,
    donw,
    left,
    right,
}

impl Directions {
    fn get_next(&self) -> [Self; 3] {
        match *self {
            Directions::up => [*self, Directions::right, Directions::left],
            Directions::donw => [*self, Directions::right, Directions::left],
            Directions::left => [*self, Directions::up, Directions::donw],
            Directions::right => [*self, Directions::up, Directions::donw],
        }
    }
}

fn get(
    map: &Vec<Vec<char>>,
    mut y: usize,
    mut x: usize,
    dir: Directions,
) -> Option<(char, usize, usize)> {
    match dir {
        Directions::up => y -= 1,
        Directions::donw => y += 1,
        Directions::left => x -= 1,
        Directions::right => x += 1,
    }
    if y < map.len() {
        if x < map[y].len() {
            if !map[y][x].is_whitespace() {
                return Some((map[y][x], y, x));
            }
        }
    }
    None
}

fn find_start(map: &Vec<Vec<char>>) -> Option<usize> {
    for (x, v) in map.first()?.iter().enumerate() {
        if *v == '|' {
            return Some(x);
        }
    }
    None
}

fn parse() -> Vec<Vec<char>> {
    read_to_string("data/19")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let line: Vec<char> = line.chars().collect();
            if line.len() >= 2 { Some(line) } else { None }
        })
        .collect()
}
