use rand::Rng;
use std::collections::HashMap;

mod utils;

fn roll(amount: &str, sides: &str, arguments: HashMap<&str, bool>) {
    match amount.parse::<u32>() {
        Ok(_v) => {},
        Err(_e) => utils::crash(1),
    }
    match sides.parse::<u32>() {
        Ok(_v) => {},
        Err(_e) => utils::crash(1),
    }
    let iterator = amount.parse::<u32>().unwrap();
    let upper = sides.parse::<u32>().unwrap();
    let mut pool = rand::thread_rng();
    // let mut max_count = 0;
    let mut result_index = utils::hashmap_init(upper);
    let mut results: Vec<u32> = Vec::new();
    for _die in 1..=iterator {
        let num: u32 = pool.gen_range(1..=upper);
        results.push(num);
        *result_index.get_mut(&num).unwrap() += 1;
        // if num == upper {
        //     max_count += 1;
        // }
        // println!("{}: {}", die, num);
    }
    // println!("# Max Rolls: {}", max_count);


    if arguments["average"] == false {
        print!(" \"#\": Times Rolled\nJSON:\n{{");
        let mut result_index_vec: Vec<(&u32, &u32)> = result_index.iter().collect();
        result_index_vec.sort_by(|a, b| a.cmp(b));
        for pair in result_index_vec {
            print!("\n \"{}\": {},", pair.0.to_string(), pair.1);
        }
        println!("\n}}");
    }
    else {
        let average: f32 = results.iter().sum::<u32>() as f32 / results.len() as f32;
        println!("Average of rolls: {}", average);
    }
    
    // println!("{:?}", result_index);
    // println!("{}, {}", amount, sides)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    let die: Vec<&str> = args.last().unwrap().split('d').collect();


    let mut switches = HashMap::from([
        ("average", false),
    ]);
    for arg in &args {
        if arg == "-a" {
            switches.insert(
                "average",
                true,
            );
        }
    }
    // println!("{:?}", switches);
    // println!("{:?}", die);
    println!("Rolling: {}", &args.last().unwrap());
    let test_if_die = &die.len();
    match test_if_die {
        2 => if &die[1] == &"" {
            utils::crash(1);
        },
        _ => utils::crash(1),
    };
    roll(die[0], die[1], switches);
}