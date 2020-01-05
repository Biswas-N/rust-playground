// Conditions - Logical based conditioning

pub fn run() {
    println!("----> Conditionals : See CLI/conditionals.rs <----");

    let age = 18;
    let id_checked = false;

    // Long form
    if age > 18 {
        println!("What do you wanna drink?");
    }
    else if id_checked {
        println!("Sorry buddy!");
    }
    else {
        println!("Show ID")
    }

    // Short form
    if id_checked { println!("Checked!!"); } else { println!("Not Checked!!"); }
}