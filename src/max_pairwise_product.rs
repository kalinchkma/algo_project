/**
 * Problem: Find the maximum fairwise product of given sequence  integers 0 <= a0 .... an <= 10^5
 * Input: The first line of input contains number 2 <= n <= 2.10^5. the next line contain n non-negative integer  0 < <= a0 ... an <= 10^5
 * output: single number the maximum pairwise product
 */
use std::io;
use rand::Rng;


 // solve with Big0 of n^2
#[allow(dead_code)]
pub fn max_pair_product() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Error reading line");
    let n: i32 = match input.trim().parse::<i32>() {
        Ok(n) => n,
        Err(e) => {println!("Error: {}", e);0}
    };

    let mut ar: Vec<usize> = Vec::new();
    for _ in 0..n {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error reading line");
        let num: usize = match input.trim().parse::<usize>() {
            Ok(n) => n,
            Err(e) => {println!("Error: {}", e); 0}
        };
        ar.push(num);
    }

    
    // println!("Maxmum product of tow number of list is: {}", big_on(&mut ar));
    // println!("Maxmum product of tow number of list is: {}", big_o_n2(&mut ar));
}

pub fn stress_test() {
    let mut rng: rand::ThreadRng = rand::thread_rng();
    loop {
        let n: usize = rng.gen_range(0, 100000) % 1000 + 2;
        println!("{n}");
        let mut ar: Vec<usize> = Vec::new();
        for _ in 0..n {
            ar.push(rng.gen_range(0, 100000));
        }
        for i in ar.clone() {
            print!("{i} ");
        }
        print!("\n");
        let (ar, res1) = big_o_n2(&mut ar);
        let res2 = big_on(ar);
        if res1 != res2 {
            println!("Wrong answer: {}, {}", res1, res2);
            break;
        } else {
            println!("OK!");
        }
    }
}


/**
 * This on also not correct
 */
fn big_on(ar: &mut Vec<usize>) -> usize {
    let mut max: usize = usize::MIN;
    let mut max_2: usize = usize::MIN;
    for i in 0..ar.len() {
        if ar[i] > ar[max] {
            max = i;
        }
    }
    if max == 0 {
        max_2 = 1;
    }
    for i in 0..ar.len() {
        if i != max && ar[i] > ar[max_2] {
                max_2 = i;
        }
    }
    let res =  ar[max] * ar[max_2];
    return res;
}
/*
This solution is not correct for time complexity
*/
fn big_o_n2(ar: &mut Vec<usize>) -> ( &mut Vec<usize>, usize) {
    let length = ar.len();
    let mut result: usize = 0;
    for i in 0..length  {
        for j in i+1..length {
            let cal = ar[i] * ar[j];
            
            if result < cal {
                result = cal;
            }
        }
    }
    return (ar, result)
}