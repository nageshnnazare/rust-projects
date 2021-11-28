/*  Variables hold primitive data or references to data
   Variables are immutable by default
   Rust is a block scoped language
*/
pub fn run() {
    // default var
    let name = "Brad";

    // mutable var
    let mut age = 37;
    println!("My name is {} and I am {}", name, age);

    age = 40;
    println!("My name is {} and I am {}", name, age);

    // const var
    const ID: i32 = 001;
    println!("id : {}", ID);

    // assign multiple var
    let (my_name, my_age) = ("BRAD", 37);
    println!("{:?}", (my_age, my_name));
}
