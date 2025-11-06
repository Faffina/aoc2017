use std::fs::read_to_string;

pub fn first() {
    let digits: Vec<_> = read_to_string("data/1")
        .unwrap()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect();
    let mut sum = 0;
    for v in digits.windows(2) {
        let one = v[0];
        let two = v[1];
        if one == two {
            sum += one;
        }
    }
    if digits.last().unwrap() == digits.first().unwrap() {
        sum += digits[0];
    }
    println!("part one: {}", sum);
}

pub fn second() {
    let digits: Vec<_> = read_to_string("data/1")
        .unwrap()
        .chars()
        .filter_map(|x| x.to_digit(10))
        .collect();
    let mut sum = 0;
    for (i, v) in digits.iter().enumerate() {
        let one = *v;
        let two_index = (i + digits.len() / 2) % digits.len();
        let two = digits[two_index];
        if one == two {
            sum += one;
        }
    }
    println!("part two: {}", sum);
}
