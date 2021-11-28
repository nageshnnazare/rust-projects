/*
    Vectors are resizable arrays.
*/

pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    println!("{:?}", numbers);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // reassign value
    numbers[2] = 10;

    // adding elements
    numbers.push(20);
    numbers.push(30);

    // removing elements
    numbers.pop();

    // Get array length
    println!("Vector length: {}", numbers.len());
    // Arrays are stack allocated
    println!("Vector occupies {} bytes", std::mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[1..4];
    println!("Slice: {:?}", slice);

    // Loop through array values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }
    println!("Number: {:?}", numbers);
}
