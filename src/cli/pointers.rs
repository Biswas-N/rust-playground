// Pointers or Reference pointers - Point to a resource in memory

pub fn run() {
    println!("----> Pointers : See CLI/pointers.rs <----");

    // ---> Primitive array pointer
    let arr1 = [1,2,3,4,5]; // Allocates a block of memory and "arr1" holds the start point of the memory
    let mut arr2 = arr1;

    // Before edit
    println!("{:?}", (arr1, arr2));

    // Edit
    arr2[2] = arr2[2] * 2;

    // After edit
    println!("Array: {:?}", (arr1, arr2));

    // ---> Vector pointers
    let vec1 = vec![1,2,3,4,5];
    let vec2 = &vec1;

    println!("Vector: {:?}", (&vec1, vec2));


}