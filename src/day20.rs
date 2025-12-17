use std::{collections::HashSet, fs::read_to_string, ops::AddAssign};

pub fn first() {
    let mut particle = Particle::parse();
    let mut clossest = particle.iter().min().unwrap().id;
    let mut time_whit_no_change: usize = 0;
    const MAX_TIME: usize = 1_000_000;

    while time_whit_no_change < MAX_TIME {
        for p in particle.iter_mut() {
            p.apply();
        }
        let new_clossest = particle.iter().min().unwrap().id;

        if new_clossest != clossest {
            time_whit_no_change = 0;
            clossest = new_clossest;
            println!("new_best {new_clossest}");
            continue;
        }
        time_whit_no_change += 1;
    }
}
pub fn second() {
    let mut particle = Particle::parse();
    let mut clossest = particle.len();
    let mut time_whit_no_change: usize = 0;
    const MAX_TIME: usize = 1_000_000;

    remove_colding(&mut particle);
    while time_whit_no_change < MAX_TIME {
        for p in particle.iter_mut() {
            p.apply();
        }
        remove_colding(&mut particle);
        let new_clossest = particle.len();

        if new_clossest != clossest {
            time_whit_no_change = 0;
            clossest = new_clossest;
            println!("new_best {new_clossest}");
            continue;
        }
        time_whit_no_change += 1;
    }
}

fn remove_colding(particle: &mut Vec<Particle>) {
    particle.sort();
    let mut mark_for_del = HashSet::new();

    let mut start = 0;

    for i in 1..particle.len() {
        if particle[start].distance != particle[i].distance {
            if i - start > 1 {
                for a in start..i {
                    for b in start..i {
                        if particle[a].id != particle[b].id {
                            if particle[a].position == particle[b].position {
                                mark_for_del.insert(a);
                                mark_for_del.insert(b);
                            }
                        }
                    }
                }
            }
            start = i;
        }
    }

    let mut mark_for_del: Vec<usize> = mark_for_del.into_iter().collect();
    mark_for_del.sort();
    mark_for_del.reverse();
    for i in mark_for_del {
        particle.swap_remove(i);
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Vec3 {
    x: i64,
    y: i64,
    z: i64,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Particle {
    position: Vec3,
    velocity: Vec3,
    acceleration: Vec3,
    id: usize,
    distance: usize,
}

impl Ord for Particle {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.distance.cmp(&other.distance)
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Vec3 {
    fn distance(self) -> usize {
        self.x.abs() as usize + self.y.abs() as usize + self.z.abs() as usize
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Particle {
    fn apply(&mut self) {
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.distance = self.position.distance();
    }
}

impl Particle {
    fn parse() -> Vec<Self> {
        read_to_string("data/20")
            .unwrap()
            .lines()
            .enumerate()
            .filter_map(|(id, line)| {
                let position = &line[line.find('<')? + 1..line.find('>')?]
                    .trim()
                    .split(',')
                    .filter_map(|x| Some(x.parse::<i64>().ok()?))
                    .collect::<Vec<i64>>();
                let position = Vec3 {
                    x: position[0],
                    y: position[1],
                    z: position[2],
                };

                let line = &line[line.find('>')? + 1..];
                let velocity = &line[line.find('<')? + 1..line.find('>')?]
                    .trim()
                    .split(',')
                    .filter_map(|x| Some(x.parse::<i64>().ok()?))
                    .collect::<Vec<i64>>();
                let velocity = Vec3 {
                    x: velocity[0],
                    y: velocity[1],
                    z: velocity[2],
                };

                let line = &line[line.find('>')? + 1..];
                let acceleration = &line[line.find('<')? + 1..line.find('>')?]
                    .trim()
                    .split(',')
                    .filter_map(|x| Some(x.parse::<i64>().ok()?))
                    .collect::<Vec<i64>>();
                let acceleration = Vec3 {
                    x: acceleration[0],
                    y: acceleration[1],
                    z: acceleration[2],
                };
                Some(Particle {
                    id,
                    position,
                    velocity,
                    acceleration,
                    distance: position.distance(),
                })
            })
            .collect()
    }
}
