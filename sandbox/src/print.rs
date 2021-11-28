pub fn run() {
    // print to console
    println!("Hello from the print.rs file..");

    // basic formatting
    println!("{} is from {}", "Brad", "Mars");

    // positional arguments
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Brad", "Mars", "code"
    );

    // Named Arguments
    println!(
        "{name} likes to play {activity} ",
        name = "john",
        activity = "baseball"
    );

    // placeholder traits
    println!("bin: {:b}, hex: {:x}, oct: {:o}", 10, 10, 10);

    // placeholder for debug trait
    println!("{:?}", (12, false, "hello"));

    // basic math
    println!("{} + {} = {}", 10, 10, 10 + 10);
}
