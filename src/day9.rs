use std::fs::read_to_string;


pub fn first() {
    let mut pre_data:Vec<char> = read_to_string("data/9").unwrap().chars().collect();
    let mut last = false;
    let mut garbage = false;
    for c in pre_data.iter_mut() {
        if garbage {
            if last {
                *c = '0';
            }
            match *c {
                '>' => {last = false; garbage = false;}
                '!' => last = true,
                _ => last = false,
            }
            *c = '0';
        } else if *c == '<' {
            *c = '0';
            garbage = true;
        }
    }

    let mut open = 0;
    let mut sum = 0;
    for c in pre_data.into_iter() {
        match c {
            '{' => open += 1,
            '}' => {sum += open; open -= 1},
            _ => ()
        }
    }

    println!("{sum}");
}

pub fn second() {
    let mut pre_data:Vec<char> = read_to_string("data/9").unwrap().chars().collect();
    let mut last = false;
    let mut garbage = false;
    let mut sum = 0;
    for c in pre_data.iter_mut() {
        if garbage {
            if last {
                *c = '0';
            }
            match *c {
                '>' => {last = false; garbage = false;}
                '!' => last = true,
                '0' => last = false,
                _ => {last = false; sum += 1}
            }
        } else if *c == '<' {
            garbage = true;
        }
    }

    println!("{sum}");
}
