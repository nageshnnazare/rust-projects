/*
   Arrays - fixed list where elements are the same data type
*/

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());
    
    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);
    
    // Loop through array values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }
    
}
