// Fixed list of identical elements

pub fn run(){
    println!("----> Arrays : See CLI/arrays.rs <----");

    let mut numbers: [i32; 5] = [1,2,3,4,5]; // Array "numbers" of type i32 and size 5

    println!("{:?}", (numbers));

    // Access array elements
    numbers[2] = numbers[2] + 1;
    println!("After update: {}", numbers[2]);

    // Array functions
    println!("Length: {}", numbers.len());
    println!("Size: {} bytes", std::mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[0..3]; // Slice is a pointer, pointing towards a part of array
    println!("Slice: {:?}", (slice, &numbers[1..4]));

    // Loops on Arrays
    for i in numbers.iter() {
        println!("{}", i);
    }
}