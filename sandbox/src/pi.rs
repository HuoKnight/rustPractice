use std::time::Instant;

pub fn calc_pi() {
    let start_time = Instant::now();
    let mut num: f64 = 0.0;
    const UBOUND: u128 = 100000000000; // Bigger for more accuracy, smaller for less.
    let mut sign: f64 = -1.0;

    for i in 1..=UBOUND {
        if (i % 2) != 0 {
            num -= sign * (1.0/(i as f64));
            sign = 0.0 - sign;
        }
    }
    
    let pi: f64 = 4.0 * num;

    println!("pi estimate: {:.64}\nExecuted in {:?}.", pi, start_time.elapsed());

}