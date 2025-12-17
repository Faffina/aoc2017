use std::{collections::HashMap, fs::read_to_string};


pub fn first() {
    let mut virus = Virus::new("data/22");

    for _ in 0..10000 {
        virus.brust();
    }

    println!("total infected: {}", virus.count);
}

pub fn second() {
    let mut virus = Virus::new("data/22");

    for _ in 0..10000000 {
        virus.brust_day_2();
    }

    println!("total infected: {}", virus.count);
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum NodeStatus {
    Infected,
    Clean,
    weakened,
    flagged,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
struct Position {
    x: i64,
    y: i64,
}

struct Virus {
    direction: Position,
    position: Position,
    map: HashMap<Position, NodeStatus>,
    count: usize,
}

impl NodeStatus {
    fn flip(&mut self) {
        match self {
            NodeStatus::Clean => *self = NodeStatus::Infected,
            NodeStatus::Infected => *self = NodeStatus::Clean,
            _ => ()
        }
    }

    fn complete_fip(&mut self) {
        *self = match self {
            NodeStatus::Clean => NodeStatus::weakened,
            NodeStatus::weakened => NodeStatus::Infected,
            NodeStatus::Infected => NodeStatus::flagged,
            NodeStatus::flagged => NodeStatus::Clean,
        }
    }
}

impl Position {
    fn rotate_left(&mut self) {
        (self.x, self.y) = (-self.y, self.x);
    }

    fn rotate_right(&mut self) {
        (self.x, self.y) = (self.y, -self.x);
    }

    fn add(&mut self, other: Self ){
        self.x += other.x;
        self.y += other.y;
    }
}

impl Virus {
    fn brust_day_2(&mut self) {
        let current_node: &mut NodeStatus = self.map.entry(self.position).or_insert(NodeStatus::Clean);
        match current_node {
            NodeStatus::Clean => {
                self.direction.rotate_left();
            }
            NodeStatus::weakened => {
                self.count += 1;
            }
            NodeStatus::Infected => {
                self.direction.rotate_right();
            }
            NodeStatus::flagged => {
                self.direction.rotate_right();
                self.direction.rotate_right();
            }
        }
        current_node.complete_fip();
        self.position.add(self.direction);
    }

    fn brust(&mut self) {
        let current_node: &mut NodeStatus = self.map.entry(self.position).or_insert(NodeStatus::Clean);
        match current_node {
            NodeStatus::Clean => {self.direction.rotate_left(); self.count += 1},
            NodeStatus::Infected => self.direction.rotate_right(),
            _ => (),
        }
        current_node.flip();
        self.position.add(self.direction);
    }


    fn new(data_path: &str) -> Self {
        let data: Vec<Vec<_>> = read_to_string(data_path)
            .unwrap()
            .lines()
            .filter_map(|line| {
                let temp = line.chars().filter_map(|c| {
                    match c {
                        '#' => Some(NodeStatus::Infected),
                        '.' => Some(NodeStatus::Clean),
                        _ => None,
                    }
                }).collect::<Vec<NodeStatus>>();
                if temp.len() > 0 {
                    Some(temp)
                } else {
                    None
                }
            }).collect();

        let lenght = data.first().unwrap().len();

        for i in data.iter() {
            assert!(lenght == i.len())
        }

        let x = lenght as i64 / 2;
        let y = data.len() as i64 / 2;

        let mut map: HashMap<Position, NodeStatus> = HashMap::new();
        let position = Position {x: 0, y: 0};
        let direction = Position {x: 0, y: 1};

        for (ry, row) in data.iter().enumerate() {
            for (rx, node) in row.iter().enumerate() {
                let nx = rx as i64 - x;
                let ny = y - ry as i64;
                map.insert(Position { x: nx, y: ny }, *node);
            }
        }

        Self { direction, position, map, count: 0}
    }
}
