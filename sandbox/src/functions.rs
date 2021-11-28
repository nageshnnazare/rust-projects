pub fn run() {
    greeting("Hello", "Jane");
    println!("res = {}", add(10, 20));
    // closure
    let add_num = |n1: i32, n2: i32| n1 + n2;
    println!("c sum : {}", add_num(10, 10));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {}, nice to meet you", greet, name);
}

fn add(num1: i32, num2: i32) -> i32 {
    return num1 + num2;
}
