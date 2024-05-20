use std::collections::HashMap;


#[allow(unused)]
fn fib(n: usize) -> usize {
    if n <= 1 {
        n
    } else {
        fib(n-1) + fib(n-2)
    }
}


fn fib_mem(n: u128, m: &mut HashMap<u128, u128>) -> u128 {
    fn fib_cal(n: u128, m: &mut HashMap<u128, u128>) -> u128 {
       let m_n = match m.get(&n) {
            Some(&num) => num,
            _ => 10
        };
        if m_n != 10 {
            return m_n;
        }
        let c = fib_cal(n-1, m) + fib_cal(n - 2, m);
        m.insert(n, c);
        return c;
    }
    return fib_cal(n, m);
}

pub fn fib_main() {
    let mut m_table: HashMap<u128, u128> = HashMap::new();
    m_table.insert(0, 0);
    m_table.insert(1, 1);
    for i in 0..200 as usize {
        println!("Fib number {}: {}",i, fib_mem(i as u128, &mut m_table));
    }
    
}