use rand::Rng;
use std::collections::HashMap;
use colored::*;

fn crash(code: i32) {
    println!("{}", "Please make sure your arguments are correct.\n Syntax: {command} {args} {{# to roll}d{# sides}}}".red());
    std::process::exit(code);
}

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

fn roll(amount: &str, sides: &str, arguments: HashMap<&str, bool>) {
    match amount.parse::<u32>() {
        Ok(_v) => println!("OK"),
        Err(_e) => crash(1),
    }
    match sides.parse::<u32>() {
        Ok(_v) => println!("OK"),
        Err(_e) => crash(1),
    }
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
    if arguments["print_results"] == true {
        for (key, val) in result_index {
            println!("{} - {}", key, val);
        }
    }
    // println!("{:?}", result_index);
    // println!("{}, {}", amount, sides)
}


fn main() {
    let args: Vec<String> = std::env::args().collect();
    // println!("{:?}", args);
    let die: Vec<&str> = args.last().unwrap().split('d').collect();


    let mut switches = HashMap::from([
        ("print_results", false),
        ("count_one", false),
    ]);
    for arg in &args {
        if arg == "-r" {
            switches.insert(
                "print_results",
                true,
            );
        }
        else if arg == "-c" {
            switches.insert(
                "count_one",
                true,
            );
        }
    }
    println!("{:?}", switches);
    // println!("{:?}", die);
    println!("Rolling: {}", &args.last().unwrap());
    let test_if_die = &die.len();
    match test_if_die {
        2 => if &die[1] == &"" {
            crash(1);
        },
        _ => crash(1),
    };
    roll(die[0], die[1], switches);
}