


fn main() {
    let numbers: Vec<i32> = vec![3, 6, 9];
    let n = numbers.len() as usize;
    let mut permutation: Vec<i32> = Vec::new();
    let mut chosen: Vec<bool> = vec![false; n];
    
    search(&mut permutation, n, &mut chosen, &numbers);
}

fn search(permutation: &mut Vec<i32>, n: usize, chosen: &mut Vec<bool>, sets: &Vec<i32>) {
        if permutation.len() == n {
            // process permutation
            println!("Permutation");
            for p in permutation {
                print!("{},", sets[*p as usize]);
            }
            println!("");
        } else {
            for i in 0..n {
                if chosen[i as usize] { continue};
                chosen[i as usize] = true;
                permutation.push(i as i32);
                search(permutation, n, chosen, sets);
                chosen[i as usize] = false;
                permutation.pop();
            }
        }
    }