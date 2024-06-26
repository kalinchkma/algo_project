
fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {

    for i in 0..v.len() {
        let mut swap = false;
        for j in 0..v.len()-i-1 {
            if v[j] > v[j+1] {
                let temp = v[j];
                v[j] = v[j+1];
                v[j+1] = temp;
                swap = true
            }
        }
        if !swap {
            break;
        }
    }
    println!("{:?}", v);
    v
}

pub fn run() {
    bubble_sort(vec![35, 5,1, 77, 9, 76]);
}