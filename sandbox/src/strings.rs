/*
    Primitive str = immutable fixed-length somewhere in memory
    String = growable, heap-allocated data structure -
    Use when you need to modify or own string data
*/

pub fn run() {
    let mut hello = String::from("Hello ");
    println!("Length : {}", hello.len());

    hello.push('W');
    hello.push_str("orld");

    println!("String : {}", hello);

    println!("Capacity : {}", hello.capacity());

    println!("Is Empty : {}", hello.is_empty());
    println!("Contains 'World' : {}", hello.contains("World"));

    println!("Replace : {}", hello.replace("World", "There"));

    for word in hello.split_whitespace() {
        println!("Word : {}", word);
    }

    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    println!("{}", s);

    assert_eq!(2, s.len());
}
