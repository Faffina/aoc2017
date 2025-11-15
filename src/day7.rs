use std::fs::read_to_string;

pub fn first() {
    let data = read_to_string("data/7").unwrap();
    let programs = parse(&data);
    let mut seen_sub_programs: Vec<&str> = Vec::new();
    for p in programs.iter() {
        seen_sub_programs.append(&mut p.sub.clone());
    }
    for p in programs {
        if !seen_sub_programs.contains(&p.id) {
            println!("found program {}", p.id);
            return;
        }
    }
    println!("no program fund");
}

pub fn second() {
    let data = read_to_string("data/7").unwrap();
    let programs = parse(&data);
    let mut seen_sub_programs: Vec<&str> = Vec::new();
    for p in programs.iter() {
        seen_sub_programs.append(&mut p.sub.clone());
    }
    let mut fist_program: &str = "";
    for p in programs.iter() {
        if !seen_sub_programs.contains(&p.id) {
            fist_program = p.id;
        }
    }
    find_inbalance(fist_program, &programs);

}

fn find_inbalance<'p>(target: &'p str, data: &Vec<Program<'p>>) -> (i64, i64) {
    let target = data.iter().find(|x| x.id == target).unwrap();
    let mut sum: i64 = 0;

    if target.sub.len() != 0 {
        let mut sub_sums: Vec<(i64, i64, &'p str)> = Vec::new();
            
        for sub_target in target.sub.iter() {
            let (sub_n, sub_s) = find_inbalance(&sub_target, data);
            sub_sums.push((sub_n, sub_n + sub_s, sub_target));
        }

        if let Some((w, s, m, c)) = find_diffrent(&sub_sums) {
            let ris = w + m - s;
            println!("{w} {s} {m} {c} {ris}");
        }
        sum += sub_sums.iter().map(|x| x.1).sum::<i64>();
    }

    (target.n, sum)
}

fn find_diffrent<'p>(sums: &Vec<(i64, i64, &'p str)>) -> Option<(i64, i64, i64, &'p str)> {
    if sums.len() < 3 {
        return None;
    }

    let a = sums[0].1;
    let b = sums[1].1;
    let c = sums[2].1;

    let maj = if a == b || a == c {
        a
    } else {
        b
    };

    for (id, e, cc) in sums.iter() {
        if *e != maj {
            return Some((*id, *e, maj, cc));
        }
    }

    None
}

struct Program<'plife> {
    id: &'plife str,
    sub: Vec<&'plife str>,
    n: i64,
}

fn parse<'plife>(data: &'plife str) -> Vec<Program<'plife>> {
    data.lines()
        .filter_map(|x| {
            let parts: Vec<_> = x.split_whitespace().collect();
            match parts.as_slice() {
                [id, n] => Some(Program {
                    id: *id,
                    sub: Vec::new(),
                    n: n[1..n.len() - 1].parse::<i64>().ok()?,
                }),
                [id, n, "->", rest @ ..] => {
                    if rest.len() == 0 {
                        return None;
                    }
                    let sub: Vec<_> = rest
                        .iter()
                        .map(|subp| subp.strip_suffix(',').unwrap_or(subp))
                        .collect();
                    Some(Program {
                        id: *id,
                        sub: sub,
                        n: n[1..n.len() - 1].parse::<i64>().ok()?,
                    })
                }
                _ => None,
            }
        })
        .collect()
}
