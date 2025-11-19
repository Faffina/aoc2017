use std::sync::mpsc::{self, Sender};
use std::thread;

pub fn first() {
    let mut a: usize = 703;
    let mut b: usize = 516;

    let mut count: usize = 0;
    for _ in 0..40_000_000 {
        a = (a * 16807) % 2147483647;
        b = (b * 48271) % 2147483647;
        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }

    println!("{count}")
}

fn generator(tx: Sender<usize>, mut value: usize, factor: usize, dividor: usize) {
    for _ in 0..40_000_000 {
        value = (value * factor) % 2147483647;
        if value % dividor == 0 {
            tx.send(value).unwrap();
        }
    }
}

pub fn second() {
    let (atx, arx) = mpsc::channel::<usize>();
    let (btx, brx) = mpsc::channel::<usize>();

    let a_thread = thread::spawn(|| generator(atx, 703, 16807, 4));
    let b_thread = thread::spawn(|| generator(btx, 516, 48271, 8));

    let mut count = 0;
    for (a, b) in arx.iter().zip(brx.iter()) {
        if (a & 0xffff) == (b & 0xffff) {
            count += 1;
        }
    }
    a_thread.join().unwrap();
    b_thread.join().unwrap();
    println!("{count}");
}
