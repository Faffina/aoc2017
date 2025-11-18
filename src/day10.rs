pub fn first() {
    let lengst: [u8; 16] = [
        63, 144, 180, 149, 1, 255, 167, 84, 125, 65, 188, 0, 2, 254, 229, 24,
    ];
    let mut list: [u64; 256] = std::array::from_fn(|x| x as u64);
    let mut possiton: usize = 0;
    let mut skip: usize = 0;

    for l in lengst {
        revers(&mut list, possiton, l);
        possiton = (possiton + l as usize + skip) % list.len();
        skip += 1;
    }

    let ris = list[0] * list[1];
    println!("{ris}");
}

fn revers(list: &mut [u64; 256], start: usize, lenght: u8) {
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

pub fn second() {
    let mut lengst: Vec<u8> = "63,144,180,149,1,255,167,84,125,65,188,0,2,254,229,24"
        .as_bytes()
        .to_vec();
    lengst.extend(vec![17, 31, 73, 47, 23]);
    let mut list: [u64; 256] = std::array::from_fn(|x| x as u64);
    let mut possiton: usize = 0;
    let mut skip: usize = 0;

    for _ in 0..64 {
        for l in lengst.iter() {
            revers(&mut list, possiton, *l);
            possiton = (possiton + *l as usize + skip) % list.len();
            skip += 1;
        }
    }

    let dens_hash: Vec<u64> = list
        .chunks(16)
        .map(|x| x.iter().fold(0, |a, &x| a ^ x))
        .collect();
    for i in dens_hash {
        print!("{i:02x}")
    }
    println!("");
}
