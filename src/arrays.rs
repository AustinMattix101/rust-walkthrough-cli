// Array - Fixed list where elements are the same data types

use std::mem;

pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // Re-assign value
    numbers[3] = 20;

    println!("{:?}", numbers);

    // Get sigle val
    println!("Single value: {}", numbers[0]);

    //Get array length
    println!("Array length {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // let slice: &[i32] = &numbers;
    let slice: &[i32] = &numbers[1..3]; // Min = 1, Max = 3 => 1, 2 ?? Not 3

    println!("Slice: {:?}", slice);
}
