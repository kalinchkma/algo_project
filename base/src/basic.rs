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

    enum TheEventOfTheNewYearAndTheEventOfTheLifeOfOurNewProgramingLanguage {
        Dance,
        Sing
    }

    // alias long name with simal
    type Event = TheEventOfTheNewYearAndTheEventOfTheLifeOfOurNewProgramingLanguage;

    impl Event {
        fn inspect(&self) {
            match self {
                Event::Dance => println!("Let's Dance with new event"),
                Event::Sing => println!("Let's Sing with new event")
            }
        }
    }

    let event = Event::Dance;
    let sing = Event::Sing;

    event.inspect();
    sing.inspect();

    enum Status {
        Rich,
        Poor
    }

    use Status::{Rich, Poor};

    let bil_gets = Rich;

    match bil_gets {
        Rich => println!("He is super Rich, He dosen't care anything about anyone anymore"),
        Poor => println!("He is super poor, He is looking for a way to get rich and he has to think about everyone")
    }

    enum CountryValueBaseOnTheirMentalities {
        Bangladesh = 3,
        China = 5,
        India = 4,
        America = 8,
        Japan = 10
    }

    type Country = CountryValueBaseOnTheirMentalities;

    let countrys = [Country::Bangladesh, Country::China, Country::India, Country::America, Country::Japan];

    for country in countrys.iter() {
        match country {
            Country::Bangladesh => println!("This Country peoples are so menupulated by some old ideas, score {}",Country::Bangladesh as i32),
            Country::China => println!("This Country peoples are so menupulated by some old ideas, score {}",Country::China as i32),
            Country::America => println!("This Country peoples are so funcking claver, score {}",Country::America as i32),
            Country::India => println!("This Country peoples are so dump as bangladesh, score {}",Country::India as i32),
            Country::Japan => println!("This Country peoples are so good, score {}",Country::Japan as i32),
        }
    }
    // Linked list
   

    enum List {
        // Tupe construct that wraps an element and a pointer to the next node
        Cons(u32, Box<List>),
        // Nil signifies the end of the linked list
        Nil
    }

    impl List {
        // Create an empty list
        fn new() -> List {
            List::Nil
        }

        fn append(self, elem: u32) -> List {
            List::Cons(elem, Box::new(self))
        }

        fn len(&self) -> u32 {
            match *self {
                List::Cons(_, ref tail ) => 1 + tail.len(),
                List::Nil => 0
            }
        }

        fn stringify(&self) -> String {
            match *self {
                List::Cons(head, ref tail) => {
                    format!("{} {}", tail.stringify(), head)
                },
                List::Nil => {
                    format!("Nil")
                }
            }
        }

    }

    let mut list = List::new();

    list = list.append(1);
    list = list.append(2);
    list = list.append(3);
    list = list.append(4);

    println!("Linked list has length: {}", list.len());
    println!("Linked list has elements: {}", list.stringify());
}

fn variable_binding() {
    let number = 23;
    let boolean = true;
    let tuple = ();
    let string_slice = "Hello rust";

    let cp_number = number;
    let cp_boolean = boolean;
    let cp_tuple = tuple;
    let cp_string_slice = string_slice;

    println!("copied Number: {}, copied Boolean: {}, copied Tuple: {:?}, copied string slice: {}", cp_number, cp_boolean, cp_tuple, cp_string_slice);

    println!("Number: {}, Boolean: {}, Tuple: {:?}, string slice: {}", number, boolean, tuple, string_slice);

    // variable mutability
    let _immutable_binding = 1;
    let mut mutable_binding = 1;

    println!("Before mutation: {}", mutable_binding);

    mutable_binding += 1;

    println!("After mutation: {}", mutable_binding);

    // _immutable_binding += 1; // error

    // scope and shadowing
    let long_lived_binding = 1;

    {
        let short_lived_binding = 2;
        println!("Inner short: {}", short_lived_binding);
        let long_lived_binding = 5_f32;
        println!("Inner long: {}", long_lived_binding);
    }
    println!("Outer long: {}", long_lived_binding);
    // try to access short_lived_binding
    // println!("Outer short: {}", short_lived_binding); // error

    let shadow_var = 100;

    {
        println!("Shadow var before shadowing: {}", shadow_var);
        let shadow_var = shadow_var + 50;
        println!("Shadow var after showing: {}", shadow_var);
    }

    println!("Shadow var after completed shadow scope: {}", shadow_var);

    // declare variable binding
    let binding;
    {
        binding = 200;
    }
    println!("Binding value: {}", binding);
}

