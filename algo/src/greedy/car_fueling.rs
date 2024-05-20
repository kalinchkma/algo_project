/**
 * Car fueling:
 * ------------------------
 * Input: 
 * A car which can travel at most L kilometers with full tank, a source point A,
 * a destination point B and n gas stations at distances 
 * x1 <= x2 <= x3 <= ... <= xn in kilometers from A along the path from A to B
 * 
 * Output:
 * The minimum number of refills to get from A to B, besides refill at A    
 */
use std::io::{self, Write};

pub fn run() {
    print!("Enter how far car can go with full fuel of tank: ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading input");
    let mileage: usize = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(_) => panic!("Error parsing number")
    };
    println!("Enter a gas stations to go point A to B: ");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let mut c: String = String::new();
    let mut stations: Vec<usize> = Vec::new();
    let mut input = input.trim().to_string();
    input.push('e');
    for s in input.trim().chars().into_iter() {
        if s == ' ' || s == 'e' {
            let sd = c.trim().parse::<usize>().expect("Error parsing number");
            stations.push(sd);
            c = String::new();
        }
        c.push(s);
    }
    car_fueling(stations, mileage);
   
}

#[allow(unused)]
fn car_fueling(v: Vec<usize>, m: usize) {
    let mut count: u8 = u8::MIN;
    
    for i in 0..v.len()-1 {
        if v[i] <= m && v[i+1] > m {
            count += 1;
        }
    }
    println!("Final fueling: {}", count);
}