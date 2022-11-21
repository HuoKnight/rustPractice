pub fn run() {
    let mut hello = String::from("Hello ");

    println!("Length: {}", hello.len());

    hello.push('W');

    hello.push_str("orld");
    println!("{}", hello);

    for word in hello.split_whitespace() {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertions
    assert_eq!(2, s.len());

    println!("{}", s)
}