pub fn run() {
    // Print console
    println!("Hello from print.rs file");

    // Basic formatting
    println!("{} is from {}", "Brad", "Mass");

    // Positional Arguments
    println!(
        "{0} is from {1} and {0} like to {2}",
        "Austin", "Mass", "code"
    );

    // Named Arguments
    println!(
        "{name} like to play {activity}",
        name = "Jonh",
        activity = "Baseball"
    );

    // Placeholder traits
    println!("Binary {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholer for debug traits
    println!("{:?}", (12, true, "hello"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}
