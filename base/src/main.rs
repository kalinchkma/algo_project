
use std::fmt;

struct Model {
    name: String,
    age: u32,
}

impl fmt::Display for Model {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Age: {}", self.name, self.age)
    }
}


fn hello_world() {
    println!("Hello world");
}

fn format_greetings(name: &str) -> String {
    return format!("Welcome to the new world, {}!", name);
}

fn number_printing() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    eprint!("Error message\n");
    println!("Base 10             {}", 2310);
    println!("Base 2              {:b}", 2310);
    println!("Base 8              {:o}", 2310);
    println!("Base 16             {:x}", 2310);
    println!("Base 16 (uppercase) {:X}", 2310);
    println!("Scientific notation {:.2e}", 2310.0);
    println!("Decimal point       {:.2}", 2310.0);
    println!("Debug               {:?}", (3, 4));
    println!("Pretty debug        {:#?}", (3, 4));
    println!("Pointer             {:p}", &2310);
    println!("Binary              {:b}", 2310);
    println!("Octal               {:o}", 2310);
    println!("Lower Hex           {:x}", 2310);
    println!("Upper Hex           {:X}", 2310);
    println!("Point: {:#?}", Point { x: 3, y: 4 });

    println!("Model: {}", Model {name: "Albert".to_string(), age: 34});
}



fn main() {
    hello_world();
    println!("{}",format_greetings("Cyper"));
    number_printing();
}
