
fn search(k: i32,n: i32,v: &mut Vec<i32>, set: &Vec<i32>, subsets: &mut Vec<Vec<i32>>) {
    if k == n {
        let mut s: Vec<i32> = Vec::new();
        // process subsets
        for i in v {
            s.push(*i);
        }
        subsets.push(s);
    } else {
        search(k+1, n, v, set, subsets);
        v.push(k);
        search(k+1, n, v, set, subsets);
        v.pop();
    }
}


fn main() {
    let sets = vec![0, 1, 2];
    let mut v: Vec<i32> = Vec::new();
    let mut subsets: Vec<Vec<i32>> = Vec::new();
    search(0, sets.len() as i32, &mut v, &sets, &mut subsets);
    for sub in subsets {
        println!("{:?}", sub);
    }

    println!("{}", 1<<3);
}