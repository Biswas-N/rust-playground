// Functions - Used to store a block of code for re-use

pub fn run() {
    println!("----> Functions : See CLI/functions.rs <----");

    let msg = greeting("Good morning", "Mr. Biswas Nandamuri"); // Function call
    println!("{}", msg);

    // Closures: In-line functions (like lambdas in python)
    let add_num = | n1: i32, n2: i32 | n1+n2 ;
    println!("{}", add_num(10, 20));
}

// Function definition
// Accepts two str's
// Returns String type
fn greeting(greet: &str, name: &str) -> String {
    // Can be returned by not using a ";"
    format!("{} {}", greet, name) // Format is used to create strings from variables
}