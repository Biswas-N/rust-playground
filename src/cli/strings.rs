/*
    Two types:

    str : Primitive immutable, fixed length string
    String : Heap allocated, Mutable
*/

pub fn run() {
    println!("----> Strings : See CLI/strings.rs <----");

    // Normal str type
    let f_name = "Biswas";
    let l_name = "Nandamuri";

    // String type
    let mut greeting = String::from("Hello, ");
    greeting.push_str(f_name); // To push a string, "push" just for a character
    greeting.push(' '); // To push just for a character
    greeting.push_str(l_name);

    // String functions
    // Length
    println!("Lengths: {:?}", (greeting.len(), l_name.len()) );

    // Capacity: max holding size
    println!("Capacities: {:?}", (greeting.capacity()) ); // No "capacity" for l_name, as it is &str

    // Is Empty
    println!("Empty: {:?}", (greeting.is_empty(), l_name.is_empty()) );

    // Contains: find a substring
    println!("Contains: {:?}", (greeting.contains("Hello"), l_name.contains("Hello")) );

    // Replace: find a substring and replace it (not persisted as not saved)
    println!("Replace: {:?}", (greeting.replace("Hello", "Hola"), l_name.replace("Hello", "Hola")) );

    // Loops on strings
    for word in greeting.split(' ') {
        println!("{}", word)
    }

    // Create string with capacity
    let mut s: String = String::with_capacity(10);
    s.push('A');
    s.push('C');
    s.push('D');
    s.push('C');

    // Assertion testing
    assert_eq!(s, "ACDC"); // Displays error if left and right do-not match

    println!("{:?}", (greeting) );
}