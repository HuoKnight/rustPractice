use rand::Rng;
use std::collections::HashMap;

fn hashmap_init(upper: u32) -> HashMap<u32, u32> {
    let mut hashmap = HashMap::new();
    for i in 1..=upper {
        hashmap.insert(
            i,
            0,
        );
    }
    
    hashmap
}

fn roll(amount: &str, sides: &str) {
    let iterator = amount.parse::<u32>().unwrap();
    let upper = sides.parse::<u32>().unwrap();
    let mut pool = rand::thread_rng();
    let mut max_count = 0;
    let mut result_index = hashmap_init(upper);
    for die in 1..=iterator {
        let num: u32 = pool.gen_range(1..=upper);
        *result_index.get_mut(&num).unwrap() += 1;
        if num == upper {
            max_count += 1;
        }
        println!("{}: {}", die, num);
    }
    println!("# Max Rolls: {}", max_count);

    // println!("{:?}", result_index);
    // println!("{}, {}", amount, sides)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    let die: Vec<&str> = args[1].split('d').collect();

    // if args[2] == "-v" {
    //     let verbose = true;
    // }

    // println!("{:?}", die);
    println!("Rolling: {}", args[1]);
    roll(die[0], die[1]);
}