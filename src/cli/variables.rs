// Variables are immutable by default

pub fn run() {
    println!("----> Variables : See CLI/variables.rs <----");

    let name = "Biswas";
    // name = "Biswas Nandamuri"; // Cannot assign twice to immutable variable

    let mut age = 24;
    age = age + 1;
    println!("My name is {} and I am {} years old", name, age);

    // Define constant
    const ID: i32 = 001;
    println!("ID: {}", ID);

    // Assign multiple vars
    let (my_name, my_age) = ("Brad", 37);
    println!("{} is {}", my_name, my_age);
}
