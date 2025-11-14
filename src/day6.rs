use std::collections::HashSet;

pub fn first() {
    let mut fist_state = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut seen = HashSet::new();
    let mut steps = 0;
    while seen.insert(fist_state.clone()) {
        let max = fist_state.iter().max().unwrap();
        let max_index = fist_state.iter().position(|x| x == max).unwrap();
        let red = fist_state[max_index];
        fist_state[max_index] = 0;
        for i in 1..=red {
            let index = (max_index + i) % fist_state.len();
            fist_state[index] += 1;
        }
        steps += 1;
    }
    println!("{steps}");
}

pub fn second() {
    let mut fist_state = [4, 1, 15, 12, 0, 9, 9, 5, 5, 8, 7, 3, 14, 5, 12, 3];
    let mut seen = HashSet::new();
    let mut steps = 0;
    while seen.insert(fist_state.clone()) {
        let max = fist_state.iter().max().unwrap();
        let max_index = fist_state.iter().position(|x| x == max).unwrap();
        let red = fist_state[max_index];
        fist_state[max_index] = 0;
        for i in 1..=red {
            let index = (max_index + i) % fist_state.len();
            fist_state[index] += 1;
        }
    }
    let find_this = fist_state.clone();
    let max = fist_state.iter().max().unwrap();
    let max_index = fist_state.iter().position(|x| x == max).unwrap();
    let red = fist_state[max_index];
    fist_state[max_index] = 0;
    for i in 1..=red {
        let index = (max_index + i) % fist_state.len();
        fist_state[index] += 1;
    }
    steps += 1;
    while find_this != fist_state {
        let max = fist_state.iter().max().unwrap();
        let max_index = fist_state.iter().position(|x| x == max).unwrap();
        let red = fist_state[max_index];
        fist_state[max_index] = 0;
        for i in 1..=red {
            let index = (max_index + i) % fist_state.len();
            fist_state[index] += 1;
        }
        steps += 1;
    }
    println!("{steps}");
}
