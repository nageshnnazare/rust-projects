#[macro_use]
#[allow(dead_code)]
extern crate serde_derive;

use std::io;
use std::io::Write;
use std::process;

mod blockchain;

fn main() {
    let mut miner_addr = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    println!("Input miner address");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_addr);

    println!("Input difficulty");
    io::stdout().flush();
    io::stdin().read_line(&mut difficulty);
    let diff = difficulty.trim().parse::<u32>().expect("need an integer");

    println!("Generating Genesis block...");
    let mut chain = blockchain::Chain::new(miner_addr.trim().to_string(), diff);

    loop {
        println!("");
        println!("*** Menu ***");
        println!("1) New Transaction");
        println!("2) Mine Block");
        println!("3) Change Difficulty");
        println!("4) Change reward");
        println!("0) Exit");

        print!("Your Choice : ");
        io::stdout().flush();

        choice.clear();
        io::stdin().read_line(&mut choice);
        println!("");

        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting...");
                process::exit(0);
            }
            1 => {
                let mut sender = String::new();
                let mut receiver = String::new();
                let mut amount = String::new();

                println!("Input sender address : ");
                io::stdout().flush();
                io::stdin().read_line(&mut sender);

                println!("Input receiver address : ");
                io::stdout().flush();
                io::stdin().read_line(&mut receiver);

                println!("Input amount : ");
                io::stdout().flush();
                io::stdin().read_line(&mut amount);

                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    receiver.trim().to_string(),
                    amount.trim().parse().unwrap(),
                );
                match res {
                    true => println!("Transaction added..."),
                    false => println!("Transaction failed !"),
                }
            }
            2 => {
                println!("Generating block...");
                let res = chain.generate_new_block();
                match res {
                    true => println!("Block generated successfully"),
                    false => println!("Block generation failed"),
                }
            }
            3 => {
                let mut new_diff = String::new();
                print!("Enter new difficulty : ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_diff);
                let res = chain.update_difficulty(new_diff.trim().parse().unwrap());
                match res {
                    true => println!("Difficulty updated..."),
                    false => println!("Failed to update difficulty"),
                }
            }
            4 => {
                let mut new_reward = String::new();
                print!("Enter new reward : ");
                io::stdout().flush();
                io::stdin().read_line(&mut new_reward);
                let res = chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => println!("Reward updated..."),
                    false => println!("Failed to update reward"),
                }
            }
            _ => {
                println!("Invalid option, Please retry..")
            }
        }
    }
}
