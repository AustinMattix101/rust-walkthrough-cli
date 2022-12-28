// Structs - Used to create custom data types

// // Traditional Struct
// struct Color {
//     red: u8,
//     green: u8,
//     blue: u8,
// }

// // Tuple struct
// struct Color(u8, u8, u8);

struct Person {
    first_name: String,
    last_name: String,
}

impl Person {
    // Construnct person
    fn new(first: &str, last: &str) -> Person {
        Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        }
    }

    // Get Full Name
    fn full_name(&self) -> String {
        format!("{} {}", &self.first_name, &self.last_name)
    }

    // Set last Name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // let mut c = Color {
    //     red: 255,
    //     green: 0,
    //     blue: 0,
    // };

    // c.red = 200;

    // println!("Color: {} {} {}", c.red, c.blue, c.green);

    // let mut c = Color(255, 0, 0);
    // c.0 = 190;

    // println!("Color : {} {} {}", c.0, c.1, c.2);

    let mut p = Person::new("Marry", "Doe");
    println!("Person: {}", p.full_name());
    p.set_last_name("Mattix");
    println!("Person: {}", p.full_name());
    println!("Persion Tuple: {:?}", p.to_tuple());
}
