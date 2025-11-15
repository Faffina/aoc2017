use std::{collections::HashMap, fs::read_to_string};

pub fn first() {
    let mut regirstes: HashMap<&str, i64> = HashMap::new();
    let data =  read_to_string("data/8").unwrap();
    data.lines().for_each(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            [xr, op, delta, "if", ar, cond, cond_op] => {
                let ar = regirstes.entry(*ar).or_insert(0);
                let cond_op = cond_op.parse::<i64>().unwrap();
                if match *cond {
                    "<" => *ar < cond_op,
                    "<=" => *ar <= cond_op,
                    ">" => *ar > cond_op,
                    ">=" => *ar >= cond_op,
                    "==" => *ar == cond_op,
                    "!=" => *ar != cond_op,
                    _ => false,
                } {
                    let xr = regirstes.entry(*xr).or_insert(0);
                    let op = match *op {
                        "inc" => 1,
                        "dec" => -1,
                        _ => 0,
                    };
                    let delta = delta.parse::<i64>().unwrap();
                    *xr += op * delta;
                }
            }
            _ => (),
        }
    });
    let ris = regirstes.iter().map(|x| x.1).max().unwrap();
    println!("found solution: {ris:?}");
}

pub fn second() {
    let mut regirstes: HashMap<&str, i64> = HashMap::new();
    let mut max = 0;
    let data =  read_to_string("data/8").unwrap();
    data.lines().for_each(|line| {
        let parts: Vec<_> = line.split_whitespace().collect();
        match parts.as_slice() {
            [xr, op, delta, "if", ar, cond, cond_op] => {
                let ar = regirstes.entry(*ar).or_insert(0);
                let cond_op = cond_op.parse::<i64>().unwrap();
                if match *cond {
                    "<" => *ar < cond_op,
                    "<=" => *ar <= cond_op,
                    ">" => *ar > cond_op,
                    ">=" => *ar >= cond_op,
                    "==" => *ar == cond_op,
                    "!=" => *ar != cond_op,
                    _ => false,
                } {
                    let xr = regirstes.entry(*xr).or_insert(0);
                    let op = match *op {
                        "inc" => 1,
                        "dec" => -1,
                        _ => 0,
                    };
                    let delta = delta.parse::<i64>().unwrap();
                    *xr += op * delta;
                    if *xr > max {
                        max = *xr
                    }
                }
            }
            _ => (),
        }
    });
    println!("found solution: {max}");
}
