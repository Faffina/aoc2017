use std::{collections::HashSet, fs::read_to_string};

pub fn first() {
    let connections = parse();
    let mut q: Vec<usize> = Vec::new();
    let mut l: HashSet<usize> = HashSet::new();
    q.push(0);
    while let Some(n) = q.pop() {
        if !l.insert(n) {
            continue;
        }
        q.extend(connections[n].iter());
    }
    println!("group size: {}", l.len() )
}

pub fn second() {
    let mut connections = parse();
    let mut n_grups = 0;
    loop {
        if let Some(first) = connections.iter().position(|e| !e.is_empty()) {
            n_grups += 1;
            let mut q: Vec<usize> = Vec::new();
            let mut l: HashSet<usize> = HashSet::new();
            q.push(first);
            while let Some(n) = q.pop() {
                if !l.insert(n) {
                    continue;
                }
                q.extend(connections[n].iter());
                connections[n].clear();
            }
        } else {
            break;
        }
    }
    println!("number of groups {n_grups}");
}

fn parse() -> Vec<Vec<usize>> {
    let mut ris: Vec<Vec<usize>> = Vec::new();
    let data = read_to_string("data/12").unwrap();
    let lines = data.lines();

    for line in lines {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            [id, "<->", connections @ ..] => {
                let id:usize = id.parse().unwrap();
                let connections: Vec<usize> = connections.iter()
                    .map(|x| x.trim_end_matches(','))
                    .filter_map(|x| x.parse().ok())
                    .collect();
                if id != ris.len() {
                    panic!("mismach connections {id} {}", ris.len());
                }
                ris.push(connections);
            }
            _ => (),
        }
    }
    ris
}


