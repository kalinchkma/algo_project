/**
 * Greatest common divisor
 * Concept:
 *  - Put fraction a/b in simplest form.
 *  - Divide numerator and denumerator by d, to get (a/d)/(b/d)
 *  - we need d to divide a and b
 *  - want  d to be as large as possible
 * 
 * Definition:
 *  For intergers, a and b , their greatest common divisor or gcd(a, b) is the largest interger d so that d divides both a and b
 */

 use std::io::{self, Write};

 pub fn run() {
    println!("Enter a two number:");
    print!("Numerator => ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let a = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(e) => {println!("{e}"); 0}
    };
    print!("Denumerator => ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let b = match input.trim().parse::<usize>() {
        Ok(n) => n,
        Err(e) => {println!("{e}"); 0}
    };
    println!("GCD of two numbers is: {}", euclid_gcd(a, b))
 }

 // Naive GCD Algorithms
 #[allow(unused)]
 fn gcd(a: usize, b: usize) -> usize {
    let mut best: usize = 0;
    for d in 1..a+b {
        if a % d == 0 && b % d == 0 {
            best = d;
        }
    }   
    return best;
 }

 /**
  * Euclidean Algorithm
  * GCD(a, b)
  * if b = 0:
  *     return a
  * ap <-- the remainder when a is 
  *     divided by b
  * return GCD(b, ap)
  * 
  */
  #[allow(unused)]
 fn euclid_gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    let ap = a % b;
    return  euclid_gcd(b, ap);
 }