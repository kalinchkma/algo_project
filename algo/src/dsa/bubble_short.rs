
fn bubble_sort(mut v: Vec<i32>) -> Vec<i32> {

    for (i, n) in v.iter().enumerate() {
        if (i < v.len()) {
          println!("{}, {}", n, v[i+1])
        }
    }

    vec![]
}

pub fn run() {
    bubble_sort(vec![3, 5,13, 76]);
}