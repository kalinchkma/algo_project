#![allow(dead_code)]
// This file contains basic rust programming concepts
use std::fmt::{self};

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


fn primitives_types() {
    let logical: bool = true;
    let a_float: f64 = f64::MAX;
    let an_integer: i32 = i32::MAX;
    let tup: (i32, String, bool) = (26, "Colin".to_string(), true);
    println!("logical: {},\n float: {},\n integer: {}", logical, a_float, an_integer);
    println!("Name: {}, age: {}, has dick: {}", tup.0, tup.1, tup.2);
    println!("Scientific notation: {} minus {}", 1e4, 1e-10);
    println!("Or operator {}", false || true);
    println!("And operator {}", true && true);
    println!("1 - 2 = {}", 1i32 - 2 );
    // bitwise operator
    println!("0011 AND 0101 is {:04b}", 0b0011u32 & 0b0101);
    println!("0011 OR 0101 is {:04b}", 0b0011 | 0b0101);
    println!("0011 XOR 0101 is {:04b}", 0b0011 ^ 0b0101);
    println!("1 << 5 is {:b}", 0b001 << 0b0001);
    println!("0011 >> 1010 {:08b}", 0b0011 >> 0b1010);
    println!("One million: {}", 1_00_000u32);

    // tuples
    fn reverse(pair: (i32, bool)) -> (bool, i32) {
        let (int_param, bool_param) = pair;
        (bool_param, int_param)
    }

    #[derive(Debug)]
    struct Matrix(f32, f32, f32, f32);

    let long_tuple = (1u8, 2u16, 3u32, 4u64, -1i8);
    let tuple_of_tuple = ((1u8, 2u16, 3u32), (4u64, -1i8), -2i16);

    println!("{:#?}", tuple_of_tuple);
    let matrix = Matrix(1.2, 3.4, 4.1, 4.5);
    println!("{:#?}", matrix);
    let tup = (2, false);
    println!("Reverse tupe {:?}", reverse(tup));
    fn reverse_heap_data(pair: (String, Vec<i32>)) -> (Vec<i32>, String) {
        let (string_param, vec_param) = pair;
        (vec_param, string_param)
    }

    println!("Long tuple: {:?}", long_tuple);
    let heap_data = (String::from("Hello"), vec![1, 2, 3, 4, 5]);
    println!("{:?}", reverse_heap_data(heap_data));

    // arrays and slices
    use std::mem;

    let mut rent: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let roll: [i128; 500] = [0; 500];

    let one = 4e3;

    rent[0] = 100;
    println!("First element of rent: {}", rent[0]);
    println!("Second element of rent: {}", rent[1]);

    println!("Lenght of roll: {}", roll.len());
    println!("Size of roll: {} bytes", mem::size_of_val(&roll));
    println!("Size of one: {} bytes\nvalue {}", mem::size_of_val(&one), one);

    println!("Slice of rent: {:?}", &rent[5..8]);

    // unsfae access
    // println!("Out of bound access: {:?}", rent[11]); // error
    // accessing array safely
    let mut out_element = 0;
    match rent.get(9) {
            Some(e) => out_element = *e,
            None => println!("Error: {}", "Out of bound")        
    };
    println!("Out of bound access: {:?}", out_element);
    

}



fn custom_types() {
    // struct
    struct Location {
        address: String,
        longitide: f64,
        latitude: f64,
        zip_code: u32
    }

    struct User {
        name: String,
        email: String,
        age: u32,
        location: Location
    }

    impl fmt::Display for User {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // write!(f, "Name: {}, Email: {}, Age: {}, Location: {}", self.name, self.email, self.age, self.location)
            write!(f, "Name: {}, Email: {}, Age: {},\nLocation: \naddress: {}\nlongitide: {}\nlatitude: {}\nzip_code: {}\n", self.name, self.email, self.age, self.location.address, self.location.longitide, self.location.latitude, self.location.zip_code)
        }
    }

    let peter = User {
        name: "Peter".to_string(),
        age: 43u32,
        email: "peter@yahoo.com".to_string(),
        location: Location {
            address: "123, 4th Avenue".to_string(),
            longitide: 34.234,
            latitude: 23.234,
            zip_code: 234
        }
    };

    let miss_peter = User {
        name: "Miss Peter".to_string(),
        age: 32u32,
        email: "miss@hotmail.com".to_string(),
        ..peter
    };

    println!("People: {}", miss_peter);
    let User {name: peter_wife, email: wife_email, age: wife_age, location: current_location} = miss_peter;

    println!("Wife: {}, Email: {}, Age: {}, Location: {}", peter_wife, wife_email, wife_age, current_location.address);

    // tuple struct
    #[derive(Debug)]
    struct Unit;
    #[derive(Debug)]
    struct Coordinates(f64, f64);

    let house_unit = Unit;

    let house_location = Coordinates(34f64, 34f64);

    println!("House unit: {:?}", house_unit);
    println!("House location: {:?}", house_location);

    struct Rectangle {
        width: f64,
        height: f64
    }

    impl Rectangle {
        fn new(width: f64, height: f64) -> Rectangle {
            Rectangle {
                width,
                height
            }
        }

        fn area(&self) -> f64 {
            self.width * self.height
        }
    }

    impl fmt::Display for Rectangle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "Width: {}, Height: {}\nArea: {}", self.width, self.height, self.area())
        }
    }

    let field = Rectangle::new(34.0, 23.0);

    println!("Field: {}", field);

    // enums
    enum WebEvent {
        PageLoad,
        PageUnload,
        KeyPress(char),
        Paste(String),
        Click {x: i64, y: i64}
    }

    fn inspect(event: WebEvent) {
        match event {
            WebEvent::KeyPress(c) => println!("Key press: {}", c),
            WebEvent::PageLoad => println!("Page loaded"),
            WebEvent::PageUnload => println!("Page unloaded"),
            WebEvent::Paste(s) => println!("Paste: {}", s),
            WebEvent::Click { x, y } => println!("Click at x: {}, y: {}", x, y)
        }
    }

    let pressed = WebEvent::KeyPress('x');
    let click = WebEvent::Click {x: 34, y: 23};

    inspect(click);
    inspect(pressed);

}


pub fn run() {
    // hello_world();
    // println!("{}",format_greetings("Cyper"));
    // number_printing();
    // primitives_types()
    custom_types();
}
