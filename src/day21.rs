use std::{collections::HashMap, fs::read_to_string, mem::swap};

pub fn first() {
    let rules = parse();
    let mut current = Matrix::new(".#...####");
    let mut older = Matrix::new_whit_cap(current.size);

    for _ in 0..18 {
        let chunk = if current.size % 2 == 0 {
            2
        } else if current.size % 3 == 0 {
            3
        } else {
            panic!("should not be here");
        };

        let element_number = current.size / chunk;
        let new_size = element_number * (chunk + 1);
        older.reset(new_size);

        for y in 0..element_number {
            for x in 0..element_number {
                let oldx = x * chunk;
                let oldy = y * chunk;
                let newx = x * (chunk + 1);
                let newy = y * (chunk + 1);
                let sub1 = current.sub_matrix(chunk, oldy, oldx);
                if let Some(sub) = rules.get(&sub1) {
                    older.compose(sub, newy, newx)
                } else {
                    panic!("current: {:?}, {:?}", current, sub1);
                }
            }
        }

        swap(&mut older, &mut current);
    }

    println!("{}", current.count_on());
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
struct Matrix {
    size: usize,
    grid: Vec<bool>,
}

impl Matrix {
    fn reset(&mut self, size: usize) {
        self.size = size;
        self.grid.clear();
        for _ in 0..(size * size) {
            self.grid.push(false);
        }
    }

    fn new(line: &str) -> Self {
        let grid: Vec<bool> = line
            .chars()
            .filter_map(|c| match c {
                '#' => Some(true),
                '.' => Some(false),
                _ => None,
            })
            .collect();
        let size = (grid.len() as f64).sqrt() as usize;
        assert!(size * size == grid.len());
        Self { size, grid }
    }

    fn new_whit_cap(init_size: usize) -> Self {
        Self {
            size: init_size,
            grid: vec![false; init_size * init_size],
        }
    }

    fn rearange(&self, list: &[usize]) -> Self {
        let mut new_matrix = Matrix::new_whit_cap(self.size);
        for (old, new) in list.iter().enumerate() {
            new_matrix.grid[*new] = self.grid[old];
        }
        new_matrix
    }

    fn rotate90(&self) -> Self {
        match self.size {
            2 => self.rearange(&[1, 3, 0, 2]),
            3 => self.rearange(&[2, 5, 8, 1, 4, 7, 0, 3, 6]),
            _ => panic!("cant rotate for > 3 {}", self.size),
        }
    }

    fn flip(&self) -> (Self, Self) {
        match self.size {
            2 => (self.rearange(&[1, 0, 3, 2]), self.rearange(&[2, 3, 0, 1])),
            3 => (
                self.rearange(&[2, 1, 0, 5, 4, 3, 8, 7, 6]),
                self.rearange(&[6, 7, 8, 3, 4, 5, 0, 1, 2]),
            ),
            _ => panic!("cant flip for > 3 {}", self.size),
        }
    }

    fn set(&mut self, y: usize, x: usize, led: bool) {
        self.grid[y * self.size + x] = led;
    }

    fn get(&self, y: usize, x: usize) -> bool {
        self.grid[y * self.size + x]
    }

    fn sub_matrix(&self, size: usize, y: usize, x: usize) -> Self {
        let mut sub = Matrix::new_whit_cap(size);
        assert!(y + size <= self.size);
        assert!(x + size <= self.size);

        for yy in 0..size {
            for xx in 0..size {
                sub.set(yy, xx, self.get(yy + y, xx + x));
            }
        }
        sub
    }

    fn compose(&mut self, other: &Self, y: usize, x: usize) {
        assert!(y + other.size <= self.size);
        assert!(x + other.size <= self.size);

        for yy in 0..other.size {
            for xx in 0..other.size {
                self.set(yy + y, xx + x, other.get(yy, xx));
            }
        }
    }

    fn count_on(&self) -> usize {
        self.grid.iter().filter(|x| **x).count()
    }
}

fn parse() -> HashMap<Matrix, Matrix> {
    read_to_string("data/21")
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (from, to) = line.split_once("=>")?;
            let mut from = Matrix::new(from);
            let to = Matrix::new(to);
            let mut parts = Vec::new();

            if  from.size == 0 || to.size == 0 {
                return None;
            }

            for _ in 0..4 {
                parts.push((from.clone(), to.clone()));

                let (f1, f2) = from.flip();
                parts.push((f1, to.clone()));
                parts.push((f2, to.clone()));

                from = from.rotate90();
            }

            Some(parts)
        })
        .flatten()
        .collect()
}
