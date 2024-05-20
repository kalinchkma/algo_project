use  std::thread;

pub fn thread_main() {
    let nums = vec![2, 3, 5, 6, 1, 7];

    let handle = thread::spawn(move || {
        println!("full number {:?}", nums);
       
    });
 
    handle.join().unwrap();
}

