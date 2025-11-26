
pub fn first() {
    let step = 369;
    let mut possiton = 0;
    let mut buffer: Vec<usize> = Vec::new();
    buffer.push(0);

    for i in 1..=2017 {
        possiton = (possiton + step) % buffer.len() + 1;
        buffer.insert(possiton, i);
    }

    possiton = (possiton + 1) % buffer.len();
    let value = buffer[possiton];

    println!("{value}");
}

pub fn second() {
    let step = 369;
    let mut possiton = 0;
    let mut value = 0;
    let mut size = 1;

    for i in 1..= 50000000{
        possiton = (possiton + step) % size + 1;

        if possiton == 1 {
            value = size;
        }

        size += 1;
    }

    println!("{value}");
}
