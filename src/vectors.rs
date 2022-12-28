// Vectors - Resizable Arrays

use std::mem;

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5]; // [1, 2, 3, 4, 5].to_vec();

    // Re-assign value
    numbers[3] = 20;

    //Add on to Vector
    numbers.push(23);
    numbers.push(17);

    // Pop off last value
    numbers.pop();

    println!("{:?}", numbers);

    // Get sigle val
    println!("Single value: {}", numbers[0]);

    //Get vector length
    println!("Vector length {}", numbers.len());

    // Vectors are stack allocated
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get Slice
    // let slice: &[i32] = &numbers;
    let slice: &[i32] = &numbers[1..3]; // Min = 1, Max = 3 => 1, 2 ?? Not 3

    println!("Slice: {:?}", slice);

    // Loop through vector
    for x in numbers.iter() {
        println!("Numbers: {}", x);
    }

    // Loop & mutate value
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Numbers: {:?}", numbers);
}
