/*
    Tuples group together values of different types.
    Max size is limited to 12 elements.
*/

pub fn run() {
    let person: (&str, &str, i8) = ("John", "Doe", 32);
    println!("Person : {:?}", person);

    println!("Person : {} {} {}", person.0, person.1, person.2);

}
