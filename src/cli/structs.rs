// Structs - Used to create custom data types (Almost similar to classes)
// Structs --> Similar to classess
// Impl --> Similar to class methods

use std::fmt;

// Traditional struct - More descriptive
struct Color1 {
    red: u8,
    green: u8,
    blue: u8
}

// Tuple struct - Less descriptive
struct Color2(u8, u8, u8);

// Structure functions example
struct Person {
    f_name: String,
    l_name: String
}

impl Person {
    // Construct Person
    fn new(first: &str, last: &str) -> Person {
        Person {
            f_name: first.to_string(),
            l_name: last.to_string()
        }
    }

    // Struct methods
    // GET full name
    fn full_name(&self) -> String {
        format!("{} {}", self.f_name, self.l_name)
    }

    // SET last name
    fn set_l_name(&mut self, new_name: &str) {
        self.l_name = new_name.to_string();
    }
}

impl fmt::Display for Person {
    // Format Person to display
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Person : {}\n[ \n\tFirst Name: {}\n\tLast Name: {}\n]", self.full_name(), self.f_name, self.l_name)
    }
}

pub fn run() {
    println!("----> Structs : See CLI/structs.rs <----");

    traditional();

    tuple();
    
    struct_functions();
}

fn traditional() {
    // Traditional struct playground
    let mut pure_red = Color1 {
        red: 255,
        green: 0,
        blue: 0
    };

    println!("Pure Red: {} {} {}", pure_red.red, pure_red.green, pure_red.blue);

    pure_red.red = 200;

    println!("Not So Pure Red: {} {} {}", pure_red.red, pure_red.green, pure_red.blue);
}

fn tuple() {
    // Tuple struct playground
    let mut tuple_pure_red = Color2(255, 0, 0);

    println!("Tuple - Pure Red: {} {} {}", tuple_pure_red.0, tuple_pure_red.1, tuple_pure_red.2);

    tuple_pure_red.0 = 200;

    println!("Tuple - Not So Pure Red: {} {} {}", tuple_pure_red.0, tuple_pure_red.1, tuple_pure_red.2);
}

fn struct_functions() {
    let mut name = Person::new("Teddy", "Buddy");

    println!("{}", name);
    name.set_l_name("Bunny");
    println!("{}", name);
}