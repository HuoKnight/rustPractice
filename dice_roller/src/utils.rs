use std::collections::HashMap;
use colored::*;

pub fn crash(code: i32) {
    println!("{}", "Please make sure your arguments are correct.\n Syntax: {command} {args} {{# to roll}d{# sides}}}".red());
    std::process::exit(code);
}

pub fn hashmap_init(upper: u32) -> HashMap<u32, u32> {
    let mut hashmap = HashMap::new();
    for i in 1..=upper {
        hashmap.insert(
            i,
            0,
        );
    }
    
    hashmap
}
