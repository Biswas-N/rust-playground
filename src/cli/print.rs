pub fn run() {
    // Print a string
    println!("----> Print : See CLI/print.rs <----");

    // Basic formatting
    println!("{} is {}", "This", "formatting");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Biswas", "Hyderabad", "Code"
    );

    // Named Arguments
    println!("{name} is from {city}", name = "Biswas", city = "Hyderabad");

    // Placeholder traits
    println!("Binary: {:b}, Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Debug Placeholder trait
    println!("{:?}", (12, true, "Hello"));
}
