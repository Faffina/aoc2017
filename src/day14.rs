use std::collections::VecDeque;

pub fn first() {
    let input = "amgozmfv";
    let mut used = 0;
    for i in 0..128 {
        used += hash(&format!("{input}-{i}")).count_ones()
    } 
    println!("used {used}")
}

pub fn second() {
    let input = "amgozmfv";
    let mut grid = [0u128; 128];
    for i in 0..128 {
        grid[i] = hash(&format!("{input}-{i}"))
    } 

    let mut number_of_grups = 0;
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();

    for y in 0..128 {
        for x in 0..128 {
            if grid[y] & (1 << x) != 0{
                number_of_grups += 1;
                q.push_back((y, x));

                while let Some((y, x)) = q.pop_front() {
                   if grid[y] & (1 << x) != 0 {
                       grid[y] &= !(1 << x);
                       if x != 0 {
                           q.push_back((y, x - 1))
                       }
                       if x != 127 {
                           q.push_back((y, x + 1))
                       } 
                       if y != 0 {
                           q.push_back((y - 1, x))
                       }
                       if y != 127 {
                           q.push_back((y + 1, x));
                       }
                   } 
                }
            }
        }
    }

    println!("{number_of_grups}")
}

fn revers(list: &mut [u8; 256], start: usize, lenght: u8) {
    let n = list.len();

    if n <= 1 {
        return;
    }

    for i in 0..(lenght as usize / 2) {
        let s = (i + start) % n;
        let e = (start as i64 + lenght as i64 - i as i64 - 1) as usize % n;
        list.swap(s, e);
    }
}

pub fn hash(data: &str) -> u128 {
    let mut lengths: Vec<u8> = data.bytes().collect();
    lengths.extend_from_slice(&[17, 31, 73, 47, 23]);
    let mut list: [u8; 256] = std::array::from_fn(|x| x as u8);
    let mut possiton: usize = 0;
    let mut skip: usize = 0;
    for _ in 0..64 {
        for &l in &lengths {
            revers(&mut list, possiton, l);
            possiton = (possiton + l as usize + skip) % list.len();
            skip += 1;
        }
    }
    let dens_hash: [u8; 16] = list
        .chunks(16)
        .map(|x| x.iter().fold(0, |a, &x| a ^ x))
        .collect::<Vec<u8>>()
        .try_into()
        .unwrap();
    u128::from_be_bytes(dens_hash)
}
