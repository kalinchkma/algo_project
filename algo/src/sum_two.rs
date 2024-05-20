/**
 * Sum of two numbers
 */

use std::io;

pub fn sum_two_number() {
    println!("Enter a number two number:");
    let mut a = String::new();
    io::stdin().read_line(&mut a).expect("Failed to read line");
    let mut b = String::new();
    io::stdin().read_line(&mut b).expect("Failed to read line");

    // parse number from string method one
    let a1: i32 = match a.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {
            println!("Error: {}", e); 
            0}
    };
    // parse number from string method two
    let b1: i32 = b.trim().parse().expect("Not a valid number");

    println!("Sum of number is: {}", a1 + b1);
}