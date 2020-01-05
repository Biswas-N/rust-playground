// Loops - Repetition of code block based on the condition

pub fn run() {
    println!("----> Loops : See CLI/loops.rs <----");

    let mut count = 0;

    // Infinite loop
    loop {
        count += 1;
        println!("{}", count);

        if count == 20 { break; }
    }

    // While loop (FizzBuzz)
    count = 1;
    while count <= 100 {
        if count % 15 == 0 {
            println!("FizzBuzz");
        } else if count % 3 == 0 {
            println!("Fizz");
        } else if count % 5 == 0 {
            println!("Buzz");
        } else {
            println!("{}", count);
        }

        count += 1;
    }

    // For range loop
    for i in 0..100 { // 0..100 ranges from 0(inclusive) to 100(exclusive)
        println!("{}", i);
    }
}