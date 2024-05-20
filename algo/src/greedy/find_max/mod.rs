/**
 * Find max digit of given number
 * Algorithms:(greedy)
 *  Append it to the number 
 *  remove it from the list of digits
 *  Repeat while there digits in the list
 */

use std::io;

#[allow(unused)]
pub fn run() {
    println!("Enter a sequence of numbers:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    println!("Possible max number of given digit: {}",find_max(&mut input));
 }

#[allow(unused)]
fn find_max(seq: &mut String) -> String {
    let mut big_num = String::new();

    while seq.len() > 0 {
        let mut max: usize = 0;
        let mut rmidx: usize = 0;
        for (idx, c) in seq.trim().char_indices() {
           let n = String::from(c); 
           let n: usize = n.trim().parse::<usize>().expect("Error parsing number");
           if n > max {
             max = n;
             rmidx = idx;
           }
        }
        big_num.push(seq.as_bytes()[rmidx] as char);
        seq.remove(rmidx);
    }
   
    big_num
 }



