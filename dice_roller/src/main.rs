use rand::Rng;

fn roll(amount: &str, sides: &str) {
    let iterator = amount.parse::<u32>().unwrap();
    let upper = sides.parse::<u32>().unwrap();
    let mut pool = rand::thread_rng();
    let mut max_count = 0;
    for _die in 1..=iterator {
        let num: u32 = pool.gen_range(1..=upper);
        if num == upper {
            max_count += 1;
        }
        println!("{}: {}", _die, num);
    }
    println!("# Max Rolls: {}", max_count);
    // println!("{}, {}", amount, sides)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    let die: Vec<&str> = args[1].split('d').collect();

    if args[2] == "-v" {
        let verbose = true;
    }

    // println!("{:?}", die);
    println!("Rolling: {}", args[1]);
    roll(die[0], die[1]);
}