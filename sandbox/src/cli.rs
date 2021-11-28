pub fn run() {
    let args: Vec<String> = std::env::args().collect();
    println!("Args : {:?}", args);
    let command = args[1].clone();
    println!("Command : {:?}", command);
    let name = "Brad";
    if command == "help" {
        println!("Hello {}", name);
    }
}
