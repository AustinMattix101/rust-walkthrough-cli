use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);

    if args.len() == 1 {
        println!("\n đ Welcome to Curious CLI");
        println!(" đ§° Version 1.0.0");
        println!(" đĻ Powered by Rust Lang");
        println!("\n > đ Please provide a valid command!");
        println!("\n > â -h, --help > to navigate to help command!");
    } else {
        let command: String = args[1].clone();
        // println!("> đ Commands: {}", command);

        let name = "Austin";
        let status = "100%";
        let help: Vec<&str> = vec![
            "hello > to get Hello prompt.",
            "status > to get status percentages.",
            "-h, --help this help command!",
        ];

        if command == "hello" {
            println!("> đ Hi {}, how are you?", name);
        } else if command == "status" {
            println!("> đĒĢ Status is {}", status);
        } else if command == "-h" || command == "--help" {
            println!("\n> đĢŗ Help:");

            for x in help.iter() {
                println!("đ {}", x);
            }

            println!("\n đ Welcome to Curious CLI");
            println!(" đ§° Version 1.0.0");
            println!(" đĻ Powered by Rust Lang");
        } else {
            println!("> â That is not a valid command!");
        }
    }
}