fn types() {
    // casting
    // no implicit casting in rust
    // explicit casting can be done using `as`

    let pi = 3.1416_f64;

    // implicit casting
    // let pi_integer = pi; // error
    let pi_integer = pi as u32;

    println!("Integer: {}", pi_integer);
    println!("Float: {}", pi);

    println!("max u8: {}", u8::MAX);

    let lowest: u8 = 20;

    let lowest_char = lowest as char; // error
    println!("Lowest char: {}", lowest_char);

    println!("Prinsing all characters available");
    // for i  in 0..u8::MAX {
    //     println!("{}: {}", i, i as char);
    // }
    // for code_point in 0..u32::MAX {
    //     if let Some(c) = std::char::from_u32(code_point) {
    //         print!("{}", c);
    //     }
    // }
    println!("{}", u32::MAX/1000000);
    let utf_char = u32::MAX/1000000;

    for code in 0..utf_char {
        if let Some(c) = std::char::from_u32(code) {
            print!("{}", c);
        }
    }
}

fn conversion() {
    // from traits
    use std::convert::From;
    use std::convert::Into;

    #[derive(Debug)]
    struct Number {
        value: i32
    }

    impl From<i32> for Number {
        fn from(item: i32) -> Self {
            Number { value: item}
        }
    }

    impl Into<Number> for f64 {
        fn into(self) -> Number {
            Number {value: self as i32}
        }
    }

    let num = Number::from(40);
    let float_num: f64 = 34.0;
    let num2: Number = float_num.into();
    println!("Number: {}", num.value);
    println!("Float number: {:?}", num2);

    // tryfrom
    use std::convert::TryFrom;
    use std::convert::TryInto;

    #[derive(Debug)]
    struct SpecialNumber {
        value: i32,
        name: f64
    }

    impl TryFrom<i32> for SpecialNumber {
        type Error = ();
        fn try_from(value: i32) -> Result<Self, Self::Error> {
            if value < 10 {
                Ok(SpecialNumber {value, name: (value * 19) as f64})
            } else {
                Err(())
            }
        }
    }

    impl TryInto<SpecialNumber> for f64 {
        type Error = ();
        fn try_into(self) -> Result<SpecialNumber, Self::Error> {
            if self < 10.0 {
                Ok(SpecialNumber {value: self as i32, name: self})
            } else {
                Err(())
            }
        }
    }

    let special_num = SpecialNumber::try_from(9).unwrap();
    let float_special: SpecialNumber = 9.0_f64.try_into().unwrap();
    println!("Special number fron u32 {:?}", special_num);
    println!("Special number from f64{:?}", float_special);

}

