// Value group of non-identical elements
// Max 12 elements

pub fn run() {
    println!("----> Tuples : See CLI/tuples.rs <----");

    let person: (&str, &str, i8) = ("Biswas", "Hyd", 25);

    println!("{} is from {} and is {}", person.0, person.1, person.2);
}