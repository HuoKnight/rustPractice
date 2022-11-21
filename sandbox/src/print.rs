pub fn run() {
    // Print to console
    println!("Hello from the print.rs file!");

    // Formatting
    println!("{0} is from {1} and {0} likes to {2}.", "HuoKnight", "the Moon", "code");

    // Named Arguments
    println!("{name} likes to play {game}", name = "HuoKnight", game = "TF2");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placehodler for debug trait
    let a = (12, true, "Hello");
    println!("{:?}", a);

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}