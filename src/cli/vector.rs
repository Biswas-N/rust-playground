// Vectors - Non-Fixed list of identical elements

pub fn run(){
    println!("----> Vector : See CLI/vector.rs <----");

    let mut numbers: Vec<i32> = vec![1,2,3,4,5]; // Vector "numbers" of type i32

    // Push into Vectors
    numbers.push(6);

    println!("{:?}", (numbers));

    // Access vector elements
    numbers[2] = numbers[2] + 1;
    println!("After update: {}", numbers[2]);

    // Vector functions (Almost similar)
    println!("Length: {}", numbers.len());
    println!("Size: {} bytes", std::mem::size_of_val(&numbers));

    // Slices
    let slice: &[i32] = &numbers[0..3]; // Slice is a pointer, pointing towards a part of vector
    println!("Slice: {:?}", (slice, &numbers[..4]));

    // Loops on vectors
    for i in numbers.iter() {
        println!("{}", i);
    }
}