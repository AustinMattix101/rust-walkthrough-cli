// (1) Primitive str = Immutable fixed-length string somewhere in memory
// (2) String = Growable, heap-allocated data structor = Use when you need to modify or own string date

pub fn run() {
    let hello = "Hello"; // (1)
    let mut string = String::from("Hello String! ");

    // Get length
    println!("Length: {}", hello.len());

    // Push char
    string.push('W');

    // Push string
    string.push_str("orld!");

    // Capacity in bytes
    println!("Capacity: {}", string.capacity());

    // Check Is Empty
    println!("Is Empty: {}", string.is_empty());

    // Contains
    println!("Contains 'World' {}", string.contains("World"));

    // Replace
    println!("Replace: {}", string.replace("String!", "my"));

    // Loop throught string by whitespace
    for word in string.split_whitespace() {
        println!("Word: {}", word)
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');
    s.push('c');

    println!("{}", s);

    // Asserting testing
    assert_eq!(3, s.len());
    // assert_eq!(11, s.capacity());
    assert_eq!(10, s.capacity());
    // assert_eq!(2, s.len());

    println!("&str: {}, String: {}", hello, string);
}
