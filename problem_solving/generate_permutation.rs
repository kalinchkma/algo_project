


fn main() {
    let numbers: Vec<i32> = vec![1, 2, 3];
    let n = numbers.len() as usize;
    let mut permutation: Vec<i32> = Vec::new();
    let mut chosen: Vec<bool> = vec![false; n];
    
    search(&mut permutation, n, &mut chosen);
}

fn search(permutation: &mut Vec<i32>, n: usize, chosen: &mut Vec<bool>) {
        if permutation.len() == n {
            // process permutation
            println!("Permutation");
            for p in permutation {
                print!("{},", p);
            }
            println!("");
        } else {
            for i in 0..n {
                if chosen[i as usize] { continue};
                chosen[i as usize] = true;
                permutation.push(i as i32);
                search(permutation, n, chosen);
                chosen[i as usize] = false;
                permutation.pop();
            }
        }
    }