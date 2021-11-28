pub fn run() {
    // primitive arrays
    let arr1 = [1, 2, 3];
    let arr2 = arr1;

    println!("{:?}", arr1);
    println!("{:?}", arr2);

    // Non-primitive
    let vec1 = vec![4, 5, 6];
    let vec2 = &vec1;

    println!("{:?}", vec1);
    println!("{:?}", vec2);
}
