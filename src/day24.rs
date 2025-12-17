use std::fs::read_to_string;

fn parse(path: &str) -> Vec<(usize, usize)> {
    read_to_string(path)
        .unwrap()
        .lines()
        .filter_map(|line| {
            let (left, right) = line.split_once('/')?;
            let left = left.trim();
            let right = right.trim();
            let left: usize = left.parse().unwrap();
            let right: usize = right.parse().unwrap();
            Some((left, right))
        })
        .collect()
}

fn recursive(
    connector: usize,
    component: Vec<(usize, usize)>,
    current_strength: usize,
    max_strength: &mut usize,
) {
    if component.len() == 0 {
        if current_strength > *max_strength {
            *max_strength = current_strength;
        }
        return;
    }

    for (index, c) in component.iter().enumerate() {
        if c.0 == connector {
            let mut new_componet = component.clone();
            new_componet.remove(index);
            recursive(
                c.1,
                new_componet,
                current_strength + c.1 + c.0,
                max_strength,
            );
        } else if c.1 == connector {
            let mut new_componet = component.clone();
            new_componet.remove(index);
            recursive(
                c.0,
                new_componet,
                current_strength + c.1 + c.0,
                max_strength,
            );
        }
    }

    if current_strength > *max_strength {
        *max_strength = current_strength;
    }
}

pub fn first() {
    let mut max = 0;
    let components = parse("data/24");
    recursive(0, components, 0, &mut max);
    println!("the max found is: {max}");
}

pub fn second() {
    let mut max = 0;
    let mut lenght = 0;
    let components = parse("data/24");
    recursivev2(0, components, 0, &mut max, 0, &mut lenght);
    println!("the max found is: {max}");
}

fn recursivev2(
    connector: usize,
    component: Vec<(usize, usize)>,
    current_strength: usize,
    max_strength: &mut usize,
    current_lenght: usize,
    max_lenght: &mut usize,
) {
    if component.len() == 0 {
        if current_lenght > *max_lenght
            || (current_lenght == *max_lenght && current_strength >= *max_strength)
        {
            *max_lenght = current_lenght;
            *max_strength = current_strength;
        }
        return;
    }

    for (index, c) in component.iter().enumerate() {
        if c.0 == connector {
            let mut new_componet = component.clone();
            new_componet.remove(index);
            recursivev2(
                c.1,
                new_componet,
                current_strength + c.1 + c.0,
                max_strength,
                current_lenght + 1,
                max_lenght,
            );
        } else if c.1 == connector {
            let mut new_componet = component.clone();
            new_componet.remove(index);
            recursivev2(
                c.0,
                new_componet,
                current_strength + c.1 + c.0,
                max_strength,
                current_lenght + 1,
                max_lenght,
            );
        }
    }

    if current_lenght > *max_lenght
        || (current_lenght == *max_lenght && current_strength >= *max_strength)
    {
        *max_lenght = current_lenght;
        *max_strength = current_strength;
    }
}
