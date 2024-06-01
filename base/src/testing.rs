#![allow(dead_code)]

pub fn run() {
    println!("Testing...");
}


struct Reactangle {
    width: u32,
    height: u32,
}

impl Reactangle {
    fn can_hold(&self, onther: &Reactangle) -> bool {
        self.width > onther.width && self.height > onther.height
    }
}

#[cfg(test)]
mod mod_tests {
    use super::*;

   #[test]
   fn larger_can_hold_smaller() {
        let larger = Reactangle { width: 8, height: 7 };
        let smaller = Reactangle { width: 5, height: 1 };
        assert_eq!(larger.can_hold(&smaller), true);
   }
}