fn flow_control() {
    let n = 5;

    if n < 5 {
        println!("n is less than 5");
    } else if n == 5 {
        println!("n is equal to 5");
    } else {
        println!("n is greater than 5");
    }

    let big_n = if n < 10 && n > 0 {
        10 * n
    } else {
        n / 2
    };
    println!("big_n: {}", big_n);

    // loops
    let mut count = 0;
    loop {
        count += 1;
        if count > 100 {
            break;
        }
        if count % 2 != 0 {
            continue;
        }
        print!("{}, ", count);
    }
    println!("");
    let mut count = 0;
    'outer: loop {
        // if count % 50 == 0 {
        //     println!("Breaking from outer loop");
        //     break 'outer;
        // }
        'inner: loop {
            count += 1;
            println!("Current count value: {}", count);
            if count % 60 == 0 {
                println!("Breaking fron inner loop to outer of number {}", count);
                break 'outer;
            }
            if count > 50000 {
                break 'inner;
            }
            count *= 2;
        }
    }

    // returning from loop
    let my_69 = loop {
        count += 1;
        if count == 69 || count > 100000{
            break count;
        }
    };
    
    println!("My fav number from loop: {}", my_69);

    // let mut num = 0;
    let num = 0;
    let start_timer = std::time::Instant::now();
    // while loop this loop will take 487 years to complete (approximately)
    // while num < i64::MAX {
    //     print!("{}, ", num);
    //     num += 1;
    // }
    println!("{}", i64::MAX);
    let end_timer = start_timer.elapsed();
    println!("Number of loop iterate: {}", num);
    println!("Time taken to iterate: {:?}", end_timer);

    for i in 0..10 {
        print!("{}, ", i);
    }
    println!("");
    for i in 0..=10 {
        print!("{}, ", i);
    }
    println!("");
    for i in (0..=100).step_by(7) {
        print!("{}, ", i);
    }
    println!("");

    let naruto_characters = vec!["Hinata", "Naruto", "Sasuke", "Sakura", "Kakashi", "Jiraya", "Orochimaru", "Itachi", "Madara", "Hashirama", "Tobirama", "Minato", "Kushina", "Nagato", "Konan", "Kisame", "Zabuza", "Haku", "Kabuto", "Orochimaru"];

    for character in naruto_characters.iter() {
        match character {
            &"Kushina" => println!("{} is a naruto's mother", character),
            &"Naruto" => println!("{} is the main character of the anime", character),
            _ => print!("{}, ", character)
        }
    }
    println!("");

    let reference = &200;
    match reference {
        &val => println!("Got a value: {}", val)
    }
    println!("Reference: {}", reference);
    match *reference {
        val => println!("Got a value with dereference: {}", val)
    }

    let try_value = 69;

    match try_value {
        val => println!("value without: {}", val)
    }
    println!("Try value: {}", try_value);

    struct Foo {
        x: (u32, u32),
        y: u32
    }

    let foo = Foo {x: (1, 2), y: 3};

    match foo {
        // Foo {x: (1, b), y} => println!("First of x is 1, b = {}, y = {}", b, y),
        // Foo {y: 2, x: i} => println!("y is 2, i = {:?}", i),
        Foo {y, ..} => println!("y = {}, x = {:?}", y, foo.x)
    }

    enum Temperature {
        Celsius(f64),
        Fahrenheit(f64)
    }

    let temp = Temperature::Celsius(34.0);

    match temp {
        Temperature::Celsius(c) if c < 30.0 => println!("Temperature is too cold"),
        Temperature::Celsius(c) if c > 35.0 => println!("Temperature is too hot"),
        Temperature::Celsius(c) => println!("Temperature is normal: {}", c),
        Temperature::Fahrenheit(f) => println!("Temperature in fahrenheit: {}", f)
    }

    fn some_number() -> Option<f32> {
        Some(43.0)
    }

    match some_number() {
        Some(n @ 45.0) => println!("Number is 45, n = {}", n),
        Some(n @ 43.0) => println!("Number is 43.0, n = {}", n),
        _ => println!("No number")
    }

    let number = Some(43);

    if let Some(i) = number {
        println!("Number: {}", i);
    }

    let lat = Some("Hello");
    let trying: Option<&str> = None;

    if let Some(i) = trying {
        println!("Lat: {}", i);
    } else {
        println!("No lat {}", lat.unwrap_or("No lat"));
    }

    enum Colors {
        Red,
        Green,
        Blue
    }

    let color = Colors::Green;

    if let Colors::Red = color {
        println!("Color is red");
    } else {
        println!("Color is not red");
    }

}   

pub fn run() {
    // hello_world();
    // println!("{}",format_greetings("Cyper"));
    // number_printing();
    // primitives_types()
    // custom_types();
    // variable_binding();
    // types();
    // conversion();
    flow_control();
}
