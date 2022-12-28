use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    // println!("Args: {:?}", args);

    if args.len() == 1 {
        println!("\n 🎉 Welcome to Curious CLI");
        println!(" 🧰 Version 1.0.0");
        println!(" 🦀 Powered by Rust Lang");
        println!("\n > 🔘 Please provide a valid command!");
        println!("\n > ❔ -h, --help > to navigate to help command!");
    } else {
        let command: String = args[1].clone();
        // println!("> 🔘 Commands: {}", command);

        let name = "Austin";
        let status = "100%";
        let help: Vec<&str> = vec![
            "hello > to get Hello prompt.",
            "status > to get status percentages.",
            "-h, --help this help command!",
        ];

        if command == "hello" {
            println!("> 👋 Hi {}, how are you?", name);
        } else if command == "status" {
            println!("> 🪫 Status is {}", status);
        } else if command == "-h" || command == "--help" {
            println!("\n> 🫳 Help:");

            for x in help.iter() {
                println!("🔘 {}", x);
            }

            println!("\n 🎉 Welcome to Curious CLI");
            println!(" 🧰 Version 1.0.0");
            println!(" 🦀 Powered by Rust Lang");
        } else {
            println!("> ❌ That is not a valid command!");
        }
    }
}
