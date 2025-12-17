
fn is_prime(number: usize) -> bool {
    let max = (number as f64).sqrt() as usize;

    for i in 2..=max {
        if number % i == 0{
            return false;
        }
    }
    true
}

pub fn second() {
    let b = 109900;
    let c = 126900;
    let mut h = 0;
    for n in (b..=c).step_by(17) {
        if !is_prime(n) {
            h += 1;
        }
    }

    println!("the number of non prime: {h}");
}
